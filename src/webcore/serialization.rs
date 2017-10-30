use std::mem;
use std::slice;
use std::i32;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::cell::{Cell, UnsafeCell};

use webcore::ffi;
use webcore::callfn::CallMut;
use webcore::newtype::Newtype;
use webcore::try_from::{TryFrom, TryInto};
use webcore::number::Number;

use webcore::value::{
    Null,
    Undefined,
    Reference,
    Value
};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tag {
    Undefined = 0,
    Null = 1,
    I32 = 2,
    F64 = 3,
    Str = 4,
    False = 5,
    True = 6,
    Array = 7,
    Object = 8,
    Reference = 9,
    Function = 10
}

impl Default for Tag {
    #[inline]
    fn default() -> Self {
        Tag::Undefined
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct PreallocatedArena {
    saved: UnsafeCell< Vec< Value > >,
    buffer: UnsafeCell< Vec< u8 > >,
    index: Cell< usize >
}

impl PreallocatedArena {
    #[doc(hidden)]
    #[inline]
    pub fn new( memory_required: usize ) -> Self {
        let mut buffer = Vec::new();
        buffer.reserve( memory_required );
        unsafe {
            buffer.set_len( memory_required );
        }

        PreallocatedArena {
            saved: UnsafeCell::new( Vec::new() ),
            buffer: UnsafeCell::new( buffer ),
            index: Cell::new( 0 )
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn reserve< T >( &self, count: usize ) -> &mut [T] {
        let bytes = mem::size_of::< T >() * count;
        let slice = unsafe {
            let buffer = &mut *self.buffer.get();
            debug_assert!( self.index.get() + bytes <= buffer.len() );

            slice::from_raw_parts_mut(
                buffer.as_mut_ptr().offset( self.index.get() as isize ) as *mut T,
                count
            )
        };

        self.index.set( self.index.get() + bytes );
        slice
    }

    #[doc(hidden)]
    #[inline]
    pub fn save( &self, value: Value ) -> &Value {
        unsafe {
            let saved = &mut *self.saved.get();
            saved.push( value );
            &*(saved.last().unwrap() as *const Value)
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn assert_no_free_space_left( &self ) {
        debug_assert!( self.index.get() == unsafe { &*self.buffer.get() }.len() );
    }
}

#[doc(hidden)]
pub trait JsSerializableOwned: Sized {
    fn into_js_owned< 'a >( value: &'a mut Option< Self >, arena: &'a PreallocatedArena ) -> SerializedValue< 'a >;
    fn memory_required_owned( &self ) -> usize;
}

#[doc(hidden)]
pub trait JsSerializable {
    fn into_js< 'a >( &'a self, arena: &'a PreallocatedArena ) -> SerializedValue< 'a >;
    fn memory_required( &self ) -> usize;
}

// This is a generic structure for serializing every JavaScript value.
#[doc(hidden)]
#[repr(C)]
#[derive(Default, Debug)]
pub struct SerializedValue< 'a > {
    data_1: u64,
    data_2: u32,
    tag: Tag,
    phantom: PhantomData< &'a () >
}

#[test]
fn test_serialized_value_size() {
    assert_eq!( mem::size_of::< SerializedValue< 'static > >(), 16 );
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedUndefined;

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedNull;

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedI32 {
    value: i32
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedF64 {
    value: f64
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedTrue {}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedFalse {}

#[repr(C)]
#[derive(Clone, Debug)]
struct SerializedUntaggedString {
    pointer: u32,
    length: u32
}

#[repr(C)]
#[derive(Clone, Debug)]
struct SerializedUntaggedArray {
    pointer: u32,
    length: u32
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedObject {
    value_pointer: u32,
    length: u32,
    key_pointer: u32
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedReference {
    refid: i32
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedFunction {
    adapter_pointer: u32,
    pointer: u32,
    deallocator_pointer: u32
}

impl SerializedUntaggedString {
    #[inline]
    fn deserialize( &self ) -> String {
        let pointer = self.pointer as *mut u8;
        let length = self.length as usize;

        unsafe {
            let vector = Vec::from_raw_parts( pointer, length, length + 1 );
            String::from_utf8_unchecked( vector )
        }
    }
}

impl SerializedUntaggedArray {
    #[inline]
    fn deserialize( &self ) -> Vec< Value > {
        let pointer = self.pointer as *const SerializedValue;
        let length = self.length as usize;
        let slice = unsafe {
            slice::from_raw_parts( pointer, length )
        };

        let vector = slice.iter().map( |value| value.deserialize() ).collect();
        unsafe {
            ffi::free( pointer as *const u8 );
        }

        vector
    }
}

impl SerializedUntaggedObject {
    #[inline]
    fn deserialize( &self ) -> BTreeMap< String, Value > {
        let length = self.length as usize;
        let key_pointer = self.key_pointer as *const SerializedUntaggedString;
        let value_pointer = self.value_pointer as *const SerializedValue;

        let key_slice = unsafe { slice::from_raw_parts( key_pointer, length ) };
        let value_slice = unsafe { slice::from_raw_parts( value_pointer, length ) };

        let map = key_slice.iter().zip( value_slice.iter() ).map( |(key, value)| {
            let key = key.deserialize();
            let value = value.deserialize();
            (key, value)
        }).collect();

        unsafe {
            ffi::free( key_pointer as *const u8 );
            ffi::free( value_pointer as *const u8 );
        }

        map
    }
}

impl SerializedUntaggedReference {
    #[inline]
    fn deserialize( &self ) -> Reference {
        unsafe { Reference::from_raw_unchecked( self.refid ) }
    }
}

macro_rules! untagged_boilerplate {
    ($tests_namespace:ident, $reader_name:ident, $tag:expr, $untagged_type:ident) => {
        impl< 'a > SerializedValue< 'a > {
            #[allow(dead_code)]
            #[inline]
            fn $reader_name( &self ) -> &$untagged_type {
                debug_assert_eq!( self.tag, $tag );
                unsafe {
                    &*(self as *const _ as *const $untagged_type)
                }
            }
        }

        impl< 'a > From< $untagged_type > for SerializedValue< 'a > {
            #[inline]
            fn from( untagged: $untagged_type ) -> Self {
                unsafe {
                    let mut value: SerializedValue = mem::uninitialized();
                    *(&mut value as *mut SerializedValue as *mut $untagged_type) = untagged;
                    value.tag = $tag;
                    value
                }
            }
        }

        #[cfg(test)]
        mod $tests_namespace {
            use super::*;

            #[test]
            fn does_not_overlap_with_the_tag() {
                let size = mem::size_of::< $untagged_type >();
                let tag_offset = unsafe { &(&*(0 as *const SerializedValue< 'static >)).tag as *const _ as usize };
                assert!( size <= tag_offset );
            }
        }
    }
}

untagged_boilerplate!( test_undefined, as_undefined, Tag::Undefined, SerializedUntaggedUndefined );
untagged_boilerplate!( test_null, as_null, Tag::Null, SerializedUntaggedNull );
untagged_boilerplate!( test_i32, as_i32, Tag::I32, SerializedUntaggedI32 );
untagged_boilerplate!( test_f64, as_f64, Tag::F64, SerializedUntaggedF64 );
untagged_boilerplate!( test_true, as_true, Tag::True, SerializedUntaggedTrue );
untagged_boilerplate!( test_false, as_false, Tag::False, SerializedUntaggedFalse );
untagged_boilerplate!( test_object, as_object, Tag::Object, SerializedUntaggedObject );
untagged_boilerplate!( test_string, as_string, Tag::Str, SerializedUntaggedString );
untagged_boilerplate!( test_array, as_array, Tag::Array, SerializedUntaggedArray );
untagged_boilerplate!( test_reference, as_reference, Tag::Reference, SerializedUntaggedReference );
untagged_boilerplate!( test_function, as_function, Tag::Function, SerializedUntaggedFunction );

impl< 'a > SerializedValue< 'a > {
    #[doc(hidden)]
    #[inline]
    pub fn deserialize( &self ) -> Value {
        match self.tag {
            Tag::Undefined => Value::Undefined,
            Tag::Null => Value::Null,
            Tag::I32 => self.as_i32().value.into(),
            Tag::F64 => self.as_f64().value.into(),
            Tag::Str => Value::String( self.as_string().deserialize() ),
            Tag::False => Value::Bool( false ),
            Tag::True => Value::Bool( true ),
            Tag::Array => Value::Array( self.as_array().deserialize() ),
            Tag::Object => Value::Object( self.as_object().deserialize() ),
            Tag::Reference => self.as_reference().deserialize().into(),
            Tag::Function => unreachable!()
        }
    }
}

impl JsSerializable for () {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedUndefined.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( () );

impl JsSerializable for Undefined {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedUndefined.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( Undefined );

impl JsSerializable for Null {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedNull.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( Null );

impl JsSerializable for Reference {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedReference {
            refid: self.as_raw()
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( Reference );

impl JsSerializable for bool {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        if *self {
            SerializedUntaggedTrue {}.into()
        } else {
            SerializedUntaggedFalse {}.into()
        }
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( bool );

impl JsSerializable for str {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedString {
            pointer: self.as_ptr() as u32,
            length: self.len() as u32
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( impl< 'a > for &'a str );

impl JsSerializable for String {
    #[inline]
    fn into_js< 'a >( &'a self, arena: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        self.as_str().into_js( arena )
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        self.as_str().memory_required()
    }
}

__js_serializable_boilerplate!( String );

impl JsSerializable for i8 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        (*self as i32).memory_required()
    }
}

__js_serializable_boilerplate!( i8 );

impl JsSerializable for i16 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        (*self as i32).memory_required()
    }
}

__js_serializable_boilerplate!( i16 );

impl JsSerializable for i32 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( i32 );

impl JsSerializable for u8 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        (*self as i32).memory_required()
    }
}

__js_serializable_boilerplate!( u8 );

impl JsSerializable for u16 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        (*self as i32).memory_required()
    }
}

__js_serializable_boilerplate!( u16 );

impl JsSerializable for u32 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedF64 {
            value: *self as f64
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        (*self as f64).memory_required()
    }
}

__js_serializable_boilerplate!( u32 );

impl JsSerializable for f32 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedF64 {
            value: *self as f64
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        (*self as f64).memory_required()
    }
}

__js_serializable_boilerplate!( f32 );

impl JsSerializable for f64 {
    #[inline]
    fn into_js< 'a >( &'a self, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        SerializedUntaggedF64 {
            value: *self
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( f64 );

impl JsSerializable for Number {
    #[inline]
    fn into_js< 'a >( &'a self, arena: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        use webcore::number::{Storage, get_storage};
        match *get_storage( self ) {
            Storage::I32( ref value ) => value.into_js( arena ),
            Storage::F64( ref value ) => value.into_js( arena )
        }
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        0
    }
}

__js_serializable_boilerplate!( Number );

impl< T: JsSerializable > JsSerializable for Option< T > {
    #[inline]
    fn into_js< 'a >( &'a self, arena: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        if let Some( value ) = self.as_ref() {
            value.into_js( arena )
        } else {
            SerializedUntaggedNull.into()
        }
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        if let Some( value ) = self.as_ref() {
            value.memory_required()
        } else {
            0
        }
    }
}

__js_serializable_boilerplate!( impl< T > for Option< T > where T: JsSerializable );

impl< T: JsSerializable > JsSerializable for [T] {
    #[inline]
    fn into_js< 'a >( &'a self, arena: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        let output = arena.reserve( self.len() );
        for (value, output_value) in self.iter().zip( output.iter_mut() ) {
            *output_value = value.into_js( arena );
        }

        SerializedUntaggedArray {
            pointer: output.as_ptr() as u32,
            length: output.len() as u32
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        mem::size_of::< SerializedValue >() * self.len() +
        self.iter().fold( 0, |sum, value| sum + value.memory_required() )
    }
}

__js_serializable_boilerplate!( impl< 'a, T > for &'a [T] where T: JsSerializable );

impl< T: JsSerializable > JsSerializable for BTreeMap< String, T > {
    #[inline]
    fn into_js< 'a >( &'a self, arena: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        let keys = arena.reserve( self.len() );
        let values = arena.reserve( self.len() );
        for (((key, value), output_key), output_value) in self.iter().zip( keys.iter_mut() ).zip( values.iter_mut() ) {
            *output_key = key.into_js( arena ).as_string().clone();
            *output_value = value.into_js( arena );
        }

        SerializedUntaggedObject {
            key_pointer: keys.as_ptr() as u32,
            value_pointer: values.as_ptr() as u32,
            length: keys.len() as u32
        }.into()
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        mem::size_of::< SerializedValue >() * self.len() +
        mem::size_of::< SerializedUntaggedString >() * self.len() +
        self.iter().fold( 0, |sum, (key, value)| sum + key.memory_required() + value.memory_required() )
    }
}

__js_serializable_boilerplate!( impl< T > for BTreeMap< String, T > where T: JsSerializable );

impl JsSerializable for Value {
    fn into_js< 'a >( &'a self, arena: &'a PreallocatedArena ) -> SerializedValue< 'a > {
        match *self {
            Value::Undefined => SerializedUntaggedUndefined.into(),
            Value::Null => SerializedUntaggedNull.into(),
            Value::Bool( ref value ) => value.into_js( arena ),
            Value::Number( ref value ) => value.into_js( arena ),
            Value::String( ref value ) => value.into_js( arena ),
            Value::Array( ref value ) => value.as_slice().into_js( arena ),
            Value::Object( ref value ) => value.into_js( arena ),
            Value::Reference( ref value ) => value.into_js( arena )
        }
    }

    fn memory_required( &self ) -> usize {
        match *self {
            Value::Undefined => Undefined.memory_required(),
            Value::Null => Null.memory_required(),
            Value::Bool( value ) => value.memory_required(),
            Value::Number( value ) => value.memory_required(),
            Value::String( ref value ) => value.memory_required(),
            Value::Array( ref value ) => value.as_slice().memory_required(),
            Value::Object( ref value ) => value.memory_required(),
            Value::Reference( ref value ) => value.memory_required()
        }
    }
}

__js_serializable_boilerplate!( Value );

#[derive(Debug)]
pub struct FunctionTag;

#[derive(Debug)]
pub struct NonFunctionTag;

impl< T: JsSerializable > JsSerializableOwned for Newtype< (NonFunctionTag, ()), T > {
    #[inline]
    fn into_js_owned< 'x >( value: &'x mut Option< Self >, arena: &'x PreallocatedArena ) -> SerializedValue< 'x > {
        JsSerializable::into_js( value.as_ref().unwrap().as_ref(), arena )
    }

    #[inline]
    fn memory_required_owned( &self ) -> usize {
        JsSerializable::memory_required( &**self )
    }
}

trait FuncallAdapter< F > {
    extern fn funcall_adapter( callback: *mut F, raw_arguments: *mut SerializedUntaggedArray );
    extern fn deallocator( callback: *mut F );
}

macro_rules! impl_for_fn {
    ($next:tt => $($kind:ident),*) => {
        impl< $($kind: TryFrom< Value >,)* F > FuncallAdapter< F > for Newtype< (FunctionTag, ($($kind,)*)), F >
            where F: CallMut< ($($kind,)*) > + 'static, F::Output: JsSerializableOwned
        {
            #[allow(unused_mut, unused_variables, non_snake_case)]
            extern fn funcall_adapter(
                    callback: *mut F,
                    raw_arguments: *mut SerializedUntaggedArray
                )
            {
                let callback = unsafe { &mut *callback };
                let mut arguments = unsafe { &*raw_arguments }.deserialize();

                unsafe {
                    ffi::free( raw_arguments as *const u8 );
                }

                if arguments.len() != F::expected_argument_count() {
                    // TODO: Should probably throw an exception into the JS world or something like that.
                    panic!( "Expected {} arguments, got {}", F::expected_argument_count(), arguments.len() );
                }

                let mut arguments = arguments.drain( .. );
                let mut nth_argument = 0;
                $(
                    let $kind = match arguments.next().unwrap().try_into() {
                        Ok( value ) => value,
                        Err( _ ) => {
                            panic!( "Argument #{} is not convertible", nth_argument + 1 );
                        }
                    };

                    nth_argument += 1;
                )*

                $crate::private::noop( &mut nth_argument );

                let result = callback.call_mut( ($($kind,)*) );
                let mut arena = PreallocatedArena::new( result.memory_required_owned() );

                let mut result = Some( result );
                let result = JsSerializableOwned::into_js_owned( &mut result, &mut arena );
                let result = &result as *const _;

                // This is kinda hacky but I'm not sure how else to do it at the moment.
                em_asm_int!( "Module.STDWEB.tmp = Module.STDWEB.to_js( $0 );", result );
            }

            extern fn deallocator( callback: *mut F ) {
                let callback = unsafe {
                    Box::from_raw( callback )
                };

                drop( callback );
            }
        }

        impl< $($kind: TryFrom< Value >,)* F > JsSerializableOwned for Newtype< (FunctionTag, ($($kind,)*)), F >
            where F: CallMut< ($($kind,)*) > + 'static, F::Output: JsSerializableOwned
        {
            #[inline]
            fn into_js_owned< 'a >( value: &'a mut Option< Self >, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
                let callback: *mut F = Box::into_raw( Box::new( value.take().unwrap().unwrap_newtype() ) );

                SerializedUntaggedFunction {
                    adapter_pointer: <Self as FuncallAdapter< F > >::funcall_adapter as u32,
                    pointer: callback as u32,
                    deallocator_pointer: <Self as FuncallAdapter< F > >::deallocator as u32
                }.into()
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                0
            }
        }

        impl< $($kind: TryFrom< Value >,)* F > JsSerializableOwned for Newtype< (FunctionTag, ($($kind,)*)), Option< F > >
            where F: CallMut< ($($kind,)*) > + 'static, F::Output: JsSerializableOwned
        {
            #[inline]
            fn into_js_owned< 'a >( value: &'a mut Option< Self >, _: &'a PreallocatedArena ) -> SerializedValue< 'a > {
                if let Some( value ) = value.take().unwrap().unwrap_newtype() {
                    let callback: *mut F = Box::into_raw( Box::new( value ) );
                    SerializedUntaggedFunction {
                        adapter_pointer: <Newtype< (FunctionTag, ($($kind,)*)), F > as FuncallAdapter< F > >::funcall_adapter as u32,
                        pointer: callback as u32,
                        deallocator_pointer: <Newtype< (FunctionTag, ($($kind,)*)), F > as FuncallAdapter< F > >::deallocator as u32
                    }.into()
                } else {
                    SerializedUntaggedNull.into()
                }
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                0
            }
        }

        next! { $next }
    }
}

loop_through_identifiers!( impl_for_fn );

impl< 'a, T: ?Sized + JsSerializable > JsSerializable for &'a T {
    #[inline]
    fn into_js< 'x >( &'x self, arena: &'x PreallocatedArena ) -> SerializedValue< 'x > {
        T::into_js( *self, arena )
    }

    #[inline]
    fn memory_required( &self ) -> usize {
        T::memory_required( *self )
    }
}

#[cfg(test)]
mod test_deserialization {
    use std::rc::Rc;
    use std::cell::Cell;
    use super::*;

    #[test]
    fn i32() {
        assert_eq!( js! { return 100; }, Value::Number( 100_i32.into() ) );
    }

    #[test]
    fn f64() {
        assert_eq!( js! { return 100.5; }, Value::Number( 100.5_f64.into() ) );
    }

    #[test]
    fn bool_true() {
        assert_eq!( js! { return true; }, Value::Bool( true ) );
    }

    #[test]
    fn bool_false() {
        assert_eq!( js! { return false; }, Value::Bool( false ) );
    }

    #[test]
    fn undefined() {
        assert_eq!( js! { return undefined; }, Value::Undefined );
    }

    #[test]
    fn null() {
        assert_eq!( js! { return null; }, Value::Null );
    }

    #[test]
    fn string() {
        assert_eq!( js! { return "Dog"; }, Value::String( "Dog".to_string() ) );
    }

    #[test]
    fn array() {
        assert_eq!( js! { return [1, 2]; }, Value::Array( vec![ Value::Number( 1.into() ), Value::Number( 2.into() ) ] ) );
    }

    #[test]
    fn object() {
        assert_eq!( js! { return {"one": 1, "two": 2}; }, Value::Object( [("one".to_string(), Value::Number(1.into())), ("two".to_string(), Value::Number(2.into()))].iter().cloned().collect() ) );
    }

    #[test]
    fn reference() {
        assert_eq!( js! { return new Date(); }.is_reference(), true );
    }

    #[test]
    fn arguments() {
        let value = js! {
            return (function() {
                return arguments;
            })( 1, 2 );
        };

        assert_eq!( value, Value::Array( vec![ Value::Number( 1.into() ), Value::Number( 2.into() ) ] ) );
    }

    #[test]
    fn function() {
        let value = Rc::new( Cell::new( 0 ) );
        let fn_value = value.clone();

        js! {
            var callback = @{move || { fn_value.set( 1 ); }};
            callback();
            callback.drop();
        };

        assert_eq!( value.get(), 1 );
    }

    #[test]
    fn function_returning_bool() {
        let result = js! {
            var callback = @{move || { return true }};
            var result = callback();
            callback.drop();

            return result;
        };

        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn function_with_single_bool_argument() {
        let value = Rc::new( Cell::new( false ) );
        let fn_value = value.clone();

        js! {
            var callback = @{move |value: bool| { fn_value.set( value ); }};
            callback( true );
            callback.drop();
        };

        assert_eq!( value.get(), true );
    }

    #[test]
    fn function_inside_an_option() {
        let value = Rc::new( Cell::new( 0 ) );
        let fn_value = value.clone();

        js! {
            var callback = @{Some( move || { fn_value.set( 1 ); } )};
            callback();
            callback.drop();
        };

        assert_eq!( value.get(), 1 );
    }

    #[test]
    #[allow(unused_assignments)]
    fn function_inside_an_empty_option() {
        let mut callback = Some( move || () );
        callback = None;

        let result = js! {
            var callback = @{callback};
            return callback === null;
        };

        assert_eq!( result, Value::Bool( true ) );
    }
}

#[cfg(test)]
mod test_serialization {
    use super::*;

    #[test]
    fn object() {
        let object: BTreeMap< _, _ > = [
            ("number".to_string(), Value::Number( 123.into() )),
            ("string".to_string(), Value::String( "Hello!".into() ))
        ].iter().cloned().collect();

        let result = js! {
            var object = @{object};
            return object.number === 123 && object.string === "Hello!" && Object.keys( object ).length == 2;
        };
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn multiple() {
        let reference: Reference = js! {
            return new Date();
        }.try_into().unwrap();

        let result = js! {
            var callback = @{|| {}};
            var reference = @{&reference};
            var string = @{"Hello!"};
            return Object.prototype.toString.call( callback ) == "[object Function]" &&
                Object.prototype.toString.call( reference ) == "[object Date]" &&
                Object.prototype.toString.call( string ) == "[object String]"
        };
        assert_eq!( result, Value::Bool( true ) );
    }
}

#[cfg(test)]
mod test_reserialization {
    use super::*;

    #[test]
    fn i32() {
        assert_eq!( js! { return @{100}; }, Value::Number( 100_i32.into() ) );
    }

    #[test]
    fn f64() {
        assert_eq!( js! { return @{100.5}; }, Value::Number( 100.5_f64.into() ) );
    }

    #[test]
    fn bool_true() {
        assert_eq!( js! { return @{true}; }, Value::Bool( true ) );
    }

    #[test]
    fn bool_false() {
        assert_eq!( js! { return @{false}; }, Value::Bool( false ) );
    }

    #[test]
    fn undefined() {
        assert_eq!( js! { return @{Undefined}; }, Value::Undefined );
    }

    #[test]
    fn null() {
        assert_eq!( js! { return @{Null}; }, Value::Null );
    }

    #[test]
    fn string() {
        assert_eq!( js! { return @{"Dog"}; }, Value::String( "Dog".to_string() ) );
    }

    #[test]
    fn array() {
        let array = vec![ Value::Number( 1.into() ), Value::Number( 2.into() ) ];
        assert_eq!( js! { return @{&[1, 2][..]}; }, Value::Array( array ) );
    }

    #[test]
    fn object() {
        let object = [("one".to_string(), Value::Number(1.into())), ("two".to_string(), Value::Number(2.into()))].iter().cloned().collect();
        assert_eq!( js! { return @{&object}; }, Value::Object( object ) );
    }

    #[test]
    fn reference() {
        let date = js! { return new Date(); };
        assert_eq!( js! { return Object.prototype.toString.call( @{date} ) }, "[object Date]" );
    }

    #[test]
    fn reference_by_ref() {
        let date = js! { return new Date(); };
        assert_eq!( js! { return Object.prototype.toString.call( @{&date} ) }, "[object Date]" );
    }

    #[test]
    fn option_some() {
        assert_eq!( js! { return @{Some( true )}; }, Value::Bool( true ) );
    }

    #[test]
    fn option_none() {
        let boolean_none: Option< bool > = None;
        assert_eq!( js! { return @{boolean_none}; }, Value::Null );
    }

    #[test]
    fn value() {
        assert_eq!( js! { return @{Value::String( "Dog".to_string() )}; }, Value::String( "Dog".to_string() ) );
    }

    #[test]
    fn closure_context() {
        let constant: u32 = 0x12345678;
        let callback = move || {
            let value: Value = constant.into();
            value
        };

        let value = js! {
            return @{callback}();
        };

        assert_eq!( value, Value::Number( 0x12345678_i32.into() ) );
    }
}

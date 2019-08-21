use std::mem;
use std::slice;
use std::i32;
use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use std::hash::Hash;
use std::ops::Deref;

use webcore::ffi;
use webcore::callfn::{CallOnce, CallMut};
use webcore::newtype::Newtype;
use webcore::try_from::{TryFrom, TryInto};
use webcore::number::Number;
use webcore::type_name::type_name;
use webcore::symbol::Symbol;
use webcore::unsafe_typed_array::UnsafeTypedArray;
use webcore::mutfn::Mut;
use webcore::once::Once;
use webcore::global_arena;
use webcore::optional_arg::OptionalArg;

use webcore::value::{
    Null,
    Undefined,
    Reference,
    Value,
    ConversionError
};

use webapi::error::TypeError;

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
    Function = 10,
    FunctionMut = 12,
    FunctionOnce = 13,
    UnsafeTypedArray = 14,
    Symbol = 15
}

impl Default for Tag {
    #[inline]
    fn default() -> Self {
        Tag::Undefined
    }
}

#[doc(hidden)]
pub trait JsSerializeOwned: Sized {
    fn into_js_owned< 'a >( value: &'a mut Option< Self > ) -> SerializedValue< 'a >;
}

/// A trait for types which can be serialized through the `js!` macro.
///
/// Do **not** try to implement this trait yourself! It's only meant
/// to be used inside generic code for specifying trait bounds.
pub trait JsSerialize {
    #[doc(hidden)]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a >;
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
struct SerializedUntaggedSymbol {
    id: i32
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

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedFunctionMut {
    adapter_pointer: u32,
    pointer: u32,
    deallocator_pointer: u32
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedFunctionOnce {
    adapter_pointer: u32,
    pointer: u32,
    deallocator_pointer: u32
}

#[repr(C)]
#[derive(Debug)]
struct SerializedUntaggedUnsafeTypedArray {
    pointer: u32,
    length: u32,
    kind: u32
}

impl SerializedUntaggedString {
    #[inline]
    fn deserialize( &self ) -> String {
        let pointer = self.pointer as *mut u8;
        let length = self.length as usize;

        if length == 0 {
            return String::new();
        }

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
            ffi::dealloc( pointer as *mut u8, length * mem::size_of::< SerializedValue >() );
        }

        vector
    }
}

/// Owns some memory allocated by FFI, and will deallocate it on drop.
///
/// `T` must not be zero-sized.
struct OwnedFfiSlice< T > {
    ptr: *const T,
    length: usize,
}

impl< T > OwnedFfiSlice< T > {
    unsafe fn new(ptr: *const T, length: usize) -> Self {
        assert_ne!( mem::size_of::< T >(), 0 );
        OwnedFfiSlice { ptr, length }
    }
}

impl< T > Deref for OwnedFfiSlice< T > {
    type Target = [ T ];

    fn deref( &self ) -> & [ T ] {
        assert_ne!( mem::size_of::< T >(), 0 );
        unsafe { slice::from_raw_parts ( self.ptr, self.length ) }
    }
}

impl< T > Drop for OwnedFfiSlice< T >  {
    fn drop( &mut self ) {
        assert_ne!( mem::size_of::< T >(), 0 );
        unsafe {
            ffi::dealloc( self.ptr as *mut u8, self.length * mem::size_of::< T >() );
        }
    }
}

pub struct ObjectDeserializer {
    key_slice: OwnedFfiSlice < SerializedUntaggedString >,
    value_slice: OwnedFfiSlice < SerializedValue< 'static > >,
    index: usize
}

impl Drop for ObjectDeserializer {
    fn drop( &mut self ) {
        // ensure that if this is dropped early, all the strings and any
        // allocated values are deserialized and then dropped.
        self.for_each( drop );
    }
}

impl Iterator for ObjectDeserializer {
    type Item = (String, Value);
    fn next( &mut self ) -> Option< Self::Item > {
        if self.index >= self.key_slice.len() {
            None
        } else {
            let key = self.key_slice[ self.index ].deserialize();
            let value = self.value_slice[ self.index ].deserialize();
            self.index += 1;
            Some( (key, value) )
        }
    }

    #[inline]
    fn size_hint( &self ) -> (usize, Option< usize >) {
        let remaining = self.key_slice.len() - self.index;
        (remaining, Some( remaining ))
    }
}

impl ExactSizeIterator for ObjectDeserializer {}

pub fn deserialize_object_to_iter( reference: &Reference ) -> ObjectDeserializer {
    let mut result: SerializedValue = Default::default();
    __js_raw_asm!( "\
        var object = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );\
        Module.STDWEB_PRIVATE.serialize_object( $1, object );",
        reference.as_raw(),
        &mut result as *mut _
    );

    assert_eq!( result.tag, Tag::Object );
    let result = result.as_object();

    let length = result.length as usize;
    let key_pointer = result.key_pointer as *const SerializedUntaggedString;
    let value_pointer = result.value_pointer as *const SerializedValue;

    // These structs will drop the FFI allocated slices when they're dropped
    //
    // The ObjectDeserializer will also iterate itself to completion on drop, to ensure
    // the strings and any allocated values are dropped and correctly deallocated.
    let key_slice = unsafe { OwnedFfiSlice::new( key_pointer, length ) };
    let value_slice = unsafe { OwnedFfiSlice::new( value_pointer, length ) };

    ObjectDeserializer {
        key_slice,
        value_slice,
        index: 0
    }
}

pub fn deserialize_object< R, F: FnOnce( &mut ObjectDeserializer ) -> R >( reference: &Reference, callback: F ) -> R {
    let mut iter = deserialize_object_to_iter( reference );

    let output = callback( &mut iter );

    drop(iter);

    output
}

pub struct ArrayDeserializer< 'a > {
    slice: &'a [SerializedValue< 'a >],
    index: usize
}

impl< 'a > Iterator for ArrayDeserializer< 'a > {
    type Item = Value;
    fn next( &mut self ) -> Option< Self::Item > {
        if self.index >= self.slice.len() {
            None
        } else {
            let value = self.slice[ self.index ].deserialize();
            self.index += 1;
            Some( value )
        }
    }

    #[inline]
    fn size_hint( &self ) -> (usize, Option< usize >) {
        let remaining = self.slice.len() - self.index;
        (remaining, Some( remaining ))
    }
}

impl< 'a > ExactSizeIterator for ArrayDeserializer< 'a > {}

pub fn deserialize_array< R, F: FnOnce( &mut ArrayDeserializer ) -> R >( reference: &Reference, callback: F ) -> R {
    let mut result: SerializedValue = Default::default();
    __js_raw_asm!( "\
        var array = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );\
        Module.STDWEB_PRIVATE.serialize_array( $1, array );",
        reference.as_raw(),
        &mut result as *mut _
    );

    assert_eq!( result.tag, Tag::Array );
    let result = result.as_array();

    let length = result.length as usize;
    let pointer = result.pointer as *const SerializedValue;

    let slice = unsafe { slice::from_raw_parts( pointer, length ) };
    let mut iter = ArrayDeserializer {
        slice,
        index: 0
    };

    let output = callback( &mut iter );

    // TODO: Panic-safety.
    unsafe {
        ffi::dealloc( pointer as *mut u8, length * mem::size_of::< SerializedValue >() );
    }

    output
}

impl SerializedUntaggedSymbol {
    #[inline]
    fn deserialize( &self ) -> Symbol {
        Symbol( self.id )
    }
}

impl SerializedUntaggedReference {
    #[inline]
    fn deserialize( &self ) -> Reference {
        unsafe { Reference::from_raw_unchecked_noref( self.refid ) }
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
untagged_boilerplate!( test_symbol, as_symbol, Tag::Symbol, SerializedUntaggedSymbol );
untagged_boilerplate!( test_reference, as_reference, Tag::Reference, SerializedUntaggedReference );
untagged_boilerplate!( test_function, as_function, Tag::Function, SerializedUntaggedFunction );
untagged_boilerplate!( test_function_mut, as_function_mut, Tag::FunctionMut, SerializedUntaggedFunctionMut );
untagged_boilerplate!( test_function_once, as_function_once, Tag::FunctionOnce, SerializedUntaggedFunctionOnce );
untagged_boilerplate!( test_unsafe_typed_array, as_unsafe_typed_array, Tag::UnsafeTypedArray, SerializedUntaggedUnsafeTypedArray );

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
            Tag::Reference => self.as_reference().deserialize().into(),
            Tag::Symbol => self.as_symbol().deserialize().into(),
            Tag::Function |
            Tag::FunctionMut |
            Tag::FunctionOnce |
            Tag::Object |
            Tag::Array |
            Tag::UnsafeTypedArray => unreachable!()
        }
    }
}

impl JsSerialize for () {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedUndefined.into()
    }
}

__js_serializable_boilerplate!( () );

impl JsSerialize for Undefined {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedUndefined.into()
    }
}

__js_serializable_boilerplate!( Undefined );

impl JsSerialize for Null {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedNull.into()
    }
}

__js_serializable_boilerplate!( Null );

impl JsSerialize for Symbol {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedSymbol {
            id: self.0
        }.into()
    }
}

__js_serializable_boilerplate!( Symbol );

impl JsSerialize for Reference {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedReference {
            refid: self.as_raw()
        }.into()
    }
}

__js_serializable_boilerplate!( Reference );

impl JsSerialize for bool {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        if *self {
            SerializedUntaggedTrue {}.into()
        } else {
            SerializedUntaggedFalse {}.into()
        }
    }
}

__js_serializable_boilerplate!( bool );

impl JsSerialize for str {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedString {
            pointer: self.as_ptr() as u32,
            length: self.len() as u32
        }.into()
    }
}

__js_serializable_boilerplate!( impl< 'a > for &'a str );

impl JsSerialize for String {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        self.as_str()._into_js()
    }
}

__js_serializable_boilerplate!( String );

impl JsSerialize for i8 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }
}

__js_serializable_boilerplate!( i8 );

impl JsSerialize for i16 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }
}

__js_serializable_boilerplate!( i16 );

impl JsSerialize for i32 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self
        }.into()
    }
}

__js_serializable_boilerplate!( i32 );

impl JsSerialize for u8 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }
}

__js_serializable_boilerplate!( u8 );

impl JsSerialize for u16 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedI32 {
            value: *self as i32
        }.into()
    }
}

__js_serializable_boilerplate!( u16 );

impl JsSerialize for u32 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedF64 {
            value: *self as f64
        }.into()
    }
}

__js_serializable_boilerplate!( u32 );

impl JsSerialize for f32 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedF64 {
            value: *self as f64
        }.into()
    }
}

__js_serializable_boilerplate!( f32 );

impl JsSerialize for f64 {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        SerializedUntaggedF64 {
            value: *self
        }.into()
    }
}

__js_serializable_boilerplate!( f64 );

impl JsSerialize for Number {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        use webcore::number::{Storage, get_storage};
        match *get_storage( self ) {
            Storage::I32( ref value ) => value._into_js(),
            Storage::F64( ref value ) => value._into_js()
        }
    }
}

__js_serializable_boilerplate!( Number );

impl< T: JsSerialize > JsSerialize for Option< T > {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        if let Some( value ) = self.as_ref() {
            value._into_js()
        } else {
            SerializedUntaggedNull.into()
        }
    }
}

__js_serializable_boilerplate!( impl< T > for Option< T > where T: JsSerialize );

impl< T: JsSerialize > JsSerialize for OptionalArg< T > {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        if let OptionalArg::Some( value ) = self.as_ref() {
            value._into_js()
        } else {
            SerializedUntaggedUndefined.into()
        }
    }
}

__js_serializable_boilerplate!( impl< T > for OptionalArg< T > where T: JsSerialize );

impl< T: JsSerialize > JsSerialize for [T] {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        let mut output = global_arena::reserve( self.len() );
        for value in self {
            unsafe {
                output.append( value._into_js() );
            }
        }

        SerializedUntaggedArray {
            pointer: output.offset() as u32,
            length: output.len() as u32
        }.into()
    }
}

__js_serializable_boilerplate!( impl< 'a, T > for &'a [T] where T: JsSerialize );

impl< T: JsSerialize > JsSerialize for Vec< T > {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        self.as_slice()._into_js()
    }
}

__js_serializable_boilerplate!( impl< T > for Vec< T > where T: JsSerialize );

fn object_into_js< 'a, K: AsRef< str >, V: 'a + JsSerialize, I: Iterator< Item = (K, &'a V) > + ExactSizeIterator >( iter: I ) -> SerializedValue< 'a > {
    let mut keys = global_arena::reserve( iter.len() );
    let mut values = global_arena::reserve( iter.len() );
    for (key, value) in iter {
        unsafe {
            keys.append( key.as_ref()._into_js().as_string().clone() );
            values.append( value._into_js() );
        }
    }

    SerializedUntaggedObject {
        key_pointer: keys.offset() as u32,
        value_pointer: values.offset() as u32,
        length: keys.len() as u32
    }.into()
}

impl< K: AsRef< str >, V: JsSerialize > JsSerialize for BTreeMap< K, V > {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        object_into_js( self.iter() )
    }
}

__js_serializable_boilerplate!( impl< K, V > for BTreeMap< K, V > where K: AsRef< str >, V: JsSerialize );

impl< K: AsRef< str > + Eq + Hash, V: JsSerialize > JsSerialize for HashMap< K, V > {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        object_into_js( self.iter() )
    }
}

__js_serializable_boilerplate!( impl< K, V > for HashMap< K, V > where K: AsRef< str > + Eq + Hash, V: JsSerialize );

impl JsSerialize for Value {
    #[doc(hidden)]
    fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
        match *self {
            Value::Undefined => SerializedUntaggedUndefined.into(),
            Value::Null => SerializedUntaggedNull.into(),
            Value::Bool( ref value ) => value._into_js(),
            Value::Number( ref value ) => value._into_js(),
            Value::Symbol( ref value ) => value._into_js(),
            Value::String( ref value ) => value._into_js(),
            Value::Reference( ref value ) => value._into_js()
        }
    }
}

__js_serializable_boilerplate!( Value );

macro_rules! impl_for_unsafe_typed_array {
    ($ty:ty, $kind:expr) => {
        impl< 'r > JsSerialize for UnsafeTypedArray< 'r, $ty > {
            #[doc(hidden)]
            #[inline]
            fn _into_js< 'a >( &'a self ) -> SerializedValue< 'a > {
                SerializedUntaggedUnsafeTypedArray {
                    pointer: self.0.as_ptr() as u32 / mem::size_of::< $ty >() as u32,
                    length: self.0.len() as u32,
                    kind: $kind
                }.into()
            }
        }

        __js_serializable_boilerplate!( impl< 'a > for UnsafeTypedArray< 'a, $ty > );
    }
}

impl_for_unsafe_typed_array!( u8, 0 );
impl_for_unsafe_typed_array!( i8, 1 );
impl_for_unsafe_typed_array!( u16, 2 );
impl_for_unsafe_typed_array!( i16, 3 );
impl_for_unsafe_typed_array!( u32, 4 );
impl_for_unsafe_typed_array!( i32, 5 );
impl_for_unsafe_typed_array!( f32, 6 );
impl_for_unsafe_typed_array!( f64, 7 );

#[derive(Debug)]
pub struct FunctionTag;

#[derive(Debug)]
pub struct NonFunctionTag;

impl< T: JsSerialize > JsSerializeOwned for Newtype< (NonFunctionTag, ()), T > {
    #[inline]
    fn into_js_owned< 'x >( value: &'x mut Option< Self > ) -> SerializedValue< 'x > {
        JsSerialize::_into_js( value.as_ref().unwrap().as_ref() )
    }
}

trait FuncallAdapter< F > {
    extern fn funcall_adapter( callback: *mut F, raw_arguments: *mut SerializedUntaggedArray );
    extern fn deallocator( callback: *mut F );
}

macro_rules! impl_for_fn_and_modifier {
    (
        args: ($($kind:ident),*),
        trait: $trait:ident,
        wrapped type: $wrappedtype:ty,
        unwrap: $wrapped:ident => $unwrap:expr,
        serialized to: $serialized_to:tt,
        call: $callback:ident => $call:expr
    ) => {
        impl< $($kind: TryFrom< Value >,)* F > FuncallAdapter< F > for Newtype< (FunctionTag, ($($kind,)*)), $wrappedtype >
            where F: $trait< ($($kind,)*) > + 'static, F::Output: JsSerializeOwned
        {
            #[allow(unused_mut, unused_variables, non_snake_case)]
            extern fn funcall_adapter(
                    $callback: *mut F,
                    raw_arguments: *mut SerializedUntaggedArray
                )
            {
                let mut arguments = unsafe { &*raw_arguments }.deserialize();

                unsafe {
                    ffi::dealloc( raw_arguments as *mut u8, mem::size_of::< SerializedValue >() );
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
                            panic!(
                                "Argument #{} is not convertible to '{}'",
                                nth_argument + 1,
                                type_name::< $kind >()
                            );
                        }
                    };

                    nth_argument += 1;
                )*

                $crate::private::noop( &mut nth_argument );

                let result = $call;

                let mut result = Some( result );
                let result = JsSerializeOwned::into_js_owned( &mut result );
                let result = &result as *const _;

                // This is kinda hacky but I'm not sure how else to do it at the moment.
                __js_raw_asm!( "Module.STDWEB_PRIVATE.tmp = Module.STDWEB_PRIVATE.to_js( $0 );", result );
            }

            extern fn deallocator( callback: *mut F ) {
                let callback = unsafe {
                    Box::from_raw( callback )
                };

                drop( callback );
            }
        }

        impl< $($kind: TryFrom< Value >,)* F > JsSerializeOwned for Newtype< (FunctionTag, ($($kind,)*)), $wrappedtype >
            where F: $trait< ($($kind,)*) > + 'static, F::Output: JsSerializeOwned
        {
            #[inline]
            fn into_js_owned< 'a >( value: &'a mut Option< Self > ) -> SerializedValue< 'a > {
                let $wrapped = value.take().unwrap().unwrap_newtype();
                let callback: *mut F = Box::into_raw( Box::new( $unwrap ) );
                let adapter_pointer = <Self as FuncallAdapter< F > >::funcall_adapter;
                let deallocator_pointer = <Self as FuncallAdapter< F > >::deallocator;
                $serialized_to {
                    adapter_pointer: adapter_pointer as u32,
                    pointer: callback as u32,
                    deallocator_pointer: deallocator_pointer as u32
                }.into()
            }
        }

        impl< $($kind: TryFrom< Value >,)* F > JsSerializeOwned for Newtype< (FunctionTag, ($($kind,)*)), Option< $wrappedtype > >
            where F: $trait< ($($kind,)*) > + 'static, F::Output: JsSerializeOwned
        {
            #[inline]
            fn into_js_owned< 'a >( value: &'a mut Option< Self > ) -> SerializedValue< 'a > {
                if let Some( $wrapped ) = value.take().unwrap().unwrap_newtype() {
                    let callback: *mut F = Box::into_raw( Box::new( $unwrap ) );
                    let adapter_pointer = <Newtype< (FunctionTag, ($($kind,)*)), $wrappedtype > as FuncallAdapter< F > >::funcall_adapter;
                    let deallocator_pointer = <Newtype< (FunctionTag, ($($kind,)*)), $wrappedtype > as FuncallAdapter< F > >::deallocator;
                    $serialized_to {
                        adapter_pointer: adapter_pointer as u32,
                        pointer: callback as u32,
                        deallocator_pointer: deallocator_pointer as u32
                    }.into()
                } else {
                    SerializedUntaggedNull.into()
                }
            }
        }
    }
}

macro_rules! impl_for_fn {
    ($next:tt => $($kind:ident),*) => {
        impl_for_fn_and_modifier!(
            args: ($($kind),*),
            trait: CallMut,
            wrapped type: F,
            unwrap: f => f,
            serialized to: SerializedUntaggedFunction,
            call: f => { unsafe { &mut *f }.call_mut( ($($kind,)*) ) }
        );

        impl_for_fn_and_modifier!(
            args: ($($kind),*),
            trait: CallMut,
            wrapped type: Mut<F>,
            unwrap: f => {f.0},
            serialized to: SerializedUntaggedFunctionMut,
            call: f => { unsafe { &mut *f }.call_mut( ($($kind,)*) ) }
        );

        impl_for_fn_and_modifier!(
            args: ($($kind),*),
            trait: CallOnce,
            wrapped type: Once<F>,
            unwrap: f => {f.0},
            serialized to: SerializedUntaggedFunctionOnce,
            call: f => { unsafe { Box::from_raw( f ) }.call_once( ($($kind,)*) ) }
        );

        next! { $next }
    }
}

loop_through_identifiers!( impl_for_fn );

impl< 'a, T: ?Sized + JsSerialize > JsSerialize for &'a T {
    #[doc(hidden)]
    #[inline]
    fn _into_js< 'x >( &'x self ) -> SerializedValue< 'x > {
        T::_into_js( *self )
    }
}

impl JsSerialize for ConversionError {
    #[doc(hidden)]
    fn _into_js< 'x >( &'x self ) -> SerializedValue< 'x > {
        let type_error: TypeError = self.into();
        let reference: Reference = type_error.into();
        let value: Value = reference.into();
        global_arena::serialize_value( value )
    }
}

#[cfg(test)]
mod test_deserialization {
    use std::rc::Rc;
    use std::cell::{Cell, RefCell};
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
    fn empty_string() {
        assert_eq!( js! { return ""; }, Value::String( "".to_string() ) );
    }

    #[test]
    fn symbol() {
        let value = js! { return Symbol(); };
        assert!( value.is_symbol() );
    }

    #[test]
    fn array() {
        assert_eq!( js! { return [1, 2]; }.is_array(), true );
    }

    #[test]
    fn object() {
        assert_eq!( js! { return {"one": 1, "two": 2}; }.is_object(), true );
    }

    #[test]
    fn object_into_btreemap() {
        let object = js! { return {"one": 1, "two": 2}; }.into_object().unwrap();
        let object: BTreeMap< String, Value > = object.into();
        assert_eq!( object, [
            ("one".to_string(), Value::Number(1.into())),
            ("two".to_string(), Value::Number(2.into()))
        ].iter().cloned().collect() );
    }

    #[test]
    fn object_into_hashmap() {
        let object = js! { return {"one": 1, "two": 2}; }.into_object().unwrap();
        let object: HashMap< String, Value > = object.into();
        assert_eq!( object, [
            ("one".to_string(), Value::Number(1.into())),
            ("two".to_string(), Value::Number(2.into()))
        ].iter().cloned().collect() );
    }

    #[test]
    fn array_into_vector() {
        let array = js! { return ["one", 1]; }.into_array().unwrap();
        let array: Vec< Value > = array.into();
        assert_eq!( array, &[
            Value::String( "one".to_string() ),
            Value::Number( 1.into() )
        ]);
    }

    #[test]
    fn reference() {
        assert_eq!( js! { return new Date(); }.is_reference(), true );
    }

    #[test]
    fn bad_reference() {
        assert_eq!( js! {
            var WeakMapProto = WeakMap.prototype;
            if (WeakMapProto.BAD_REFERENCE === undefined) {
                WeakMapProto.BAD_REFERENCE = {};
                WeakMapProto.oldSet = WeakMapProto.set;
                WeakMapProto.set = function(key, value) {
                    if (key === WeakMapProto.BAD_REFERENCE) {
                        throw new TypeError("BAD_REFERENCE");
                    } else {
                        return this.oldSet(key, value);
                    }
                };
            }
            return WeakMapProto.BAD_REFERENCE;
        }.is_reference(), true );
    }

    #[test]
    fn arguments() {
        let value = js! {
            return (function() {
                return arguments;
            })( 1, 2 );
        };

        assert_eq!( value.is_array(), false );
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

    #[test]
    fn function_once() {
        fn call< F: FnOnce( String ) -> String + 'static >( callback: F ) -> Value {
            js!(
                var callback = @{Once( callback )};
                return callback( "Dog" );
            )
        }

        let suffix = "!".to_owned();
        let result = call( move |value| { return value + suffix.as_str() } );
        assert_eq!( result, Value::String( "Dog!".to_owned() ) );
    }

    #[test]
    fn function_mut() {
        let mut count = 0;
        let callback = move || -> i32 {
            count += 1;
            count
        };
        let callback = js! { return @{Mut(callback)}; };
        assert_eq!({ let x : i32 = js!{ return @{&callback}(); }.try_into().unwrap(); x }, 1);
        assert_eq!({ let x : i32 = js!{ return @{&callback}(); }.try_into().unwrap(); x }, 2);
        assert_eq!({ let x : i32 = js!{ return @{&callback}(); }.try_into().unwrap(); x }, 3);
        js!{ @{callback}.drop(); };
    }

    #[test]
    fn function_once_cannot_be_called_twice() {
        fn call< F: FnOnce() + 'static >( callback: F ) -> Value {
            js!(
                var callback = @{Once( callback )};
                callback();

                try {
                    callback();
                } catch( error ) {
                    if( error instanceof ReferenceError ) {
                        return true;
                    }
                }

                return false;
            )
        }

        let result = call( move || {} );
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn function_once_cannot_be_called_after_being_dropped() {
        fn call< F: FnOnce() + 'static >( callback: F ) -> Value {
            js!(
                var callback = @{Once( callback )};
                callback.drop();

                try {
                    callback();
                } catch( error ) {
                    if( error instanceof ReferenceError ) {
                        return true;
                    }
                }

                return false;
            )
        }

        let result = call( move || {} );
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn function_once_calling_drop_after_being_called_does_not_do_anything() {
        fn call< F: FnOnce() + 'static >( callback: F ) -> Value {
            js!(
                var callback = @{Once( callback )};
                callback();
                callback.drop();

                return true;
            )
        }

        let result = call( move || {} );
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn function_once_calling_drop_twice_does_not_do_anything() {
        fn call< F: FnOnce() + 'static >( callback: F ) -> Value {
            js!(
                var callback = @{Once( callback )};
                callback.drop();
                callback.drop();

                return true;
            )
        }

        let result = call( move || {} );
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn issue_273() {
        let mut count = 0;
        let f = move |callback: ::stdweb::Value| {
            count += 1;
            js! {
                @{callback}();
            };
        };

        let result = js! {
            let f = @{Mut(f)};

            let caught = false;

            try {
                f(function () {
                    f(function() {});
                });
            } catch ( error ) {
                if( error instanceof ReferenceError ) {
                    caught = true;
                }
            }

            f.drop();

            return caught;
        };
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn issue_277() {
        struct MyStruct {
            was_dropped: bool
        }
        impl MyStruct {
            fn consume(self) {}
        }
        impl Drop for MyStruct {
            fn drop(&mut self) {
                assert_eq!(self.was_dropped, false);
                self.was_dropped = true;
            }
        }

        let s = MyStruct { was_dropped: false };

        let f = move || -> () {
            s.consume();
            unreachable!(); // never actually called
        };

        js! {
            let f = @{Once(f)};

            let drop = f.drop;
            drop();
            drop();
        };
    }

    #[test]
    fn test_closure_dropped_while_being_called_is_dropped_after_it_returns() {
        struct MarkTrueOnDrop( Rc< Cell< bool > > );
        impl Drop for MarkTrueOnDrop {
            fn drop( &mut self ) {
                self.0.set( true );
            }
        }

        let was_dropped = Rc::new( Cell::new( false ) );
        let was_dropped_clone = was_dropped.clone();
        let callback = move |itself: Value, check_if_dropped: Value| {
            let _mark_true_on_drop = MarkTrueOnDrop( was_dropped_clone.clone() );
            js!(
                @{itself}.drop();
                @{check_if_dropped}();
            );
        };

        let check_if_dropped = move || {
            assert_eq!( was_dropped.get(), false );
        };

        js!(
            var callback = @{callback};
            callback( callback, @{Once( check_if_dropped )} );
        );
    }

    #[test]
    fn test_dropping_the_closure_while_it_is_being_called_will_make_future_calls_throw() {
        #[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
        #[reference(instance_of = "ReferenceError")]
        pub struct ReferenceError( Reference );

        let counter = Rc::new( Cell::new( 0 ) );
        let counter_clone = counter.clone();
        let caught_value = Rc::new( RefCell::new( Value::Null ) );
        let caught_value_clone = caught_value.clone();
        let callback = move |itself: Value| {
            let value = counter_clone.get();
            counter_clone.set( value + 1 );

            if value == 0 {
                let caught = js!(
                    var callback = @{itself};
                    callback.drop();

                    var caught = null;
                    try {
                        callback( callback );
                    } catch( error ) {
                        caught = error;
                    }

                    return caught;
                );
                *caught_value_clone.borrow_mut() = caught;
            }
        };

        js!(
            var callback = @{callback};
            callback( callback );
        );

        assert_eq!( counter.get(), 1 );
        let reference_error: Result< ReferenceError, _ > = caught_value.borrow().clone().try_into();
        assert!( reference_error.is_ok() );
    }

    #[test]
    fn no_return() {
        let values = Rc::new( RefCell::new( Vec::new() ) );
        let values_clone = values.clone();
        let callback = move |value: i32| {
            values_clone.borrow_mut().push( value );
        };

        let _: () = js!( @(no_return)
            var cb = @{Mut( callback.clone() )};
            cb( 1 );
            cb.drop();
        );

        let _: () = js!( @(no_return)
            var cb = @{Mut( callback.clone() )};
            cb( 2 );
            cb.drop();
            return 20;
        );

        let a: Value = js!(
            var cb = @{Mut( callback.clone() )};
            cb( 3 );
            cb.drop();
        );

        let b: Value = js!(
            var cb = @{Mut( callback )};
            cb( 4 );
            cb.drop();
            return 40;
        );

        assert_eq!( *values.borrow(), &[1, 2, 3, 4] );
        assert_eq!( a, Value::Undefined );
        assert_eq!( b, 40 )
    }
}

#[cfg(test)]
mod test_serialization {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn object_from_btreemap() {
        let object: BTreeMap< _, _ > = [
            ("number".to_string(), Value::Number( 123.into() )),
            ("string".to_string(), Value::String( "Hello!".into() ))
        ].iter().cloned().collect();

        let result = js! {
            var object = @{object};
            return object.number === 123 && object.string === "Hello!" && Object.keys( object ).length === 2;
        };
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn object_from_borrowed_btreemap() {
        let object: BTreeMap< _, _ > = [
            ("number".to_string(), Value::Number( 123.into() ))
        ].iter().cloned().collect();

        let result = js! {
            var object = @{&object};
            return object.number === 123 && Object.keys( object ).length === 1;
        };
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn object_from_btreemap_with_convertible_key_and_value() {
        let key: Cow< str > = "number".into();
        let object: BTreeMap< _, _ > = [
            (key, 123)
        ].iter().cloned().collect();

        let result = js! {
            var object = @{object};
            return object.number === 123 && Object.keys( object ).length === 1;
        };
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn object_from_hashmap() {
        let object: HashMap< _, _ > = [
            ("number".to_string(), Value::Number( 123.into() )),
            ("string".to_string(), Value::String( "Hello!".into() ))
        ].iter().cloned().collect();

        let result = js! {
            var object = @{object};
            return object.number === 123 && object.string === "Hello!" && Object.keys( object ).length === 2;
        };
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn vector_of_strings() {
        let vec: Vec< _ > = vec![
            "one".to_string(),
            "two".to_string()
        ];

        let result = js! {
            var vec = @{vec};
            return vec[0] === "one" && vec[1] === "two" && vec.length === 2;
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
            return Object.prototype.toString.call( callback ) === "[object Function]" &&
                Object.prototype.toString.call( reference ) === "[object Date]" &&
                Object.prototype.toString.call( string ) === "[object String]"
        };
        assert_eq!( result, Value::Bool( true ) );
    }

    #[test]
    fn serialize_0() {
        assert_eq!(
            js! { return 0; },
            0
        );
    }

    #[test]
    fn serialize_1() {
        assert_eq!(
            js! { return @{1}; },
            1
        );
    }

    #[test]
    fn serialize_2() {
        assert_eq!(
            js! { return @{1} + @{2}; },
            1 + 2
        );
    }

    #[test]
    fn serialize_3() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3}; },
            1 + 2 + 3
        );
    }

    #[test]
    fn serialize_4() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4}; },
            1 + 2 + 3 + 4
        );
    }

    #[test]
    fn serialize_5() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5}; },
            1 + 2 + 3 + 4 + 5
        );
    }

    #[test]
    fn serialize_6() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6}; },
            1 + 2 + 3 + 4 + 5 + 6
        );
    }

    #[test]
    fn serialize_7() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7
        );
    }

    #[test]
    fn serialize_8() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8
        );
    }

    #[test]
    fn serialize_9() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9
        );
    }

    #[test]
    fn serialize_10() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9} + @{10}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10
        );
    }

    #[test]
    fn serialize_11() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9} + @{10} + @{11}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11
        );
    }

    #[test]
    fn serialize_12() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9} + @{10} + @{11} + @{12}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12
        );
    }

    #[test]
    fn serialize_13() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9} + @{10} + @{11} + @{12} + @{13}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13
        );
    }

    #[test]
    fn serialize_14() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9} + @{10} + @{11} + @{12} + @{13} + @{14}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14
        );
    }

    #[test]
    fn serialize_15() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9} + @{10} + @{11} + @{12} + @{13} + @{14} + @{15}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15
        );
    }

    #[test]
    fn serialize_16() {
        assert_eq!(
            js! { return @{1} + @{2} + @{3} + @{4} + @{5} + @{6} + @{7} + @{8} + @{9} + @{10} + @{11} + @{12} + @{13} + @{14} + @{15} + @{16}; },
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16
        );
    }

    #[test]
    fn interpolated_args_are_converted_at_the_start() {
        let mut string = "1".to_owned();
        let callback = js! {
            return function() {
                return @{&string};
            }
        };

        unsafe {
            string.as_bytes_mut()[0] = b'2';
        }

        let result = js! {
            return @{callback}();
        };

        assert_eq!( result, "1" );
    }

    macro_rules! test_unsafe_typed_array {
        ($test_name:ident, $ty:ty, $js_type_name:ident) => {
            #[allow(trivial_numeric_casts)]
            #[test]
            fn $test_name() {
                let slice: &[$ty] = &[1 as $ty, 2 as $ty, 3 as $ty];
                let slice = unsafe { UnsafeTypedArray::new( slice ) };
                let result: Vec< Value > = js!(
                    var slice = @{slice};
                    var sum = slice[0] + slice[1] + slice[2];
                    var name = slice.constructor.name;
                    var length = slice.length;
                    return [name, sum, length]
                ).try_into().unwrap();

                let mut result = result.into_iter();
                let name: String = result.next().unwrap().try_into().unwrap();
                let sum: f64 = result.next().unwrap().try_into().unwrap();
                let length: usize = result.next().unwrap().try_into().unwrap();

                assert_eq!( name, stringify!( $js_type_name ) );
                assert_eq!( sum as u64, 6 );
                assert_eq!( length, 3 );
            }
        }
    }

    test_unsafe_typed_array!( test_unsafe_typed_array_u8, u8, Uint8Array );
    test_unsafe_typed_array!( test_unsafe_typed_array_i8, i8, Int8Array );
    test_unsafe_typed_array!( test_unsafe_typed_array_u16, u16, Uint16Array );
    test_unsafe_typed_array!( test_unsafe_typed_array_i16, i16, Int16Array );
    test_unsafe_typed_array!( test_unsafe_typed_array_u32, u32, Uint32Array );
    test_unsafe_typed_array!( test_unsafe_typed_array_i32, i32, Int32Array );
    test_unsafe_typed_array!( test_unsafe_typed_array_f32, f32, Float32Array );
    test_unsafe_typed_array!( test_unsafe_typed_array_f64, f64, Float64Array );
}

// TODO: Move this back inside the test module.
//
// This had to be temporarily moved here due to a bug in Rust
// where the following error is generated if it's defined under
// the module:
//
//    error: cannot determine resolution for the attribute macro `reference`
//
#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Error")]
pub struct TestError( Reference );

#[cfg(test)]
mod test_reserialization {
    use super::*;
    use webcore::array::Array;

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
    fn empty_string() {
        assert_eq!( js! { return @{""}; }, Value::String( "".to_string() ) );
    }

    #[test]
    fn string_with_non_bmp_character() {
        assert_eq!( js! { return @{"😐"} + ", 😐"; }, Value::String( "😐, 😐".to_string() ) );
    }

    #[test]
    fn array() {
        let array: Array = vec![ Value::Number( 1.into() ), Value::Number( 2.into() ) ].into();
        assert_eq!( js! { return @{&array}; }.into_reference().unwrap(), *array.as_ref() );
    }

    #[test]
    fn array_values_are_not_compared_by_value() {
        let array: Array = vec![ Value::Number( 1.into() ), Value::Number( 2.into() ) ].into();
        assert_ne!( js! { return @{&[1, 2][..]}; }.into_reference().unwrap(), *array.as_ref() );
    }

    #[test]
    fn object() {
        let object: BTreeMap< _, _ > = [
            ("one".to_string(), Value::Number( 1.into() )),
            ("two".to_string(), Value::Number( 2.into() ))
        ].iter().cloned().collect();

        let object: Value = object.into();
        assert_eq!( js! { return @{&object} }, object );
    }

    #[test]
    fn symbol() {
        let value_1: Symbol = js! { return Symbol(); }.try_into().unwrap();
        let value_2: Symbol = js! { return @{&value_1}; }.try_into().unwrap();
        assert_eq!( value_1, value_2 );
        assert_eq!( js! { return @{value_1} === @{value_2}; }, true );
    }

    #[test]
    fn cloned_symbol() {
        let value_1: Symbol = js! { return Symbol(); }.try_into().unwrap();
        let value_2 = value_1.clone();
        assert_eq!( value_1, value_2 );
        assert_eq!( js! { return @{value_1} === @{value_2}; }, true );
    }

    #[test]
    fn different_symbols() {
        let value_1: Symbol = js! { return Symbol(); }.try_into().unwrap();
        let value_2: Symbol = js! { return Symbol(); }.try_into().unwrap();
        assert_ne!( value_1, value_2 );
        assert_eq!( js! { return @{value_1} !== @{value_2}; }, true );
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
    fn optional_arg_some() {
        assert_eq!( js! { return @{OptionalArg::Some( true )}; }, Value::Bool( true ) );
    }

    #[test]
    fn optional_arg_none() {
        let boolean_none: OptionalArg< bool > = OptionalArg::None;
        assert_eq!( js! { return @{boolean_none}; }, Value::Undefined );
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

    #[test]
    fn string_identity_function() {
        fn identity( string: String ) -> String {
            string
        }

        let empty = js! {
            var identity = @{identity};
            return identity( "" );
        };

        assert_eq!( empty, Value::String( "".to_string() ) );

        let non_empty = js! {
            var identity = @{identity};
            return identity( "死神はりんごしか食べない!" );
        };

        assert_eq!( non_empty, Value::String( "死神はりんごしか食べない!".to_string() ) );
    }

    type Error = TestError;

    #[test]
    fn closure_returning_reference_object() {
        fn identity( error: Error ) -> Error {
            error
        }

        let value = js! {
            var identity = @{identity};
            return identity( new ReferenceError() );
        };

        assert!( instanceof!( value, Error ) );
    }
}

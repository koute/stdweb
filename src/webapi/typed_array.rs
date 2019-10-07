use std::marker::PhantomData;
use std::mem::size_of;
use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::instance_of::InstanceOf;
use webapi::array_buffer::ArrayBuffer;

pub trait ArrayKind: Sized {
    fn is_typed_array( reference: &Reference ) -> bool;
    fn into_typed_array( slice: &[Self] ) -> TypedArray< Self >;
    fn into_typed_array_from_array_buffer( buffer: &ArrayBuffer ) -> TypedArray< Self >;
    fn from_typed_array( array: &TypedArray< Self > ) -> Vec< Self >;
}

macro_rules! arraykind {
    ($element_type:ty, $js_array_type:ident, $heap_type:ident) => {
        impl ArrayKind for $element_type {
            fn is_typed_array( reference: &Reference ) -> bool {
                instanceof!( *reference, $js_array_type )
            }

            fn into_typed_array( slice: &[Self] ) -> TypedArray< Self > {
                let slice_ptr = (slice.as_ptr() as usize / size_of::<$element_type>()) as i32;
                let raw = __js_raw_asm_int!(
                    concat!(
                        "return Module.STDWEB_PRIVATE.acquire_rust_reference( Module.",
                        stringify!($heap_type),
                        ".slice( $0, $1 ) );"
                    ),
                    slice_ptr,
                    slice_ptr + slice.len() as i32
                );

                let reference = unsafe {
                    Reference::from_raw_unchecked_noref( raw )
                };

                reference.downcast().unwrap()
            }

            fn into_typed_array_from_array_buffer( buffer: &ArrayBuffer ) -> TypedArray< Self > {
                let raw = __js_raw_asm_int!(
                    concat!(
                        "return Module.STDWEB_PRIVATE.acquire_rust_reference( new ",
                        stringify!( $js_array_type ),
                        "( Module.STDWEB_PRIVATE.acquire_js_reference( $0 ) )",
                        " );"
                    ),
                    buffer.as_ref().as_raw()
                );

                let reference = unsafe { Reference::from_raw_unchecked_noref( raw ) };
                reference.downcast().unwrap()
            }

            fn from_typed_array( array: &TypedArray< Self > ) -> Vec< Self > {
                let length = array.len() as usize;
                let mut vector = Vec::with_capacity( length );
                let vec_ptr = (vector.as_ptr() as usize / size_of::<$element_type>()) as i32;

                js!( @(no_return)
                    var array = @{array};
                    var pointer = @{vec_ptr};
                    Module.$heap_type.set( array, pointer );
                );

                unsafe {
                    vector.set_len( length );
                }

                vector
            }
        }

        impl From< TypedArray< $element_type > > for Vec< $element_type > {
            fn from( array: TypedArray< $element_type > ) -> Self {
                <$element_type>::from_typed_array( &array )
            }
        }

        impl< 'a > From< &'a TypedArray< $element_type > > for Vec< $element_type > {
            fn from( array: &'a TypedArray< $element_type > ) -> Self {
                <$element_type>::from_typed_array( array )
            }
        }
    }
}

arraykind!( i8, Int8Array, HEAP8 );
arraykind!( u8, Uint8Array, HEAPU8 );
arraykind!( i16, Int16Array, HEAP16 );
arraykind!( u16, Uint16Array, HEAPU16 );
arraykind!( i32, Int32Array, HEAP32 );
arraykind!( u32, Uint32Array, HEAPU32 );
arraykind!( f32, Float32Array, HEAPF32 );
arraykind!( f64, Float64Array, HEAPF64 );

impl< T: ArrayKind > InstanceOf for TypedArray< T > {
    #[inline]
    fn instance_of( reference: &Reference ) -> bool {
        T::is_typed_array( reference )
    }
}

/// JavaScript typed arrays are array-like objects and provide a mechanism for accessing raw binary data.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Typed_arrays)
// https://www.ecma-international.org/ecma-262/6.0/#sec-typedarray-objects
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
pub struct TypedArray< T: ArrayKind >( Reference, PhantomData< T > );

impl< T: ArrayKind > TypedArray< T > {
    /// Returns the [TypedArray](struct.ArrayBuffer.html) referenced by this typed array.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/buffer)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-get-%typedarray%.prototype.buffer
    pub fn buffer( &self ) -> ArrayBuffer {
        js!( return @{self}.buffer; ).try_into().unwrap()
    }

    /// Returns the number of elements in the buffer.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/length)
    pub fn len( &self ) -> u32 {
        let reference = self.as_ref();
        js!( return @{reference}.length; ).try_into().unwrap()
    }

    /// Copies `self` into a new `Vec`.
    pub fn to_vec( &self ) -> Vec< T > {
        T::from_typed_array( self )
    }
}

impl< 'a, T: ArrayKind > From< &'a [T] > for TypedArray< T > {
    fn from( slice: &'a [T] ) -> Self {
        T::into_typed_array( slice )
    }
}

impl< T: ArrayKind > From< ArrayBuffer > for TypedArray< T > {
    fn from( buffer: ArrayBuffer ) -> Self {
        T::into_typed_array_from_array_buffer( &buffer )
    }
}

impl< 'a, T: ArrayKind > From< &'a ArrayBuffer > for TypedArray< T > {
    fn from( buffer: &'a ArrayBuffer ) -> Self {
        T::into_typed_array_from_array_buffer( buffer )
    }
}

#[cfg(test)]
mod tests {
    use super::TypedArray;
    use webcore::try_from::TryInto;
    use webapi::array_buffer::ArrayBuffer;

    macro_rules! arraykind_test {
        ($element_type: ident, $js_array_type: ident) => {
            mod $element_type {
                use super::super::TypedArray;
                use std;
                use webcore::try_from::TryInto;
                use webcore::value::Value;
                use webapi::array_buffer::ArrayBuffer;

                const ARRAY: &[$element_type] = &[
                    std::$element_type::MIN,
                    std::$element_type::MAX
                ];

                #[test]
                fn into() {
                    let typed_array: TypedArray< $element_type > = ARRAY.into();
                    assert_eq!(
                        js!( return @{&typed_array} instanceof $js_array_type; ),
                        Value::Bool( true )
                    );
                    assert_eq!(
                        js!( return @{&typed_array}.length === @{ARRAY.len() as u32}; ),
                        Value::Bool( true )
                    );
                    assert_eq!(
                        js!( return @{&typed_array}[0] === @{ARRAY[0]}; ),
                        Value::Bool( true )
                    );
                    assert_eq!(
                        js!( return @{&typed_array}[1] === @{ARRAY[1]}; ),
                        Value::Bool( true )
                    );
                }

                #[test]
                fn from() {
                    let value = js!( return new $js_array_type( [@{ARRAY[0]}, @{ARRAY[1]}] ); );
                    let typed_array: TypedArray< $element_type > = value.try_into().unwrap();
                    let vec: Vec< $element_type > = typed_array.into();
                    assert_eq!( vec.len(), ARRAY.len() );
                    assert_eq!( vec, ARRAY);
                }

                #[test]
                fn from_array_buffer() {
                    let value = js!( return new $js_array_type( [@{ARRAY[0]}, @{ARRAY[1]}] ).buffer; );
                    let array_buffer: ArrayBuffer = value.try_into().unwrap();
                    let typed_array: TypedArray< $element_type > = array_buffer.into();
                    let vec: Vec< $element_type > = typed_array.into();
                    assert_eq!( vec.len(), ARRAY.len() );
                    assert_eq!( vec, ARRAY );
                }
            }
        }
    }

    arraykind_test!(i8, Int8Array);
    arraykind_test!(u8, Uint8Array);
    arraykind_test!(i16, Int16Array);
    arraykind_test!(u16, Uint16Array);
    arraykind_test!(i32, Int32Array);
    arraykind_test!(u32, Uint32Array);
    arraykind_test!(f32, Float32Array);
    arraykind_test!(f64, Float64Array);

    fn get_refcount() -> i32 {
        js!( return Object.keys( Module.STDWEB_PRIVATE.id_to_ref_map ).length; ).try_into().unwrap()
    }

    #[test]
    fn slice_to_typed_array_does_not_leak() {
        let initial_refcount = get_refcount();
        {
            let vec: Vec< i32 > = (0..10).collect();
            let _: TypedArray< i32 > = vec[..].into();
        }
        assert_eq!( initial_refcount, get_refcount() );
    }

    #[test]
    fn array_buffer_to_typed_array_does_not_leak() {
        let initial_refcount = get_refcount();
        {
            let array_buffer = ArrayBuffer::new( 16 ).unwrap();
            let _: TypedArray< i32 > = array_buffer.into();
        }
        assert_eq!( initial_refcount, get_refcount() );
    }
}

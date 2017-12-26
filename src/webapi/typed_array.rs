use std::marker::PhantomData;
use std::mem::size_of;
use webcore::value::{Reference, FromReference};
use webcore::try_from::TryInto;
use webapi::array_buffer::ArrayBuffer;

pub trait ArrayKind: Sized {
    fn is_typed_array( reference: &Reference ) -> bool;
    fn into_typed_array( slice: &[Self] ) -> TypedArray< Self >;
    unsafe fn into_typed_array_no_copy( slice: &[Self] ) -> TypedArray< Self >;
    fn from_typed_array( array: &TypedArray< Self > ) -> Vec< Self >;
}

macro_rules! arraykind {
    ($element_type:ty, $js_array_type:ident, $heap_type:ident, $ptr_type:ty) => {
        impl ArrayKind for $element_type {
            fn is_typed_array( reference: &Reference ) -> bool {
                instanceof!( *reference, $js_array_type )
            }

            fn into_typed_array( slice: &[Self] ) -> TypedArray< Self > {
                let slice_ptr = (slice.as_ptr() as usize / size_of::<$element_type>()) as $ptr_type;
                let raw = __js_raw_asm!(
                    concat!(
                        "return Module.STDWEB.acquire_rust_reference( ",
                        stringify!($heap_type),
                        ".slice( $0, $1 ) );"
                    ),
                    slice_ptr,
                    (slice_ptr + slice.len() as $ptr_type)
                );

                let reference = unsafe {
                    Reference::from_raw_unchecked( raw )
                };

                TypedArray::from_reference( reference ).unwrap()
            }

            // This is unsafe due to the erasure of the slice's lifetime.
            unsafe fn into_typed_array_no_copy( slice: &[Self] ) -> TypedArray< Self > {
                let slice_ptr = (slice.as_ptr() as usize / size_of::<$element_type>()) as $ptr_type;
                let raw = __js_raw_asm!(
                    concat!(
                        "return Module.STDWEB.acquire_rust_reference( new $0( ",
                        stringify!($heap_type),
                        ".buffer, $1, $2 ) );"
                    ),
                    stringify!($js_array_type),
                    slice_ptr,
                    slice.len() as $ptr_type
                );

                let reference = Reference::from_raw_unchecked( raw );
                TypedArray::from_reference( reference ).unwrap()
            }

            fn from_typed_array( array: &TypedArray< Self > ) -> Vec< Self > {
                let length = array.len();
                let mut vector = Vec::with_capacity( length );
                let vec_ptr = (vector.as_ptr() as usize / size_of::<$element_type>()) as $ptr_type;

                js!( @(no_return)
                    var array = @{array};
                    var pointer = @{vec_ptr};
                    $heap_type.set( array, pointer );
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

arraykind!(i8, Int8Array, HEAP8, i32);
arraykind!(u8, Uint8Array, HEAPU8, i32);
arraykind!(i16, Int16Array, HEAP16, i32);
arraykind!(u16, Uint16Array, HEAPU16, i32);
arraykind!(i32, Int32Array, HEAP32, i32);
arraykind!(u32, Uint32Array, HEAPU32, i32);
arraykind!(f32, Float32Array, HEAPF32, i32);
arraykind!(f64, Float64Array, HEAPF64, i32);

/// JavaScript typed arrays are array-like objects and provide a mechanism for accessing raw binary data.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Typed_arrays)
pub struct TypedArray< T: ArrayKind >( Reference, PhantomData< T > );

reference_boilerplate! {
    impl< T > for TypedArray< T > where (T: ArrayKind)
}

impl< T: ArrayKind > FromReference for TypedArray< T > {
    #[inline]
    fn from_reference( reference: Reference ) -> Option< Self > {
        if T::is_typed_array( &reference ) {
            Some( TypedArray( reference, PhantomData ) )
        } else {
            None
        }
    }
}

impl< T: ArrayKind > TypedArray< T > {
    /// Returns the [TypedArray](struct.ArrayBuffer.html) referenced by this typed array.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/buffer)
    pub fn buffer( &self ) -> ArrayBuffer {
        js!( return @{self}.buffer; ).try_into().unwrap()
    }

    /// Returns the number of elements in the buffer.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/length)
    pub fn len( &self ) -> usize {
        let reference = self.as_ref();
        let length: i32 = js!( return @{reference}.length; ).try_into().unwrap();
        length as usize
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

#[cfg(test)]
mod tests {
    macro_rules! arraykind_test {
        ($element_type: ident, $js_array_type: ident) => {
            mod $element_type {
                use super::super::TypedArray;
                use std;
                use webcore::try_from::TryInto;
                use webcore::value::Value;
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
}

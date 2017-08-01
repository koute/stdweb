use std::marker::PhantomData;
use webcore::value::{Reference, FromReference};
use webcore::try_from::TryInto;
use webapi::array_buffer::ArrayBuffer;

pub trait ArrayKind: Sized {
    fn is_typed_array( reference: &Reference ) -> bool;
    fn into_typed_array( slice: &[Self] ) -> TypedArray< Self >;
    unsafe fn into_typed_array_no_copy( slice: &[Self] ) -> TypedArray< Self >;
    fn from_typed_array( array: &TypedArray< Self > ) -> Vec< Self >;
}

// TODO: Abstract this away in a macro and implement for all the types.
impl ArrayKind for u8 {
    fn is_typed_array( reference: &Reference ) -> bool {
        instanceof!( *reference, Uint8Array )
    }

    fn into_typed_array( slice: &[Self] ) -> TypedArray< Self > {
        let raw = em_asm_int!(
            "return Module.STDWEB.acquire_rust_reference( HEAPU8.slice( $0, $1 ) );",
            slice.as_ptr() as i32,
            (slice.as_ptr() as i32 + slice.len() as i32)
        );

        let reference = unsafe {
            Reference::from_raw_unchecked( raw )
        };

        TypedArray::from_reference( reference ).unwrap()
    }

    // This is unsafe due to the erasure of the slice's lifetime.
    unsafe fn into_typed_array_no_copy( slice: &[Self] ) -> TypedArray< Self > {
        let raw = em_asm_int!(
            "return Module.STDWEB.acquire_rust_reference( new Uint8Array( HEAPU8.buffer, $0, $1 ) );",
            slice.as_ptr() as i32,
            slice.len() as i32
        );

        let reference = Reference::from_raw_unchecked( raw );
        TypedArray::from_reference( reference ).unwrap()
    }

    fn from_typed_array( array: &TypedArray< Self > ) -> Vec< Self > {
        let length = array.len();
        let mut vector = Vec::with_capacity( length );

        js!( @(no_return)
            var array = @{array};
            var pointer = @{vector.as_ptr() as i32};
            HEAPU8.set( array, pointer );
        );

        unsafe {
            vector.set_len( length );
        }

        vector
    }
}

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

impl From< TypedArray< u8 > > for Vec< u8 > {
    fn from( array: TypedArray< u8 > ) -> Self {
        u8::from_typed_array( &array )
    }
}

impl< 'a > From< &'a TypedArray< u8 > > for Vec< u8 > {
    fn from( array: &'a TypedArray< u8 > ) -> Self {
        u8::from_typed_array( array )
    }
}

#[cfg(test)]
mod tests {
    use super::TypedArray;
    use webcore::value::Value;
    use webcore::try_from::TryInto;

    #[test]
    fn into() {
        let array: &[u8] = &[1, 2, 4];
        let typed_array: TypedArray< u8 > = array.into();
        assert_eq!( js!( return @{&typed_array} instanceof Uint8Array; ), Value::Bool( true ) );
        assert_eq!( js!( return @{&typed_array}.length === 3; ), Value::Bool( true ) );
        assert_eq!( js!( return @{&typed_array}[0] === 1; ), Value::Bool( true ) );
        assert_eq!( js!( return @{&typed_array}[1] === 2; ), Value::Bool( true ) );
        assert_eq!( js!( return @{&typed_array}[2] === 4; ), Value::Bool( true ) );
    }

    #[test]
    fn from() {
        let value = js!( return new Uint8Array( [1, 2, 4] ); );
        let typed_array: TypedArray< u8 > = value.try_into().unwrap();
        let vec: Vec< u8 > = typed_array.into();
        assert_eq!( vec.len(), 3 );
        assert_eq!( vec, &[1, 2, 4] );
    }
}

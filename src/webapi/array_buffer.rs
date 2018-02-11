use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::value::Value;
use webapi::typed_array::TypedArray;

/// The `ArrayBuffer` object is used to represent a generic, fixed-length raw binary data buffer.
/// You cannot directly manipulate the contents of an ArrayBuffer; instead, you create an [TypedArray](struct.TypedArray.html)
/// to do it.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ArrayBuffer")]
pub struct ArrayBuffer( Reference );

impl ArrayBuffer {
    /// Creates a new `ArrayBuffer` with the given length in bytes.
    pub fn new( length: usize ) -> Self {
        let length: Value = length.try_into().unwrap();
        js!( return new ArrayBuffer( @{length} ); ).try_into().unwrap()
    }

    /// Returns the length of the buffer, in bytes.
    pub fn len( &self ) -> usize {
        let reference = self.as_ref();
        let length: i32 = js!( return @{reference}.byteLength; ).try_into().unwrap();
        length as usize
    }
}

// TODO: Implement for other types.
// TODO: Implement slightly more efficiently than going through the TypedArray.
impl From< ArrayBuffer > for Vec< u8 > {
    fn from( buffer: ArrayBuffer ) -> Self {
        let typed_array: TypedArray< u8 > = buffer.into();
        typed_array.into()
    }
}

impl< 'a > From< &'a ArrayBuffer > for Vec< u8 > {
    fn from( buffer: &'a ArrayBuffer ) -> Self {
        let typed_array: TypedArray< u8 > = buffer.into();
        typed_array.into()
    }
}

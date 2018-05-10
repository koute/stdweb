use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::value::Value;
use webapi::typed_array::TypedArray;
use private::TODO;

/// The `ArrayBuffer` object is used to represent a generic, fixed-length raw binary data buffer.
/// You cannot directly manipulate the contents of an ArrayBuffer; instead, you create an [TypedArray](struct.TypedArray.html)
/// to do it.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)
// https://www.ecma-international.org/ecma-262/6.0/#sec-arraybuffer-constructor
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ArrayBuffer")]
pub struct ArrayBuffer( Reference );

impl ArrayBuffer {
    /// Creates a new `ArrayBuffer` with the given length in bytes.
    // https://www.ecma-international.org/ecma-262/6.0/#sec-arraybuffer-length
    pub fn new( length: u64 ) -> Result< Self, TODO > {
        let length: Value = length.try_into().unwrap();
        Ok( js!( return new ArrayBuffer( @{length} ); ).try_into().unwrap() )
    }

    /// Returns the length of the buffer, in bytes.
    // https://www.ecma-international.org/ecma-262/6.0/#sec-get-arraybuffer.prototype.bytelength
    pub fn len( &self ) -> u64 {
        let reference = self.as_ref();
        let length = js!( return @{reference}.byteLength; ).try_into().unwrap();
        length
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

use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::reference_type::ReferenceType;

/// A blob object represents a file-like object of immutable, raw data.
/// Blobs represent data that isn't necessarily in a JavaScript-native format.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
// https://w3c.github.io/FileAPI/#dfn-Blob
pub trait IBlob: ReferenceType {
    /// The size, in bytes, of the data contained in the Blob object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob/size)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-size%E2%91%A0
    fn len( &self ) -> u64 {
        let reference = self.as_ref();
        let length: u64 = js!( return @{reference}.size; ).try_into().unwrap();
        length
    }

    /// A string indicating the MIME type of the data contained in the `Blob`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob/type)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-type%E2%91%A0
    fn mime( &self ) -> Option< String > {
        let reference = self.as_ref();
        let mime: String = js!( return @{reference}.type; ).try_into().unwrap();
        if mime.is_empty() {
            None
        } else {
            Some( mime )
        }
    }

    /// Create a new `Blob` object containing the data in the specified range of bytes of the
    /// source `Blob`.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    /// https://w3c.github.io/FileAPI/#ref-for-dfn-slice
    fn slice( &self, start: Option<u64>, end: Option<u64>, content_type: Option<&str> ) -> Blob {
        let reference = self.as_ref();
        js! (
            return @{reference}.slice(
                @{start.map(|val| val as f64)}, 
                @{end.map(|val| val as f64)}, 
                @{content_type}
            );
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IBlob](trait.IBlob.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
// https://w3c.github.io/FileAPI/#dfn-Blob
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Blob")]
pub struct Blob( Reference );

impl IBlob for Blob {}

impl Blob {
    /// Creates a new `Blob`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)
    // https://w3c.github.io/FileAPI/#constructorBlob
    pub fn new() -> Self {
        js! (
            return new Blob();
        ).try_into().unwrap()
    }
}

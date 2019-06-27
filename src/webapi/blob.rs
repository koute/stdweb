use std::ops::{RangeBounds, Bound};

use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::reference_type::ReferenceType;
use webcore::number::Number;

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
    /// See also [slice_with_content_type](IBlob::slice_with_content_type).
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-slice
    fn slice< T >( &self, range: T ) -> Blob
        where T: RangeBounds<u64>
    {
        self._slice(range, None)
    }

    /// [slice](IBlob::slice) `Blob` with the provided `content_type`.
    fn slice_with_content_type< T >( &self, range: T, content_type: &str ) -> Blob
        where T: RangeBounds<u64>
    {
        self._slice(range, Some(content_type))
    }

    #[doc(hidden)]
    fn _slice< T >( &self, range: T, content_type: Option<&str> ) -> Blob
        where T: RangeBounds<u64>
    {
        let start: Option<Number> = match range.start_bound() {
            Bound::Included(&n) => Some(n),
            Bound::Excluded(&n) => Some(n + 1),
            _ => None
        }.try_into().unwrap();
        let end: Option<Number> = match range.end_bound() {
            Bound::Included(&n) => Some(n + 1),
            Bound::Excluded(&n) => Some(n),
            _ => None
        }.try_into().unwrap();
        let reference = self.as_ref();
        js! (
            return @{reference}.slice(@{start}, @{end}, @{content_type});
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
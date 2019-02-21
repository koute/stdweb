use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::blob::{IBlob, Blob};

/// The File interface provides information about files and allows JavaScript
/// in a web page to access their content.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/File)
// https://w3c.github.io/FileAPI/#dfn-file
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "File")]
#[reference(subclass_of(Blob))]
pub struct File( pub(crate) Reference );

impl IBlob for File {}

impl File {
    /// Returns the name of the file referenced by the `File` object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/File/name)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-name%E2%91%A0
    pub fn name( &self ) -> String {
        js!( return @{self}.name; ).try_into().unwrap()
    }
}

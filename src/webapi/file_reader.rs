use webcore::value::{Value, Reference};
use webcore::try_from::TryInto;
use webapi::blob::IBlob;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::array_buffer::ArrayBuffer;
use private::TODO;

/// The FileReader object lets web applications asynchronously read the contents of files
/// (or raw data buffers) stored on the user's computer, using [File](struct.File.html)
/// or [Blob](struct.Blob.html) objects to specify the file or data to read.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)
// https://w3c.github.io/FileAPI/#dfn-filereader
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "FileReader")]
#[reference(subclass_of(EventTarget))]
pub struct FileReader( Reference );

impl IEventTarget for FileReader {}

/// The [result](struct.FileReader.html#method.result) of a read operation performed with a [FileReader](struct.File.html).
#[derive(Clone, Debug)]
pub enum FileReaderResult {
    /// A string; a result of calling [FileReader::read_as_text](struct.FileReader.html#method.read_as_text).
    String( String ),

    /// An [ArrayBuffer](struct.ArrayBuffer.html); a result of calling [FileReader::read_as_array_buffer](struct.FileReader.html#method.read_as_array_buffer).
    ArrayBuffer( ArrayBuffer )
}

/// A number indicating the state of the `FileReader`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileReaderReadyState {
    /// No data has been loaded yet.
    Empty,
    /// Data is currently being loaded.
    Loading,
    /// The entire read request has been completed.
    Done
}

impl FileReader {
    /// Returns a newly constructed `FileReader`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/FileReader)
    // https://w3c.github.io/FileAPI/#dom-filereader-filereader
    pub fn new() -> FileReader {
        js!( return new FileReader(); ).try_into().unwrap()
    }

    /// Starts reading the contents of the specified blob. Once finished
    /// the `result` attribute will contain the contents of the file as a text string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-readAsText
    pub fn read_as_text< T: IBlob >( &self, blob: &T ) -> Result< (), TODO > {
        js!( @{self}.readAsText( @{blob.as_ref()} ); );
        Ok(())
    }

    /// Starts reading the contents of the specified blob. Once finished
    /// the `result` attribute will contain the contents of the file as an [TypedArray](struct.ArrayBuffer.html).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-readAsArrayBuffer
    pub fn read_as_array_buffer< T: IBlob >( &self, blob: &T ) -> Result< (), TODO > {
        js!( @{self}.readAsArrayBuffer( @{blob.as_ref()} ); );
        Ok(())
    }

    /// Aborts the read operation. Upon return, the `ready_state` will be `Done`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-abort%E2%91%A0
    pub fn abort( &self ) {
        js!( return @{self}.abort(); );
    }

    /// Returns the current state of the reader.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-readyState
    pub fn ready_state( &self ) -> FileReaderReadyState {
        let state: i32 = js!( return @{self}.readyState; ).try_into().unwrap();
        match state {
            0 => FileReaderReadyState::Empty,
            1 => FileReaderReadyState::Loading,
            2 => FileReaderReadyState::Done,
            _ => unreachable!( "Unexpected value of FileReader::readyState: {}", state )
        }
    }

    /// The file's contents. This method will only return a value after the read operation
    /// is complete, and the format of the data depends on which of the methods was used
    /// to initiate the read operation.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-result
    pub fn result( &self ) -> Option< FileReaderResult > {
        let result = js!( return @{self}.result; );
        match result {
            Value::Undefined | Value::Null => None,
            Value::String( text ) => Some( FileReaderResult::String( text ) ),
            Value::Reference( reference ) => Some( FileReaderResult::ArrayBuffer( reference.try_into().unwrap() ) ),
            _ => unreachable!( "Unexpected result of a FileReader: {:?}", result )
        }
    }
}

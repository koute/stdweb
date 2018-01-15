use webcore::value::{Value, Reference};
use webcore::try_from::TryInto;
use webapi::blob::IBlob;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::array_buffer::ArrayBuffer;

/// The FileReader object lets web applications asynchronously read the contents of files
/// (or raw data buffers) stored on the user's computer, using [File](struct.File.html)
/// or [Blob](struct.Blob.html) objects to specify the file or data to read.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)
pub struct FileReader( Reference );

impl IEventTarget for FileReader {}

reference_boilerplate! {
    FileReader,
    instanceof FileReader
    convertible to EventTarget
}

/// The [result](struct.FileReader.html#method.result) of a read operation performed with a [FileReader](struct.File.html).
#[derive(Clone, Debug)]
pub enum FileReaderResult {
    /// A string; a result of calling [FileReader::read_as_text](struct.FileReader.html#method.read_as_text).
    String( String ),

    /// An [ArrayBuffer](struct.ArrayBuffer.html); a result of calling [FileReader::read_as_text](struct.FileReader.html#method.read_as_array_buffer).
    ArrayBuffer( ArrayBuffer )
}

/// A number indicating the state of the `FileReader`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FileReaderReadyState {
    Empty,
    Loading,
    Done
}

impl FileReader {
    /// Returns a newly constructed `FileReader`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/FileReader)
    pub fn new() -> FileReader {
        js!( return new FileReader(); ).try_into().unwrap()
    }

    /// Starts reading the contents of the specified blob. Once finished
    /// the `result` attribute will contain the contents of the file as a text string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)
    pub fn read_as_text< T: IBlob >( &self, blob: &T ) {
        js!( @{self}.readAsText( @{blob.as_ref()} ); );
    }

    /// Starts reading the contents of the specified blob. Once finished
    /// the `result` attribute will contain the contents of the file as an [TypedArray](struct.ArrayBuffer.html).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)
    pub fn read_as_array_buffer< T: IBlob >( &self, blob: &T ) {
        js!( @{self}.readAsArrayBuffer( @{blob.as_ref()} ); );
    }

    /// Aborts the read operation. Upon return, the `ready_state` will be `Done`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)
    pub fn abort( &self ) {
        js!( return @{self}.abort(); );
    }

    /// Returns the current state of the reader.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)
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

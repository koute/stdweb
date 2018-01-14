use webcore::value::{Value, Reference, ConversionError};
use webcore::try_from::{TryFrom, TryInto};
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::event::CloseEventCode;
use webapi::blob::Blob;
use webapi::array_buffer::ArrayBuffer;
use webapi::typed_array::TypedArray;

/// The WebSocket object provides the API for creating and managing a WebSocket connection to a
/// server, as well as for sending and receiving data on the connection.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
pub struct WebSocket( Reference );

impl IEventTarget for WebSocket {}

reference_boilerplate! {
    WebSocket,
    instanceof WebSocket
    convertible to EventTarget
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryType {
    Blob,
    ArrayBuffer
}

impl BinaryType {
    fn to_str(self) -> &'static str {
        match self {
            BinaryType::Blob => "blob",
            BinaryType::ArrayBuffer => "arraybuffer",
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "blob" => BinaryType::Blob,
            "arraybuffer" => BinaryType::ArrayBuffer,
            other => panic!("Invalid binary type: {:?}", other)
        }
    }
}

/// A number indicating the state of the `WebSocket`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReadyState {
    Connecting = 0,
    Open = 1,
    Closing = 2,
    Closed = 3
}

impl TryFrom<Value> for ReadyState {
    type Error = ConversionError;

    /// Performs the conversion.
    fn try_from(v: Value) -> Result<ReadyState, ConversionError> {
        match v.try_into()? {
            0 => Ok(ReadyState::Connecting),
            1 => Ok(ReadyState::Open),
            2 => Ok(ReadyState::Closing),
            3 => Ok(ReadyState::Closed),
            other => Err(ConversionError::Custom(format!("Unknown ready_state: {}", other)))
        }
    }
}

pub trait AsBinaryRef {
    fn as_binary_ref(&self) -> Reference;
}

impl AsBinaryRef for Blob {
    fn as_binary_ref(&self) -> Reference { self.as_ref().clone() }
}

impl AsBinaryRef for ArrayBuffer {
    fn as_binary_ref(&self) -> Reference { self.as_ref().clone() }
}

impl AsBinaryRef for [u8] {
    fn as_binary_ref(&self) -> Reference { TypedArray::from(self).as_ref().clone() }
}

impl WebSocket {
    /// Returns a newly constructed `WebSocket`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn new(url: &str) -> WebSocket {
        js!( return new WebSocket(@{url}); ).try_into().unwrap()
    }

    /// Returns a newly constructed `WebSocket`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn new_with_protocols(url: &str, protocols: &[&str]) -> WebSocket {
        js!( return new WebSocket(@{url}, @{protocols}); ).try_into().unwrap()
    }

    /// Returns the binary type of the web socket. Only affects received messages.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn binary_type(&self) -> BinaryType {
        let binary_type: String = js!( return @{self}.binaryType; ).try_into().unwrap();
        BinaryType::from_str(&binary_type)
    }

    /// Sets the binary type of the web socket. Only affects received messages.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn set_binary_type(&self, binary_type: BinaryType) {
        js!( @(no_return) @{self}.binaryType = @{binary_type.to_str()}; );
    }

    /// Returns the number of bytes of data that have been queued using calls to send()
    /// but not yet transmitted to the network. This value resets to zero once all queued
    /// data has been sent. This value does not reset to zero when the connection is closed;
    /// if you keep calling send(), this will continue to climb. 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn buffered_amount(&self) -> usize {
        js!( return @{self}.bufferedAmount; ).try_into().unwrap()
    }

    /// Returns the extensions selected by the server. This is currently only the empty
    /// string or a list of extensions as negotiated by the connection.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn extensions(&self) -> String {
        js!( return @{self}.extensions; ).try_into().unwrap()
    }

    /// Returns a string indicating the name of the sub-protocol the server selected;
    /// this will be one of the strings specified in the protocols parameter when
    /// creating the WebSocket object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn protocol(&self) -> String {
        js!( return @{self}.protocol; ).try_into().unwrap()
    }

    /// Returns a string indicating the name of the sub-protocol the server selected;
    /// this will be one of the strings specified in the protocols parameter when
    /// creating the WebSocket object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn ready_state(&self) -> ReadyState {
        js!( return @{self}.protocol; ).try_into().unwrap()
    }

    /// Returns the URL as resolved by the constructor. This is always an absolute URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    pub fn url(&self) -> String {
        js!( return @{self}.url; ).try_into().unwrap()
    }

    /// Closes the WebSocket connection or connection attempt, if any. If the connection
    /// is already CLOSED, this method does nothing.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#close())
    pub fn close(&self) {
        js!( @(no_return) @{self}.close(); );
    }

    /// Closes the WebSocket connection or connection attempt, if any. If the connection
    /// is already CLOSED, this method does nothing.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#close())
    pub fn close_with_status(&self, code: CloseEventCode, reason: &str) {
        let code = code.0;
        js!( @(no_return) @{self}.close(@{code}, @{reason}); );
    }

    /// Enqueues the specified data to be transmitted to the server over the WebSocket
    /// connection, increasing the value of bufferedAmount by the number of bytes needed
    /// to contain the data. If the data can't be sent (for example, because it needs to
    /// be buffered but the buffer is full), the socket is closed automatically.
    pub fn send_text(&self, text: &str) {
        js!( @(no_return) @{self}.send(@{text}); );
    }

    fn _send_binary(&self, binary: Reference) {
        js!( @(no_return) @{self}.send(@{binary}); );
    }

    /// Enqueues the specified data to be transmitted to the server over the WebSocket
    /// connection, increasing the value of bufferedAmount by the number of bytes needed
    /// to contain the data. If the data can't be sent (for example, because it needs to
    /// be buffered but the buffer is full), the socket is closed automatically.
    pub fn send_binary<T: AsBinaryRef>(&self, binary: &T) {
        self._send_binary(binary.as_binary_ref());
    }
}


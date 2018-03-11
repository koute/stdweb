use webcore::value::{Value, Reference, ConversionError};
use webcore::try_from::{TryFrom, TryInto};
use webcore::unsafe_typed_array::UnsafeTypedArray;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::blob::Blob;
use webapi::array_buffer::ArrayBuffer;
use webapi::dom_exception::{InvalidAccessError, SecurityError, SyntaxError};
use private::TODO;

/// Wrapper type around a CloseEvent code, indicating why the WebSocket was closed
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketCloseCode(pub u16);

// Close codes are defined here:
// https://tools.ietf.org/html/rfc6455#section-7.4
newtype_enum!(SocketCloseCode {
    /// Normal closure; the connection successfully completed whatever purpose for which it was
    /// created.
    NORMAL_CLOSURE = 1000,
    /// The endpoint is going away, either because of a server failure or because the browser is
    /// navigating away from the page that opened the connection.
    GOING_AWAY = 1001,
    /// The endpoint is terminating the connection due to a protocol error.
    PROTOCOL_ERROR = 1002,
    /// The connection is being terminated because the endpoint received data of a type it cannot
    /// accept (for example, a text-only endpoint received binary data).
    UNSUPPORTED_DATA = 1003,
    /// Reserved. Indicates that no status code was provided even though one was expected.
    NO_STATUS_RECEIVED = 1005,
    /// Reserved. Used to indicate that a connection was closed abnormally (that is, with no close
    /// frame being sent) when a status code is expected.
    ABNORMAL_CLOSURE = 1006,
    /// The endpoint is terminating the connection because a message was received that contained
    /// inconsistent data (e.g., non-UTF-8 data within a text message).
    INVALID_FRAME_PAYLOAD_DATA = 1007,
    /// The endpoint is terminating the connection because it received a message that violates its
    /// policy. This is a generic status code, used when codes 1003 and 1009 are not suitable.
    POLICY_VIOLATION = 1008,
    /// The endpoint is terminating the connection because a data frame was received that is too
    /// large.
    MESSAGE_TOO_BIG = 1009,
    /// The client is terminating the connection because it expected the server to negotiate one or
    /// more extensions, but the server didn't.
    MISSING_EXTENSION = 1010,
    /// The server is terminating the connection because it encountered an unexpected condition
    /// that prevented it from fulfilling the request.
    INTERNAL_ERROR = 1011,
    /// The server is terminating the connection because it is restarting.
    SERVICE_RESTART = 1012,
    /// The server is terminating the connection due to a temporary condition, e.g. it is
    /// overloaded and is casting off some of its clients.
    TRY_AGAIN_LATER = 1013,
    /// The server was acting as a gateway or proxy and received an invalid response from the
    /// upstream server. This is similar to 502 HTTP Status Code.
    BAD_GATEWAY = 1014,
    /// Reserved. Indicates that the connection was closed due to a failure to perform a TLS
    /// handshake (e.g., the server certificate can't be verified).
    TLS_HANDSHAKE = 1015,
});

/// The WebSocket object provides the API for creating and managing a WebSocket connection to a
/// server, as well as for sending and receiving data on the connection.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
// https://html.spec.whatwg.org/#websocket
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "WebSocket")]
#[reference(subclass_of(EventTarget))]
pub struct WebSocket( Reference );

impl IEventTarget for WebSocket {}

/// The type of binary data being transmitted by the WebSocket connection.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#Attributes)
// https://html.spec.whatwg.org/#dom-websocket-binarytype
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketBinaryType {
    /// A Blob object represents a file-like object of immutable, raw data.
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
    Blob,
    /// The ArrayBuffer object is used to represent a generic, fixed-length raw binary data buffer.
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)
    ArrayBuffer
}

impl SocketBinaryType {
    fn to_str(self) -> &'static str {
        match self {
            SocketBinaryType::Blob => "blob",
            SocketBinaryType::ArrayBuffer => "arraybuffer",
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "blob" => SocketBinaryType::Blob,
            "arraybuffer" => SocketBinaryType::ArrayBuffer,
            other => panic!("Invalid binary type: {:?}", other)
        }
    }
}

/// A number indicating the state of the `WebSocket`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#Ready_state_constants)
// https://html.spec.whatwg.org/#dom-websocket-readystate
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketReadyState {
    Connecting = 0,
    Open = 1,
    Closing = 2,
    Closed = 3
}

impl TryFrom<Value> for SocketReadyState {
    type Error = ConversionError;

    /// Performs the conversion.
    fn try_from(v: Value) -> Result<SocketReadyState, ConversionError> {
        match v.try_into()? {
            0 => Ok(SocketReadyState::Connecting),
            1 => Ok(SocketReadyState::Open),
            2 => Ok(SocketReadyState::Closing),
            3 => Ok(SocketReadyState::Closed),
            other => Err(ConversionError::Custom(format!("Unknown ready_state: {}", other)))
        }
    }
}

impl WebSocket {
    /// Returns a newly constructed `WebSocket`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket
    pub fn new(url: &str) -> Result<WebSocket, CreationError> {
        js_try!(
            return new WebSocket(@{url});
        ).unwrap()
    }

    /// Returns a newly constructed `WebSocket`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket
    pub fn new_with_protocols(url: &str, protocols: &[&str]) -> Result<WebSocket, CreationError> {
        js_try!(
            return new WebSocket(@{url}, @{protocols});
        ).unwrap()
    }

    /// Returns the binary type of the web socket. Only affects received messages.
    /// The default binary type is `Blob`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-binarytype
    pub fn binary_type(&self) -> SocketBinaryType {
        let binary_type: String = js!( return @{self}.binaryType; ).try_into().unwrap();
        SocketBinaryType::from_str(&binary_type)
    }

    /// Sets the binary type of the web socket. Only affects received messages.
    /// The default binary type is `Blob`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-binarytype
    pub fn set_binary_type(&self, binary_type: SocketBinaryType) {
        js!( @(no_return) @{self}.binaryType = @{binary_type.to_str()}; );
    }

    /// Returns the number of bytes of data that have been queued using calls to send()
    /// but not yet transmitted to the network. This value resets to zero once all queued
    /// data has been sent. This value does not reset to zero when the connection is closed;
    /// if you keep calling send(), this will continue to climb.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-bufferedamount
    pub fn buffered_amount(&self) -> u64 {
        js!( return @{self}.bufferedAmount; ).try_into().unwrap()
    }

    /// Returns the extensions selected by the server. This is currently only the empty
    /// string or a list of extensions as negotiated by the connection.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-extensions
    pub fn extensions(&self) -> String {
        js!( return @{self}.extensions; ).try_into().unwrap()
    }

    /// Returns a string indicating the name of the sub-protocol the server selected;
    /// this will be one of the strings specified in the protocols parameter when
    /// creating the WebSocket object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-protocol
    pub fn protocol(&self) -> String {
        js!( return @{self}.protocol; ).try_into().unwrap()
    }

    /// Returns the current state of the connection.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-readystate
    pub fn ready_state(&self) -> SocketReadyState {
        js!( return @{self}.readyState; ).try_into().unwrap()
    }

    /// Returns the URL as resolved by the constructor. This is always an absolute URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-url
    pub fn url(&self) -> String {
        js!( return @{self}.url; ).try_into().unwrap()
    }

    /// Closes the WebSocket connection or connection attempt, if any. If the connection
    /// is already CLOSED, this method does nothing.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#close())
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-close
    pub fn close(&self) {
        js!( @(no_return) @{self}.close(); );
    }

    /// Closes the WebSocket connection or connection attempt, if any. If the connection
    /// is already CLOSED, this method does nothing.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket#close())
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-close
    pub fn close_with_status(&self, code: SocketCloseCode, reason: &str) -> Result<(), CloseError> {
        js_try!( @(no_return)
            @{self}.close(@{code.0}, @{reason});
        ).unwrap()
    }

    /// Enqueues the specified data to be transmitted to the server over the WebSocket
    /// connection, increasing the value of bufferedAmount by the number of bytes needed
    /// to contain the data. If the data can't be sent (for example, because it needs to
    /// be buffered but the buffer is full), the socket is closed automatically.
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-send
    pub fn send_text(&self, text: &str) -> Result< (), TODO > {
        js!( @(no_return) @{self}.send(@{text}); );
        Ok(())
    }

    /// Enqueues the specified data to be transmitted to the server over the WebSocket
    /// connection, increasing the value of bufferedAmount by the number of bytes needed
    /// to contain the data. If the data can't be sent (for example, because it needs to
    /// be buffered but the buffer is full), the socket is closed automatically.
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-send
    pub fn send_blob(&self, blob: &Blob) -> Result< (), TODO > {
        js!( @(no_return) @{self}.send(@{blob}); );
        Ok(())
    }

    /// Enqueues the specified data to be transmitted to the server over the WebSocket
    /// connection, increasing the value of bufferedAmount by the number of bytes needed
    /// to contain the data. If the data can't be sent (for example, because it needs to
    /// be buffered but the buffer is full), the socket is closed automatically.
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-send
    pub fn send_array_buffer(&self, array_buffer: &ArrayBuffer) -> Result< (), TODO > {
        js!( @(no_return) @{self}.send(@{array_buffer}); );
        Ok(())
    }

    /// Enqueues the specified data to be transmitted to the server over the WebSocket
    /// connection, increasing the value of bufferedAmount by the number of bytes needed
    /// to contain the data. If the data can't be sent (for example, because it needs to
    /// be buffered but the buffer is full), the socket is closed automatically.
    // https://html.spec.whatwg.org/#the-websocket-interface:dom-websocket-send
    pub fn send_bytes(&self, bytes: &[u8]) -> Result< (), TODO > {
        js!( @(no_return) @{self}.send(@{ UnsafeTypedArray(bytes) }); );
        Ok(())
    }
}

/// Errors thrown by `WebSocket::new`.
error_enum_boilerplate! {
    CreationError,
    SecurityError, SyntaxError
}

/// Errors thrown by `WebSocket::close_with_status`.
error_enum_boilerplate! {
    CloseError,
    InvalidAccessError, SyntaxError
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_close_codes() {
        assert_eq!(&format!("{:?}", SocketCloseCode::NORMAL_CLOSURE), "SocketCloseCode::NORMAL_CLOSURE");
        assert_eq!(&format!("{:?}", SocketCloseCode::GOING_AWAY), "SocketCloseCode::GOING_AWAY");
        assert_eq!(&format!("{:?}", SocketCloseCode(1000)), "SocketCloseCode::NORMAL_CLOSURE");
        assert_eq!(&format!("{:?}", SocketCloseCode(3001)), "SocketCloseCode(3001)");
    }
}

#[cfg(all(test, feature = "web_test"))]
mod web_tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(WebSocket::new("ws://localhost").is_ok());

        match WebSocket::new("bad url") {
            Err(CreationError::SyntaxError(_)) => (),
            v => panic!("expected SyntaxError, got {:?}", v),
        }
    }

    #[test]
    fn test_close() {
        let socket = WebSocket::new("ws://localhost").unwrap();

        socket.close();

        assert!(socket.close_with_status( SocketCloseCode::NORMAL_CLOSURE, "closed" ).is_ok());

        // Invalid close code
        match socket.close_with_status( SocketCloseCode(12345), "closed" ) {
            Err(CloseError::InvalidAccessError(_)) => (),
            v => panic!("expected InvalidAccessError, got {:?}", v),
        }

        // Close reason too long (>123 bytes according to spec)
        match socket.close_with_status(
            SocketCloseCode::NORMAL_CLOSURE,
            &(0..200).map(|_| "X").collect::<String>()
        ) {
            Err(CloseError::SyntaxError(_)) => (),
            v => panic!("expected SyntaxError, got {:?}", v),
        }
    }
}

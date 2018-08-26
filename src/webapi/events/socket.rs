use std::fmt::Debug;

use webcore::value::{Reference, Value, ConversionError};
use webcore::try_from::{TryFrom, TryInto};
use webapi::blob::Blob;
use webapi::array_buffer::ArrayBuffer;
use webapi::web_socket::SocketCloseCode;
use webapi::event::{IEvent, Event};

/// A SocketCloseEvent is sent to clients using WebSockets when the connection is closed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/close)
// https://html.spec.whatwg.org/multipage/web-sockets.html#closeevent
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "CloseEvent")]
#[reference(event = "close")]
#[reference(subclass_of(Event))]
pub struct SocketCloseEvent( Reference );

impl SocketCloseEvent {
    /// Returns the close code sent by the server.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code)
    // https://html.spec.whatwg.org/multipage/web-sockets.html#the-closeevent-interface:dom-closeevent-code
    #[inline]
    pub fn code( &self ) -> SocketCloseCode {
        SocketCloseCode(js!(
            return @{self.as_ref()}.code;
        ).try_into().unwrap())
    }

    /// Returns the reason the server closed the connection. This is specific to the particular server and sub-protocol.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/reason)
    // https://html.spec.whatwg.org/multipage/web-sockets.html#the-closeevent-interface:dom-closeevent-reason
    #[inline]
    pub fn reason( &self ) -> String {
        js!(
            return @{self.as_ref()}.reason;
        ).try_into().unwrap()
    }

    /// Returns whether or not the connection was cleanly closed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/wasClean)
    // https://html.spec.whatwg.org/multipage/web-sockets.html#the-closeevent-interface:dom-closeevent-wasclean
    #[inline]
    pub fn was_clean( &self ) -> bool {
        js!(
            return @{self.as_ref()}.wasClean;
        ).try_into().unwrap()
    }
}

impl IEvent for SocketCloseEvent {}

/// The error event is fired when an error occurred; the exact circumstances vary,
/// events by this name are used from a variety of APIs.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/error)
// https://html.spec.whatwg.org/#event-error
// https://html.spec.whatwg.org/multipage/web-sockets.html#handler-websocket-onerror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(event = "error")]
#[reference(subclass_of(Event))]
pub struct SocketErrorEvent( Reference );

impl IEvent for SocketErrorEvent {}

/// An open event informs the target that a data connection, has been established.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/open)
// https://html.spec.whatwg.org/#event-open
// https://html.spec.whatwg.org/multipage/web-sockets.html#handler-websocket-onopen
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(event = "open")]
#[reference(subclass_of(Event))]
pub struct SocketOpenEvent( Reference );

impl IEvent for SocketOpenEvent {}

/// Represents the types of data which can be received on a web socket. Messages
/// are transmitted tagged as either binary or text: text messages are always
/// received as strings. Binary messages may be received as either blobs or array
/// buffers as preferred by the receiver. This choice is indicated via the
/// `binary_type` field on the web socket.
#[derive(Debug, Clone)]
pub enum SocketMessageData {
    /// Text message
    Text(String),
    /// Binary message received as a blob
    Blob(Blob),
    /// Binary message received as an array buffer
    ArrayBuffer(ArrayBuffer),
}

impl SocketMessageData {
    /// Try to receive the message as text
    pub fn into_text(self) -> Option<String> {
        if let SocketMessageData::Text(s) = self { Some(s) } else { None }
    }
    /// Try to receive the message as a binary blob
    pub fn into_blob(self) -> Option<Blob> {
        if let SocketMessageData::Blob(b) = self { Some(b) } else { None }
    }
    /// Try to receive the message as an array buffer
    pub fn into_array_buffer(self) -> Option<ArrayBuffer> {
        if let SocketMessageData::ArrayBuffer(b) = self { Some(b) } else { None }
    }
}

impl TryFrom<Value> for SocketMessageData {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<SocketMessageData, ConversionError> {
        match v {
            Value::String(s) => Ok(SocketMessageData::Text(s)),
            Value::Reference(ref r) => {
                if let Ok(b) = r.clone().try_into() {
                    Ok(SocketMessageData::Blob(b))
                } else if let Ok(b) = r.clone().try_into() {
                    Ok(SocketMessageData::ArrayBuffer(b))
                } else {
                    Err(ConversionError::Custom(format!("Unknown message event data: {:?}", r)))
                }
            },
            other => Err(ConversionError::Custom(format!("Unknown message event data: {:?}", other)))
        }
    }
}

/// The MessageEvent interface represents a message received by a target object.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)
// https://html.spec.whatwg.org/multipage/comms.html#messageevent
pub trait IMessageEvent: IEvent where <Self::Data as TryFrom<Value>>::Error: Debug {
    /// The type of data received with this MessageEvent
    type Data: TryFrom<Value>;

    /// The data sent by the message emitter.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data)
    // https://html.spec.whatwg.org/multipage/comms.html#the-messageevent-interface:dom-messageevent-data
    #[inline]
    fn data( &self ) -> Self::Data {
        js!(
            return @{self.as_ref()}.data;
        ).try_into().unwrap()
    }

    /// A string representing the origin of the message emitter.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/origin)
    // https://html.spec.whatwg.org/multipage/comms.html#the-messageevent-interface:dom-messageevent-origin
    #[inline]
    fn origin( &self ) -> String {
        js!(
            return @{self.as_ref()}.origin;
        ).try_into().unwrap()
    }

    /// A string representing a unique ID for the event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId)
    // https://html.spec.whatwg.org/multipage/comms.html#the-messageevent-interface:dom-messageevent-lasteventid
    #[inline]
    fn last_event_id( &self ) -> String {
        js!(
            return @{self.as_ref()}.lastEventId;
        ).try_into().unwrap()
    }

    /// A MessageEventSource (which can be a WindowProxy, MessagePort, or ServiceWorker object)
    /// representing the message emitter.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/source)
    // https://html.spec.whatwg.org/multipage/comms.html#the-messageevent-interface:dom-messageevent-source
    #[inline]
    fn source( &self ) -> Option<Reference> {
        js!(
            return @{self.as_ref()}.source;
        ).try_into().ok()
    }

    /// An array of MessagePort objects representing the ports associated with the channel the
    /// message is being sent through (where appropriate, e.g. in channel messaging or when sending
    /// a message to a shared worker).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/ports)
    // https://html.spec.whatwg.org/multipage/comms.html#the-messageevent-interface:dom-messageevent-ports
    #[inline]
    fn ports( &self ) -> Vec<Reference> {
        js!(
            return @{self.as_ref()}.ports;
        ).try_into().unwrap()
    }
}

/// A message event informs a WebSocket object that a message has been received.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/message)
// https://html.spec.whatwg.org/#event-message
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MessageEvent")]
#[reference(event = "message")]
#[reference(subclass_of(Event))]
pub struct SocketMessageEvent( Reference );

// https://html.spec.whatwg.org/multipage/web-sockets.html#handler-websocket-onmessage
impl IMessageEvent for SocketMessageEvent {
    type Data = SocketMessageData;
}

impl IEvent for SocketMessageEvent {}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::event::ConcreteEvent;

    #[test]
    fn test_close_event() {
        let event: SocketCloseEvent = js!(
            return new CloseEvent(
                @{SocketCloseEvent::EVENT_TYPE},
                {
                    code: 1000,
                    reason: "WebSocket was closed normally",
                    wasClean: true
                }
            );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), SocketCloseEvent::EVENT_TYPE );
        assert_eq!( event.code(), SocketCloseCode::NORMAL_CLOSURE );
        assert_eq!( event.reason(), "WebSocket was closed normally" );
        assert!( event.was_clean() );
    }
}

use webapi::event_target::{IEventTarget, EventTarget};
use webcore::unsafe_typed_array::UnsafeTypedArray;
use webcore::value::{
    Reference,
    Value,
};
use webcore::try_from::TryInto;

/// Use XMLHttpRequest (XHR) objects to interact with servers.
/// You can retrieve data from a URL without having to do a full page refresh.
/// This enables a Web page to update just part of a page without disrupting
/// what the user is doing. XMLHttpRequest is used heavily in Ajax programming.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest)
pub struct XMLHttpRequest( Reference );

reference_boilerplate! {
    XMLHttpRequest,
    instanceof XMLHttpRequest
    convertible to EventTarget
}

/// An enum indicating the state of the `XMLHttpRequest`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum XHRReadyState {
    /// Client has been created. [open()](struct.XMLHttpRequest.html#method.open) not called yet.
    Unsent,
    /// [open()](struct.XMLHttpRequest.html#method.open) has been called.
    Opened,
    /// [send()](struct.XMLHttpRequest.html#method.send) has been called, and headers and [status()](struct.XMLHttpRequest.html#method.status) are available.
    HeadersReceived,
    /// Downloading; [reponse_text()](struct.XMLHttpRequest.html#method.reponse_text) holds partial data.
    Loading,
    /// The operation is complete.
    Done,
}

impl IEventTarget for XMLHttpRequest {}


impl XMLHttpRequest {
    /// Creates new `XMLHttpRequest`.
    pub fn new() -> XMLHttpRequest {
        js!( return new XMLHttpRequest(); ).try_into().unwrap()
    }

    /// Returns the current state of the request as a [XHRReadyState](enum.XHRReadyState.html).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)
    pub fn ready_state(&self) -> XHRReadyState {
        use self::XHRReadyState::*;
        let state: u16 = js!( return @{self}.readyState; ).try_into().unwrap();
        match state {
            0 => Unsent,
            1 => Opened,
            2 => HeadersReceived,
            3 => Loading,
            4 => Done,
            _ => unreachable!( "Unexpected value of XMLHttpRequest::readyState: {}", state )
        }
    }

    /// Returns a string that contains the response to the request as text, or None
    /// if the request was unsuccessful or has not yet been sent.
    ///
    ///[(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseText)
    pub fn response_text(&self) -> Option<String> {
        let response = js!(return @{self}.responseText;);
        match response {
            Value::Null => None,
            Value::String(resp) => Some(resp),
            _ => unreachable!(),
        }
    }

    /// Returns an unsigned short with the status of the response of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/status)
    pub fn status(&self) -> u16 {
        js!(return @{self}.status;).try_into().unwrap()
    }

    /// Open connection with given method (ie GET or POST), and the url to hit.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)
    pub fn open(&self, method: &str, url: &str) {
        js! { @(no_return)
            @{self}.open(@{method}, @{url}, true);
        };
    }

    /// Returns the string containing the text of the specified header. If there
    /// are multiple response headers with the same name, then their values are
    /// returned as a single concatenated string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getResponseHeader)
    pub fn get_response_header(&self, header: &str) -> Option<String> {
        let header = js!( return @{self}.getResponseHeader(@{header}); );
        match header {
            Value::Null => None,
            Value::String(text) => Some(text),
            _ => unreachable!(),
        }
    }

    /// Sets the value of an HTTP request header. Must be called after `open()`,
    /// but before `send()`. If this method is called several times with the same
    /// header, the values are merged into one single request header.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setRequestHeader)
    pub fn set_request_header(&self, header: &str, value: &str) {
        // TODO: Handle InvalidStateError and SyntaxError exceptions.
        // https://xhr.spec.whatwg.org/#the-setrequestheader()-method
        js! { @(no_return)
            @{self}.setRequestHeader(@{header}, @{value});
        };
    }

    /// Send request on an open connection with no data
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send(&self) {
        js! { @(no_return)
            @{self}.send();
        };
    }

    /// Send request on an open connection with string body
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send_with_string(&self, body: &str) {
        js! { @(no_return)
            @{self}.send(@{body});
        };
    }

    /// Send request on an open connection with a byte array body
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send_with_bytes(&self, body: &[u8]) {
        js! { @(no_return)
            @{self}.send(@{UnsafeTypedArray(body)});
        };
    }

    /// Aborts the request if it has already been sent.
    /// When a request is aborted, its [ready_state](struct.XMLHttpRequest.html#method.ready_state) is changed to [Done](enum.XHRReadyState.html#variant.Done)
    /// and the [status](struct.XMLHttpRequest.html#method.status) code is set to
    /// [Unsent](enum.XHRReadyState.html#variant.Unsent).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/abort)
    pub fn abort(&self) {
        js! { @(no_return)
            @{self}.abort();
        };
    }
}

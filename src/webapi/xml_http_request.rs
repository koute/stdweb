use webapi::event_target::{IEventTarget, EventTarget};
use webapi::dom_exception::{InvalidAccessError, InvalidStateError};
use webcore::unsafe_typed_array::UnsafeTypedArray;
use webcore::value::{
    Reference,
    Value,
};
use webcore::try_from::{TryFrom, TryInto};
use private::TODO;

/// Use XmlHttpRequest (XHR) objects to interact with servers.
/// You can retrieve data from a URL without having to do a full page refresh.
/// This enables a Web page to update just part of a page without disrupting
/// what the user is doing. XmlHttpRequest is used heavily in Ajax programming.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XmlHttpRequest)
// https://xhr.spec.whatwg.org/#xmlhttprequest
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "XMLHttpRequest")]
#[reference(subclass_of(EventTarget))]
pub struct XmlHttpRequest( Reference );

/// An enum indicating the state of the `XmlHttpRequest`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)
// https://xhr.spec.whatwg.org/#dom-xmlhttprequest-readystate
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum XhrReadyState {
    /// Client has been created. [open()](struct.XmlHttpRequest.html#method.open) not called yet.
    Unsent,
    /// [open()](struct.XmlHttpRequest.html#method.open) has been called.
    Opened,
    /// [send()](struct.XmlHttpRequest.html#method.send) has been called, and headers and [status()](struct.XmlHttpRequest.html#method.status) are available.
    HeadersReceived,
    /// Downloading; [reponse_text()](struct.XmlHttpRequest.html#method.reponse_text) holds partial data.
    Loading,
    /// The operation is complete.
    Done,
}

/// An enum describing the type of the response to `XmlHttpRequest`
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)
// https://xhr.spec.whatwg.org/#dom-xmlhttprequest-responsetype
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum XhrResponseType {
    /// A JavaScript ArrayBuffer containing binary data
    ArrayBuffer,
    /// A Blob object container the binary data
    Blob,
    /// An HTML Document or XML XMLDocument
    Document,
    /// A JavaScript object parsed from JSON
    Json,
    /// Text in a String object
    Text
}

impl IEventTarget for XmlHttpRequest {}

error_enum_boilerplate! {
    /// An error returned from `XmlHttpRequest::set_response_type`
    XhrSetResponseTypeError,

    #[allow(missing_docs)]
    InvalidStateError,
    #[allow(missing_docs)]
    InvalidAccessError
}

impl XmlHttpRequest {
    /// Creates new `XmlHttpRequest`.
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest
    pub fn new() -> XmlHttpRequest {
        js!( return new XMLHttpRequest(); ).try_into().unwrap()
    }

    /// Returns the current state of the request as a [XhrReadyState](enum.XhrReadyState.html).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-readystate
    pub fn ready_state(&self) -> XhrReadyState {
        use self::XhrReadyState::*;
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

    /// Returns the type of the request as a [XhrResponseType](enum.XhrResponseType.html)
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-responsetype
    pub fn response_type(&self) -> XhrResponseType {
        use self::XhrResponseType::*;
        let repsonse_type: String = js! ( return @{self}.responseType; ).try_into().unwrap();
        match repsonse_type.as_str() {
            "arraybuffer" => ArrayBuffer,
            "blob" => Blob,
            "document" => Document,
            "json" => Json,
            "text" | "" => Text,
            x => unreachable!( "Unexpected value of XMLHttpRequest::responseType:: {}", x)
        }
    }

    /// Set the type that the XmlHttpRequest should return
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-responsetype
    pub fn set_response_type(&self, kind: XhrResponseType) -> Result<(), XhrSetResponseTypeError> {
        use self::XhrResponseType::*;
        let response_type = match kind {
            ArrayBuffer => "arraybuffer",
            Blob => "blob",
            Document => "document",
            Json => "json",
            Text => "text"
        };

        js_try!(
            @{self}.responseType = @{response_type};
        ).unwrap()
    }

    /// Returns a string that contains the response to the request as text, or None
    /// if the request was unsuccessful or has not yet been sent.
    ///
    ///[(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseText)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-responsetext
    pub fn response_text(&self) -> Result< Option< String >, TODO > {
        let response = js!(return @{self}.responseText;);
        match response {
            Value::Null => Ok( None ),
            Value::String( resp ) => Ok( Some( resp ) ),
            _ => unreachable!(),
        }
    }

    /// Returns the object representing the response
    ///
    ///[(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/response)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-response
    pub fn raw_response(&self) -> Value {
        js!(return @{self}.response;)
    }

    /// Returns an unsigned short with the status of the response of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/status)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-status
    pub fn status(&self) -> u16 {
        js!(return @{self}.status;).try_into().unwrap()
    }

    /// Open connection with given method (ie GET or POST), and the url to hit.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-open
    pub fn open(&self, method: &str, url: &str) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.open(@{method}, @{url}, true);
        };

        Ok(())
    }

    /// Returns the string containing the text of the specified header. If there
    /// are multiple response headers with the same name, then their values are
    /// returned as a single concatenated string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getResponseHeader)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-getresponseheader
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
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-setrequestheader
    pub fn set_request_header(&self, header: &str, value: &str) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.setRequestHeader(@{header}, @{value});
        };

        Ok(())
    }

    /// Send request on an open connection with no data
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-send
    pub fn send(&self) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.send();
        };

        Ok(())
    }

    /// Send request on an open connection with string body
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-send
    pub fn send_with_string(&self, body: &str) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.send(@{body});
        };

        Ok(())
    }

    /// Send request on an open connection with a byte array body
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-send
    pub fn send_with_bytes(&self, body: &[u8]) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.send(@{UnsafeTypedArray(body)});
        };

        Ok(())
    }

    /// Aborts the request if it has already been sent.
    /// When a request is aborted, its [ready_state](struct.XmlHttpRequest.html#method.ready_state) is changed to [Done](enum.XhrReadyState.html#variant.Done)
    /// and the [status](struct.XmlHttpRequest.html#method.status) code is set to
    /// [Unsent](enum.XhrReadyState.html#variant.Unsent).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/abort)
    // https://xhr.spec.whatwg.org/#ref-for-dom-xmlhttprequest-abort
    pub fn abort(&self) {
        js! { @(no_return)
            @{self}.abort();
        };
    }
}

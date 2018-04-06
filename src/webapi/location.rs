use webcore::value::Reference;
use webcore::value::Value;
use webcore::try_from::TryInto;
use webapi::dom_exception::SecurityError;

/// The `Location` interface represents the location (URL) of the object it
/// is linked to. Changes done on it are reflected on the object it relates
/// to. Both the [Document](struct.Document.html) and [Window](struct.Window.html)
/// interface have such a linked `Location`, accessible via [Document::location](struct.Document.html#method.location)
/// and [Window::location](struct.Window.html#method.location) respectively.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location)
// https://html.spec.whatwg.org/#location
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Location")]
pub struct Location( Reference );

macro_rules! jsattr {
    [$s:ident, $attr:ident] => {{
        js_try!(
            return @{$s}.$attr;
        ).unwrap()
    }}
}

impl Location {
    /// The entire URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)
    // https://html.spec.whatwg.org/#dom-location-href
    pub fn href( &self ) -> Result< String, SecurityError > {
        jsattr!(self, href)
    }

    /// Returns a `String` containing the Unicode serialization of the origin of the represented
    /// URL, that is:
    ///
    /// - For URL using the http or https, the scheme followed by `'://'`, followed by the domain,
    ///   followed by `':'`, followed by the port (the default port, 80 and 443 respectively, if
    ///   explicitely specified);
    /// - For URL using `file: scheme`, the value is browser dependant.
    /// - For URL using the blob: scheme, the origin of the URL following blob:. E.g
    ///   "blob:https://mozilla.org" will have "https://mozilla.org".
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/origin)
    // https://html.spec.whatwg.org/#dom-location-origin
    pub fn origin( &self ) -> Result< String, SecurityError > {
        jsattr!(self, origin)
    }

    /// Returns a `String` representing the protocol scheme of the URL, including the final ':'.
    ///
    /// Example: `http:`
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)
    // https://html.spec.whatwg.org/#dom-location-protocol
    pub fn protocol( &self ) -> Result< String, SecurityError > {
        jsattr!(self, protocol)
    }

    /// Returns a `String` containing the host (i.e. hostname) and then, if the port of the
    /// URL is nonempty, a ':', and the port of the URL.
    ///
    /// Example: `hitchhikers.com:4242`
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)
    // https://html.spec.whatwg.org/#dom-location-host
    pub fn host( &self ) -> Result< String, SecurityError > {
        jsattr!(self, host)
    }

    /// Returns a `String` which is the domain of the URL
    ///
    /// Example: `mozilla.com`
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)
    // https://html.spec.whatwg.org/#dom-location-hostname
    pub fn hostname( &self ) -> Result< String, SecurityError > {
        jsattr!(self, hostname)
    }

    /// Returns a `String` containing the port number or `""` if there is no port.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)
    // https://html.spec.whatwg.org/#dom-location-port
    pub fn port( &self ) -> Result< String, SecurityError > {
        jsattr!(self, port)
    }

    /// Returns a `String` containing an initial '/' followed by the path of the URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)
    // https://html.spec.whatwg.org/#dom-location-pathname
    pub fn pathname( &self ) -> Result< String, SecurityError > {
        jsattr!(self, pathname)
    }

    /// Returns a `String` which is a search string, also called a query string, that is a `String`
    /// containing a '?' followed by the parameters of the URL.
    ///
    /// These can then be further parsed via another library.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)
    // https://html.spec.whatwg.org/#dom-location-search
    pub fn search( &self ) -> Result< String, SecurityError > {
        jsattr!(self, search)
    }

    /// Returns a `String` containing a '#' followed by the fragment
    /// identifier of the URL. The fragment is not percent-decoded.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-hash
    pub fn hash( &self ) -> Result< String, SecurityError > {
        jsattr!(self, hash)
    }
}

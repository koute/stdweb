use webcore::value::Reference;
use webapi::dom_exception::SecurityError;

/// The `Location` interface represents the location (URL) of the object it
/// is linked to. Changes done on it are reflected on the object it relates
/// to. Both the [Document](struct.Document.html) and [Window](struct.Window.html)
/// interface have such a linked `Location`, accessible via [Document::location](struct.Document.html#method.location)
/// and [Window::location](struct.Window.html#method.location) respectively.
///
/// Note that all `Location` methods can return a `SecurityError` if the `Location` object's
/// relevant `Document`'s origin is not same origin-domain with the entry settings object's origin.
/// See: https://html.spec.whatwg.org/#dom-location-href
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location)
// https://html.spec.whatwg.org/#location
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Location")]
pub struct Location( Reference );

impl Location {
    /// The entire URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-href
    pub fn href( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.href; ).unwrap()
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
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-origin
    pub fn origin( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.origin; ).unwrap()
    }

    /// Returns a `String` representing the protocol scheme of the URL, including the final ':'.
    ///
    /// Example: `http:`
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-protocol
    pub fn protocol( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.protocol; ).unwrap()
    }

    /// Returns a `String` containing the host (i.e. hostname) and then, if the port of the
    /// URL is nonempty, a ':', and the port of the URL.
    ///
    /// Example: `hitchhikers.com:4242`
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-host
    pub fn host( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.host; ).unwrap()
    }

    /// Returns a `String` which is the domain of the URL
    ///
    /// Example: `mozilla.com`
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-hostname
    pub fn hostname( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.hostname; ).unwrap()
    }

    /// Returns a `String` containing the port number or `""` if there is no port.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-port
    pub fn port( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.port; ).unwrap()
    }

    /// Returns a `String` containing an initial '/' followed by the path of the URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-pathname
    pub fn pathname( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.pathname; ).unwrap()
    }

    /// Returns a `String` which is a search string, also called a query string, that is a `String`
    /// containing a '?' followed by the parameters of the URL.
    ///
    /// These can then be further parsed via another library.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-search
    pub fn search( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.search; ).unwrap()
    }

    /// Returns a `String` containing a '#' followed by the fragment
    /// identifier of the URL. The fragment is not percent-decoded.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-hash
    pub fn hash( &self ) -> Result< String, SecurityError > {
        js_try!( return @{self}.hash; ).unwrap()
    }
}

use webcore::value::Reference;
use webcore::try_from::TryInto;
use private::UnimplementedException;

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

impl Location {
    /// The entire URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)
    // https://html.spec.whatwg.org/#dom-location-href
    pub fn href( &self ) -> Result< String, UnimplementedException > {
        Ok( js!(
            return @{self}.href;
        ).try_into().unwrap() )
    }

    /// Returns a `String` containing a '#' followed by the fragment
    /// identifier of the URL. The fragment is not percent-decoded.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)
    // https://html.spec.whatwg.org/#the-location-interface:dom-location-hash
    pub fn hash( &self ) -> Result< String, UnimplementedException > {
        Ok( js!(
            return @{self}.hash;
        ).try_into().unwrap() )
    }
}

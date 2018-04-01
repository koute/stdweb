use webcore::value::Reference;
use webcore::try_from::TryInto;
use private::TODO;

/// Used by the `dataset` HTML attribute to represent data for custom attributes added to elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringMap)
// https://html.spec.whatwg.org/#domstringmap
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DOMStringMap")]
pub struct StringMap( Reference );

// The methods here are deliberately named exactly as those from Rust's HashMap.
impl StringMap {
    /// Returns a value corresponding to the key.
    // https://html.spec.whatwg.org/#dom-domstringmap-nameditem
    pub fn get( &self, key: &str ) -> Option< String > {
        js!( return @{self}[ @{key} ]; ).try_into().ok()
    }

    /// Inserts a key-value pair into the map.
    // https://html.spec.whatwg.org/#dom-domstringmap-setitem
    pub fn insert( &self, key: &str, value: &str ) -> Result< (), TODO > {
        js!( @(no_return)
            @{self}[ @{key} ] = @{value};
        );

        Ok(())
    }

    /// Removes a key from the map.
    // https://html.spec.whatwg.org/#dom-domstringmap-removeitem
    pub fn remove( &self, key: &str ) {
        js!( @(no_return)
            delete @{self}[ @{key} ];
        );
    }

    /// Returns true if the map contains a value for the specified key.
    pub fn contains_key( &self, key: &str ) -> bool {
        js!( return @{key} in @{self}; ).try_into().unwrap()
    }
}

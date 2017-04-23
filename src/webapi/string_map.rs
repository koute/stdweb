use webcore::value::Reference;
use webcore::try_from::TryInto;

/// Used by the `dataset` HTML attribute to represent data for custom attributes added to elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringMap)
pub struct StringMap( Reference );

reference_boilerplate! {
    StringMap,
    instanceof DOMStringMap
}

// The methods here are deliberately named exactly as those from Rust's HashMap.
impl StringMap {
    /// Returns a value corresponding to the key.
    pub fn get( &self, key: &str ) -> Option< String > {
        js!( return @{self}[ @{key} ]; ).try_into().ok()
    }

    /// Inserts a key-value pair into the map.
    pub fn insert( &self, key: &str, value: &str ) {
        js!( @(no_return)
            @{self}[ @{key} ] = @{value};
        );
    }

    /// Removes a key from the map.
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

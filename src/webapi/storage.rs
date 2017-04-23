use webcore::value::Reference;
use webcore::try_from::TryInto;

/// The `Storage` interface of the Web Storage API provides access to
/// the session storage or local storage for a particular domain.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Storage)
pub struct Storage( Reference );

reference_boilerplate! {
    Storage,
    instanceof Storage
}

impl Storage {
    /// Gets the number of data items stored in the `Storage` object.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)
    pub fn len( &self ) -> usize {
        let length: i32 = js!( return @{self}.length; ).try_into().unwrap();
        length as usize
    }

    /// Returns a value corresponding to the key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)
    pub fn get( &self, key: &str ) -> Option< String > {
        js!( return @{self}.getItem( @{key} ); ).try_into().ok()
    }

    /// Inserts a key-value pair into the storage.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)
    pub fn insert( &self, key: &str, value: &str ) {
        js!( @(no_return)
            @{self}.setItem( @{key}, @{value} );
        );
    }

    /// Removes a key from the storage.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)
    pub fn remove( &self, key: &str ) {
        js!( @(no_return)
            @{self}.removeItem( @{key} );
        );
    }

    /// When invoked, will empty all keys out of the storage.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)
    pub fn clear( &self ) {
        js!( @(no_return)
            @{self}.clear();
        );
    }

    /// Return the name of the nth key in the storage.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)
    pub fn key( &self, nth: usize ) -> Option< String > {
        js!( return @{self}.key( @{nth as u32} ); ).try_into().ok()
    }

    /// Returns true if the storage contains a value for the specified key.
    pub fn contains_key( &self, key: &str ) -> bool {
        js!( return !!@{self}.getItem( @{key} ); ).try_into().unwrap()
    }
}
use webcore::value::Reference;
use webcore::try_from::TryInto;

/// The `TokenList` represents a set of space-separated tokens.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList)
pub struct TokenList( Reference );

reference_boilerplate! {
    TokenList,
    instanceof DOMTokenList
}

impl TokenList {
    /// Gets the number of tokens in the list.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/length)
    pub fn len( &self ) -> usize {
        let length: i32 = js!( return @{self}.length; ).try_into().unwrap();
        length as usize
    }

    /// Adds token to the underlying string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)
    pub fn add( &self, token: &str ) {
        js! { @(no_return)
            @{self}.add( @{token} );
        }
    }

    /// Removes token from the underlying string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)
    pub fn remove( &self, token: &str ) {
        js! { @(no_return)
            @{self}.remove( @{token} );
        }
    }

    /// Returns `true` if the underlying string contains token, otherwise `false`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains)
    pub fn contains( &self, token: &str ) -> bool {
        js!( return @{self}.contains( @{token} ); ).try_into().unwrap()
    }
}
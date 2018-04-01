use webcore::value::Reference;
use webcore::try_from::TryInto;
use private::TODO;

/// The `TokenList` represents a set of space-separated tokens.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList)
// https://dom.spec.whatwg.org/#domtokenlist
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DOMTokenList")]
pub struct TokenList( Reference );

impl TokenList {
    /// Gets the number of tokens in the list.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/length)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-length
    pub fn len( &self ) -> u32 {
        js!( return @{self}.length; ).try_into().unwrap()
    }

    /// Adds token to the underlying string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-add
    pub fn add( &self, token: &str ) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.add( @{token} );
        }

        Ok(())
    }

    /// Removes token from the underlying string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-remove
    pub fn remove( &self, token: &str ) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.remove( @{token} );
        }

        Ok(())
    }

    /// Returns `true` if the underlying string contains token, otherwise `false`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains)
    // https://dom.spec.whatwg.org/#ref-for-dom-domtokenlist-contains
    pub fn contains( &self, token: &str ) -> bool {
        js!( return @{self}.contains( @{token} ); ).try_into().unwrap()
    }
}

use webcore::value::Reference;
use webcore::try_from::TryInto;

/// The `DOMStringList` interface is a non-fashionable retro way of representing a list of strings.
///
/// [(JavaScript docs)](https://html.spec.whatwg.org/multipage/common-dom-interfaces.html#the-domstringlist-interface)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DOMStringList")]
pub struct DOMStringList( Reference );

impl DOMStringList {

    /// Returns the number of strings in the `DOMStringList`.
    pub fn length( &self ) -> u32 {
        js!( return @{self}.length; ).try_into().unwrap()
    }
    
    /// Returns the string with index `index`.
    pub fn item( &self, index: u32) -> Option<String> {
        js!( return @{self}.item(@{index}); ).try_into().unwrap()
    }
    
    /// Returns true if the DOMStringList contains `string`, and false otherwise.
    pub fn contains( &self, string: &str) -> bool {
        js! ( return @{self}.container(@{string}); ).try_into().unwrap()
    }

}

impl IntoIterator for DOMStringList {
    type Item = String;
    type IntoIter = DOMStringListIterator;

    fn into_iter(self) -> Self::IntoIter {
        DOMStringListIterator { dom_string_list: self, index: 0 }
    }
}

#[derive(Debug)]
pub struct DOMStringListIterator {
    dom_string_list: DOMStringList,
    index: u32
}

impl Iterator for DOMStringListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let result = self.dom_string_list.item(self.index);
        self.index += 1;
        result
    }
}

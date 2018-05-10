use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::element::Element;

/// The `HtmlCollection` interface represents a generic collection
/// (array-like object similar to arguments) of elements (in document order)
/// and offers methods and properties for selecting from the list.
/// 
/// An `HtmlCollection` in the HTML DOM is live; it is automatically
/// updated when the underlying document is changed.
/// 
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection)
// https://dom.spec.whatwg.org/#interface-htmlcollection
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLCollection")]
pub struct HtmlCollection( Reference );

impl HtmlCollection {
    /// Returns the number of elements in the collection.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection)
    // https://dom.spec.whatwg.org/#ref-for-dom-htmlcollection-length
    pub fn len( &self ) -> u32 {
        js!( return @{self}.length; ).try_into().unwrap()
    }

    /// Returns an element from an `HtmlCollection` by index.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/item)
    // https://dom.spec.whatwg.org/#ref-for-dom-htmlcollection-item
    pub fn item( &self, index: u32 ) -> Option< Element > {
        js!(
            return @{self}.item(@{index});
        ).try_into().unwrap()
    }

    /// Returns an iterator over the collection.
    pub fn iter( &self ) -> ElementIter {
        ElementIter {
            list: self.clone(),
            index: 0
        }
    }
}


impl IntoIterator for HtmlCollection {
    type Item = Element;
    type IntoIter = ElementIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        ElementIter {
            list: self,
            index: 0
        }
    }
}

impl< 'a > IntoIterator for &'a HtmlCollection {
    type Item = Element;
    type IntoIter = ElementIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        ElementIter {
            list: self.clone(),
            index: 0
        }
    }
}

#[derive(Debug)]
pub struct ElementIter {
    list: HtmlCollection,
    index: u32
}

impl Iterator for ElementIter {
    type Item = Element;
    fn next( &mut self ) -> Option< Self::Item > {
        let item = self.list.item(self.index);
        self.index += 1;
        item
    }
}

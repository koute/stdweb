use webcore::reference_type::ReferenceType;
use webapi::node_list::NodeList;
use webapi::element::Element;
use private::UnimplementedException;

/// The `ParentNode` mixin contains methods and properties
/// that are common to all types of `Node` objects that can
/// have children.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ParentNode)
// https://dom.spec.whatwg.org/#parentnode
pub trait IParentNode: ReferenceType {
    /// Returns the first element that is a descendant of the element on which it is
    /// invoked that matches the specified group of selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)
    // https://dom.spec.whatwg.org/#ref-for-dom-parentnode-queryselector
    fn query_selector( &self, selector: &str ) -> Result< Option< Element >, UnimplementedException > {
        unsafe {
            Ok( js!( return @{self.as_ref()}.querySelector( @{selector} ); ).into_reference_unchecked() )
        }
    }

    /// Returns a non-live [NodeList](struct.NodeList.html) of all elements descended
    /// from the element on which it is invoked that matches the specified group of CSS selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)
    // https://dom.spec.whatwg.org/#ref-for-dom-parentnode-queryselectorall
    fn query_selector_all( &self, selector: &str ) -> Result< NodeList, UnimplementedException > {
        unsafe {
            Ok( js!( return @{self.as_ref()}.querySelectorAll( @{selector} ); ).into_reference_unchecked().unwrap() )
        }
    }
}

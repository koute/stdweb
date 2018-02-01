use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::token_list::TokenList;
use webapi::node_list::NodeList;

/// The `IElement` interface represents an object of a [Document](struct.Document.html).
/// This interface describes methods and properties common to all
/// kinds of elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
pub trait IElement: INode {
    /// The Element.classList is a read-only property which returns a live
    /// [TokenList](struct.TokenList.html) collection of the class attributes
    /// of the element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)
    fn class_list( &self ) -> TokenList {
        unsafe {
            js!( return @{self.as_ref()}.classList; ).into_reference_unchecked().unwrap()
        }
    }

    /// Returns the first element that is a descendant of the element on which it is
    /// invoked that matches the specified group of selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)
    fn query_selector( &self, selector: &str ) -> Option< Element > {
        // TODO: This can throw an exception in case of an invalid selector;
        //       convert the return type to a Result.
        unsafe {
            js!( return @{self.as_ref()}.querySelector( @{selector} ); ).into_reference_unchecked()
        }
    }

    /// Returns a non-live [NodeList](struct.NodeList.html) of all elements descended
    /// from the element on which it is invoked that matches the specified group of CSS selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)
    fn query_selector_all( &self, selector: &str ) -> NodeList {
        unsafe {
            js!( return @{self.as_ref()}.querySelectorAll( @{selector} ); ).into_reference_unchecked().unwrap()
        }
    }
}

/// A reference to a JavaScript object which implements the [IElement](trait.IElement.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
pub struct Element( Reference );

impl IEventTarget for Element {}
impl INode for Element {}
impl IElement for Element {}

reference_boilerplate! {
    Element,
    instanceof Element
    convertible to EventTarget
    convertible to Node
}

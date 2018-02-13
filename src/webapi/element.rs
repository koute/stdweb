use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::token_list::TokenList;
use webapi::parent_node::IParentNode;

/// The `IElement` interface represents an object of a [Document](struct.Document.html).
/// This interface describes methods and properties common to all
/// kinds of elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
// https://dom.spec.whatwg.org/#element
pub trait IElement: INode + IParentNode {
    /// The Element.classList is a read-only property which returns a live
    /// [TokenList](struct.TokenList.html) collection of the class attributes
    /// of the element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-classlist
    fn class_list( &self ) -> TokenList {
        unsafe {
            js!( return @{self.as_ref()}.classList; ).into_reference_unchecked().unwrap()
        }
    }
}

/// A reference to a JavaScript object which implements the [IElement](trait.IElement.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
// https://dom.spec.whatwg.org/#element
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "Element")]
#[reference(subclass_of(EventTarget, Node))]
pub struct Element( Reference );

impl IEventTarget for Element {}
impl INode for Element {}
impl IElement for Element {}

impl< T: IElement > IParentNode for T {}

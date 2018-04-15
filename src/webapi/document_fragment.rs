use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};

/// A reference to a JavaScript object which implements the [IElement](trait.IElement.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
// https://dom.spec.whatwg.org/#element
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DocumentFragment")]
#[reference(subclass_of(EventTarget, Node))]
pub struct DocumentFragment( Reference );

impl IEventTarget for DocumentFragment {}
impl INode for DocumentFragment {}
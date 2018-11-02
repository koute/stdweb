use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::parent_node::IParentNode;

/// A reference to a JavaScript object DocumentFragment.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment)
// https://dom.spec.whatwg.org/#documentfragment
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DocumentFragment")]
#[reference(subclass_of(EventTarget, Node))]
pub struct DocumentFragment( Reference );

impl IEventTarget for DocumentFragment {}
impl INode for DocumentFragment {}
impl IParentNode for DocumentFragment {}

use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};

/// The `TextNode` represents the textual content of an [IElement](trait.IElement.html)
///
/// If an element has no markup within its content, it has
/// a single child `TextNode` that contains the element's
/// text. However, if the element contains markup, it is parsed
/// into information items and `TextNode`s that form its children.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Text)
// https://dom.spec.whatwg.org/#text
pub struct TextNode( Reference );

impl IEventTarget for TextNode {}
impl INode for TextNode {}

reference_boilerplate! {
    TextNode,
    instanceof Text
    convertible to EventTarget
    convertible to Node
}

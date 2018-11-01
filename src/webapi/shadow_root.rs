use webapi::document_fragment::DocumentFragment;
use webapi::element::Element;
use webapi::event_target::{EventTarget, IEventTarget};
use webapi::node::{INode, Node};
use webapi::parent_node::IParentNode;
use webcore::try_from::TryInto;
use webcore::value::Reference;

/// The mode associated to a shadow root.
/// Mainly used in [IElement::attach_shadow](trait.IElement.html#method.attach_shadow) and
/// [IShadowRoot::mode](trait.IShadowRoot.html#method.mode).
// https://dom.spec.whatwg.org/#shadowroot-mode
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShadowRootMode {
    /// { mode: "open" }
    Open,
    /// { mode: "closed" }
    Closed,
}

impl ShadowRootMode {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ShadowRootMode::Open => "open",
            ShadowRootMode::Closed => "closed",
        }
    }
}

/// The `ShadowRoot` interface of the Shadow DOM API is the root node of a DOM
/// subtree that is rendered separately from a document's main DOM tree.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)
// https://dom.spec.whatwg.org/#interface-shadowroot
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ShadowRoot")]
#[reference(subclass_of(EventTarget, Node, DocumentFragment))]
pub struct ShadowRoot(Reference);

impl IEventTarget for ShadowRoot {}
impl INode for ShadowRoot {}
impl IParentNode for ShadowRoot {}

impl ShadowRoot {
    /// The mode property of the `ShadowRoot` specifies its mode.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode)
    // https://dom.spec.whatwg.org/#ref-for-dom-shadowroot-mode
    pub fn mode(&self) -> ShadowRootMode {
        let mode_string: String = js!( return @{self.as_ref()}.mode; ).try_into().unwrap();

        match mode_string.as_str() {
            "open" => ShadowRootMode::Open,
            "closed" => ShadowRootMode::Closed,
            _ => unreachable!("mode can only be `open` or `closed`"),
        }
    }

    /// The host read-only property of the `ShadowRoot` returns a reference to the DOM element
    /// the ShadowRoot is attached to.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host)
    // https://dom.spec.whatwg.org/#ref-for-dom-shadowroot-host
    pub fn host(&self) -> Element {
        js!( return @{self.as_ref()}.host; ).try_into().unwrap()
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::document::document;
    use webapi::element::{Element, IElement};
    use webapi::html_elements::{SlotContentKind, SlotElement, TemplateElement};
    use webapi::node::{CloneKind, INode, Node};
    use webapi::parent_node::IParentNode;

    #[test]
    fn test_shadow_root_host() {
        let element = document().create_element("div").unwrap();
        let shadow_root = element.attach_shadow(ShadowRootMode::Open).unwrap();
        assert_eq!(shadow_root.host(), element);
    }

    #[test]
    fn test_shadow_dom() {
        let div: Element = Node::from_html(r#"<div>
  <span id="span1" slot="slot1"></span>
</div>"#)
            .unwrap()
            .try_into()
            .unwrap();
        let tpl: TemplateElement = Node::from_html(r#"<template>
  <slot name="slot1" id="slot1"><span id="span2"></span></slot><br>
  <slot name="slot2" id="slot2"><span id="span3"></span></slot><br>
</template>"#)
            .unwrap()
            .try_into()
            .unwrap();

        let span1 = div.query_selector("#span1").unwrap().unwrap();

        let shadow_root = div.attach_shadow(ShadowRootMode::Open).unwrap();
        let n = tpl.content().clone_node(CloneKind::Deep).unwrap();

        shadow_root.append_child(&n);

        let slot1: SlotElement = shadow_root
            .query_selector("#slot1")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let slot2: SlotElement = shadow_root
            .query_selector("#slot2")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(
            slot1
                .assigned_nodes(SlotContentKind::AssignedOnly)
                .iter()
                .map(|m| m.clone().try_into().unwrap())
                .collect::<Vec<Element>>(),
            &[span1.clone()]
        );
        assert_eq!(slot2.assigned_nodes(SlotContentKind::AssignedOnly).len(), 0);

        assert_eq!(
            slot1
                .assigned_nodes(SlotContentKind::WithFallback)
                .iter()
                .map(|m| m.clone().try_into().unwrap())
                .collect::<Vec<Element>>(),
            &[span1.clone()]
        );

        let slot2_nodes = slot2.assigned_nodes(SlotContentKind::WithFallback);
        assert_eq!(slot2_nodes.len(), 1);
        let fallback_span = slot2_nodes[0].clone();

        assert_eq!(js!( return @{fallback_span}.id; ), "span3");
    }
}

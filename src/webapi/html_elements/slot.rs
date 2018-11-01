use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// An enum which determines whether
/// [SlotElement::assigned_nodes](struct.SlotElement.html#method.assigned_nodes) /
/// [SlotElement::assigned_elements](struct.SlotElement.html#method.assigned_elements) will
/// return the fallback content when nothing has been assigned to the slot.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SlotContentKind {
    /// Will only return content assigned.
    AssignedOnly,
    /// Will return the fallback content if nothing has been assigned.
    WithFallback,
}

impl SlotContentKind {
    fn to_bool(&self) -> bool {
        match *self {
            SlotContentKind::AssignedOnly => false,
            SlotContentKind::WithFallback => true,
        }
    }
}

/// The HTML `<slot>` element represents a placeholder inside a web component that
/// you can fill with your own markup, which lets you create separate DOM trees and
/// present them together.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
// https://html.spec.whatwg.org/multipage/scripting.html#htmlslotelement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLSlotElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct SlotElement( Reference );

impl IEventTarget for SlotElement {}
impl INode for SlotElement {}
impl IElement for SlotElement {}
impl IHtmlElement for SlotElement {}

impl SlotElement {
    /// The slot's name
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)
    // https://html.spec.whatwg.org/multipage/scripting.html#attr-slot-name
    #[inline]
    pub fn name ( &self ) -> String {
        js! (
            return @{self}.name;
        ).try_into().unwrap()
    }

    /// Setter of name.
    #[inline]
    pub fn set_name( &self, new_name: &str ) {
        js! ( @(no_return)
            @{self}.name = @{new_name};
        );
    }

    /// Returns slot's assigned nodes.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    // https://html.spec.whatwg.org/multipage/scripting.html#the-slot-element:dom-slot-assignednodes
    pub fn assigned_nodes( &self, kind: SlotContentKind ) -> Vec<Node> {
        js! (
            return @{self}.assignedNodes( { flatten: @{kind.to_bool()} } );
        ).try_into().unwrap()
    }

    /// Similar to [assigned_nodes()](#method.assigned_nodes) but limited result to only elements.
    ///
    /// [(Spec)](https://html.spec.whatwg.org/multipage/scripting.html#dom-slot-assignedelements)
    // https://html.spec.whatwg.org/multipage/scripting.html#the-slot-element:dom-slot-assignedelements
    pub fn assigned_elements( &self, kind: SlotContentKind ) -> Vec<Element> {
        js! (
            return @{self}.assignedElements( { flatten: @{kind.to_bool()} } );
        ).try_into().unwrap()
    }
}

// Remove unused imports when `#[ignore]` below is removed.
#[allow(unused_imports)]
#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::element::{Element, IElement};
    use webapi::html_elements::TemplateElement;
    use webapi::node::{CloneKind, INode, Node};
    use webapi::parent_node::IParentNode;
    use webapi::shadow_root::ShadowRootMode;

    // `#[ignore]`ed because travis keeps complaining.
    #[test]
    #[ignore]
    fn test_assigned_elements() {
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
            slot1.assigned_elements(SlotContentKind::AssignedOnly),
            &[span1.clone()]
        );
        assert_eq!(
            slot2.assigned_elements(SlotContentKind::AssignedOnly).len(),
            0
        );

        assert_eq!(
            slot1
                .assigned_nodes(SlotContentKind::WithFallback)
                .iter()
                .map(|m| m.clone().try_into().unwrap())
                .collect::<Vec<Element>>(),
            &[span1.clone()]
        );
        assert_eq!(
            slot1.assigned_elements(SlotContentKind::WithFallback),
            &[span1.clone()]
        );

        let slot2_nodes = slot2.assigned_nodes(SlotContentKind::WithFallback);
        let slot2_elements = slot2.assigned_elements(SlotContentKind::WithFallback);

        assert_eq!(
            slot2_nodes
                .iter()
                .map(|m| m.clone().try_into().unwrap())
                .collect::<Vec<Element>>(),
            slot2_elements
        );
        assert_eq!(slot2_nodes.len(), 1);
        let fallback_span = slot2_nodes[0].clone();

        assert_eq!(js!( return @{fallback_span}.id; ), "span3");
    }
}

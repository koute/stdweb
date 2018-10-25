use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};
use webapi::slotable::ISlotable;

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
impl ISlotable for SlotElement {}

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
        let is_flatten = match kind {
            SlotContentKind::AssignedOnly => false,
            SlotContentKind::WithFallback => true,
        };

        js! (
            return @{self}.assignedNodes( { flatten: @{is_flatten} } );
        ).try_into().unwrap()
    }

    /// Similar to [assigned_nodes()](#method.assigned_nodes) but limited result to only elements.
    ///
    /// [(Spec)](https://html.spec.whatwg.org/multipage/scripting.html#dom-slot-assignedelements)
    // https://html.spec.whatwg.org/multipage/scripting.html#the-slot-element:dom-slot-assignedelements
    pub fn assigned_elements( &self, kind: SlotContentKind ) -> Vec<Element> {
        let is_flatten = match kind {
            SlotContentKind::AssignedOnly => false,
            SlotContentKind::WithFallback => true,
        };

        js! (
            return @{self}.assignedElements( { flatten: @{is_flatten} } );
        ).try_into().unwrap()
    }

}

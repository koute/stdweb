use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// The HTML `<slot>` element represents a placeholder inside a web component that
/// you can fill with your own markup, which lets you create separate DOM trees and
/// present them together.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
// https://html.spec.whatwg.org/multipage/scripting.html#the-slot-element
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
    pub fn set_name<S: AsRef<str>>( &self, new_name: S ) {
        js! ( @(no_return)
            @{self}.name = @{new_name.as_ref()};
        );
    }

    /// Returns slot's assigned nodes.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)
    // https://html.spec.whatwg.org/multipage/scripting.html#the-slot-element:dom-slot-assignednodes
    pub fn assigned_nodes( &self, flatten: bool ) -> Vec<Node> {
        if flatten {
            js! (
                return @{self}.assignedNodes( { flatten: true } );
            ).try_into().unwrap()
        } else {
            js! (
                return @{self}.assignedNodes();
            ).try_into().unwrap()
        }
    }

    /// Similar to [assigned_nodes()](#method.assigned_nodes) but limited result to only elements.
    ///
    /// [(Spec)](https://html.spec.whatwg.org/multipage/scripting.html#dom-slot-assignedelements)
    // https://html.spec.whatwg.org/multipage/scripting.html#the-slot-element:dom-slot-assignedelements
    pub fn assigned_elements( &self, flatten: bool ) -> Vec<Element> {
        if flatten {
            js! (
                return @{self}.assignedElements( { flatten: true } );
            ).try_into().unwrap()
        } else {
            js! (
                return @{self}.assignedElements();
            ).try_into().unwrap()
        }
    }

}


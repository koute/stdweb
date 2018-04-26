use webapi::element::{Element, IElement};
use webapi::event_target::{EventTarget, IEventTarget};
use webapi::html_element::{HtmlElement, IHtmlElement};
use webapi::node::{INode, Node};
use webcore::try_from::TryInto;
use webcore::value::Reference;

/// The HTML <select> element represents a control that provides a menu of options.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
// https://html.spec.whatwg.org/#the-select-element
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLOptionElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct OptionElement(Reference);

impl IEventTarget for OptionElement {}
impl INode for OptionElement {}
impl IElement for OptionElement {}
impl IHtmlElement for OptionElement {}

impl OptionElement {
    /// The position of the option within the list of options it belongs to, in tree-order.
    /// If the option is not part of a list of options, like when it is part of
    /// the <datalist> element, the value is 0.
    // https://html.spec.whatwg.org/#dom-option-index
    pub fn index(&self) -> i32 {
        js!(
            return @{self}.index;
        ).try_into().unwrap()
    }

    /// Reflects the value of the value HTML attribute, if it exists;
    /// otherwise reflects value of the Node.textContent property.
    //
    pub fn value(&self) -> String {
        js!(
            return @{self}.value;
        ).try_into().unwrap()
    }
}
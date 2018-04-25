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
#[reference(instance_of = "HTMLSelectElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct SelectElement(Reference);

impl IEventTarget for SelectElement {}
impl INode for SelectElement {}
impl IElement for SelectElement {}
impl IHtmlElement for SelectElement {}

impl SelectElement {
    /// Returns the `Some(index)` of the first selected item, if any, or `None` if there is no selected item.
    // https://html.spec.whatwg.org/#dom-select-selectedindex
    /*pub fn selected_index(&self) -> Option<i64> {
        let si = js! (
            return @{self}.selectedIndex;
        ).try_into()
            .unwrap();
        if si < 0 {
            None
        } else {
            Some(si)
        }
    }*/

    /// Change selected index to the given value.
    // https://html.spec.whatwg.org/#dom-select-selectedindex
    /*pub fn set_selected_index(&self, selected_index: Option<i64>) {
        let selected_index = selected_index.unwrap_or(-1);
        js!{
            @(no_return)
            @{self}.selectedIndex = @{selected_index};
        }
    }*/

    /// Returns the `Some(value)` of the first selected item, if any, or `None` if there is no selected item.
    // https://html.spec.whatwg.org/#dom-select-value
    pub fn value(&self) -> Option<String> {
        let value = js!(
            return @{self}.value;
        ).try_into()
            .unwrap();
        if value == "" {
            None
        } else {
            Some(value)
        }
    }

    /// Change the selected value to the given value.
    // https://html.spec.whatwg.org/#dom-select-value
    pub fn set_value(&self, value: Option<String>) {
        let value = value.unwrap_or("".to_string());
        js!{
            @(no_return)
            @{self}.value = @{value};
        }
    }
}

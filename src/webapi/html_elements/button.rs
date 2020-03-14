use webapi::element::{Element, IElement};
use webapi::event_target::{EventTarget, IEventTarget};
use webapi::html_element::{HtmlElement, IHtmlElement};
use webapi::node::{INode, Node};
use webcore::try_from::TryInto;
use webcore::value::Reference;

/// The HTML `<textarea>` element represents a multi-line plain-text editing control.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/HTML/Element/textarea)
// https://html.spec.whatwg.org/#htmlButtonElement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLButtonElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct ButtonElement(Reference);

impl IEventTarget for ButtonElement {}
impl INode for ButtonElement {}
impl IElement for ButtonElement {}
impl IHtmlElement for ButtonElement {}

impl ButtonElement {
    // Sets the type of button.
    // https://html.spec.whatwg.org/#attr-button-type
    #[inline]
    pub fn set_type(&self, kind: &str) {
        js! { @(no_return)
            @{self}.type = @{kind};
        }
    }

    // Sets either button disable or not
    // https://html.spec.whatwg.org/#attr-fe-disabled
    #[inline]
    pub fn set_disabled(&self, status: bool) {
        js! { @(no_return)
            @{self}.disabled = @{status};
        }
    }

    // Sets the name of button, useful if button related to form element
    // https://html.spec.whatwg.org/#attr-fe-name
    #[inline]
    pub fn set_name(&self, name: &str) {
        js! { @(no_return)
            @{self}.name = @{name};
        }
    }

    // Sets the value of button, useful if button related to form element
    // https://html.spec.whatwg.org/#attr-button-value
    #[inline]
    pub fn set_raw_value(&self, value: &str) {
        js! { @(no_return)
            @{self}.value = @{value};
        }
    }
}

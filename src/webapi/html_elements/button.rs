use webapi::element::{Element, IElement};
use webapi::event_target::{EventTarget, IEventTarget};
use webapi::html_element::{HtmlElement, IHtmlElement};
use webapi::node::{INode, Node};
use webcore::try_from::TryInto;
use webcore::value::Reference;

/// The HTML `<button>` element represents a clickable button
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
// https://html.spec.whatwg.org/#the-button-element
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLButtonElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct ButtonElement(Reference);

impl IEventTarget for ButtonElement {}
impl INode for ButtonElement {}
impl IElement for ButtonElement {}
impl IHtmlElement for ButtonElement {}

impl ButtonElement {
    /// The type attribute controls the behavior of the button when it is activated. 
    /// It is an enumerated attribute. Allowed values: submit | reset | button
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type)
    /// https://html.spec.whatwg.org/#attr-button-type
    #[inline]
    pub fn set_type(&self, kind: &str) {
        js! { @(no_return)
            @{self}.type = @{kind};
        }
    }

    /// This Boolean attribute prevents the user from interacting with the button: it cannot be pressed or focused.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-disabled)
    /// https://html.spec.whatwg.org/#attr-fe-disabled
    #[inline]
    pub fn set_disabled(&self, status: bool) {
        js! { @(no_return)
            @{self}.disabled = @{status};
        }
    }

    /// The name of the button, submitted as a pair with the button’s value as part of the form data.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-name)
    /// https://html.spec.whatwg.org/#attr-fe-name
    #[inline]
    pub fn set_name(&self, name: &str) {
        js! { @(no_return)
            @{self}.name = @{name};
        }
    }

    /// Defines the value associated with the button’s name when it’s submitted with the form data. 
    /// This value is passed to the server in params when the form is submitted.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-value)
    /// https://html.spec.whatwg.org/#attr-button-value    
    #[inline]
    pub fn set_raw_value(&self, value: &str) {
        js! { @(no_return)
            @{self}.value = @{value};
        }
    }
}

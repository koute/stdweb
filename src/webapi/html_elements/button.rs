use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// The HTML `<textarea>` element represents a multi-line plain-text editing control.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/HTML/Element/textarea)
// https://html.spec.whatwg.org/#htmlButtonElement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLButtonElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct ButtonElement( Reference );

impl IEventTarget for ButtonElement {}
impl INode for ButtonElement {}
impl IElement for ButtonElement {}
impl IHtmlElement for ButtonElement {}

impl ButtonElement {
    /// The value of the control.
    // https://html.spec.whatwg.org/#the-button-element
    #[inline]
    pub fn set_type( &self, kind: &str ) {
        js! { @(no_return)
            @{self}.type = @{kind};
        }
    }
}
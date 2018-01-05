use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// The HTML <textarea> element represents a multi-line plain-text editing control.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/HTML/Element/textarea)
pub struct TextAreaElement( Reference );

impl IEventTarget for TextAreaElement {}
impl INode for TextAreaElement {}
impl IElement for TextAreaElement {}
impl IHtmlElement for TextAreaElement {}

reference_boilerplate! {
    TextAreaElement,
    instanceof HTMLTextAreaElement
    convertible to EventTarget
    convertible to Node
    convertible to Element
    convertible to HtmlElement
}

impl TextAreaElement {
    /// The value of the control.
    #[inline]
    pub fn value( &self ) -> String {
        js! (
            return @{self}.value;
        ).try_into().unwrap()
    }

    /// Sets the value of the control.
    #[inline]
    pub fn set_value( &self, value: &str ) {
        js! { @(no_return)
            @{self}.value = @{value};
        }
    }
}

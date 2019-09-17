use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::dom_exception::InvalidStateError;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// The HTML input element is used to create interactive controls
/// for web-based forms in order to accept data from the user.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/HTML/Element/input)
// https://html.spec.whatwg.org/#htmlinputelement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLInputElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct InputElement( Reference );

impl IEventTarget for InputElement {}
impl INode for InputElement {}
impl IElement for InputElement {}
impl IHtmlElement for InputElement {}

impl InputElement {
    /// The value of the control. This attribute is optional except when the input is a radio button or a checkbox.
    /// 
    // https://html.spec.whatwg.org/#the-input-element:dom-input-value
    #[inline]
    pub fn raw_value( &self ) -> String {
        js! (
            return @{self}.value;
        ).try_into().unwrap()
    }

    /// Sets the value of the control.
    /// 
    // https://html.spec.whatwg.org/#dom-input-value
    #[inline]
    pub fn set_raw_value( &self, value: &str ) {
        js! { @(no_return)
            @{self}.value = @{value};
        }
    }

    /// The offset to the start of the selection.
    /// This attribute only applies when the input is a text, search, url, telephone or password.
    /// 
    // https://html.spec.whatwg.org/#the-input-element:dom-textarea/input-selectionstart
    #[inline]
    pub fn selection_start( &self ) -> Option<u32> {
        js! (
            return @{self}.selectionStart;
        ).try_into().ok()
    }

    /// Sets the offset to the start of the selection.
    /// This attribute only applies when the input is a text, search, url, telephone or password.
    /// 
    // https://html.spec.whatwg.org/#the-input-element:dom-textarea/input-selectionstart
    #[inline]
    pub fn set_selection_start( &self, value: u32 ) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{self}.selectionStart = @{value};
        ).unwrap()
    }

    /// The offset to the end of the selection.
    /// This attribute only applies when the input is a text, search, url, telephone or password.
    /// 
    // https://html.spec.whatwg.org/#the-input-element:dom-textarea/input-selectionstart
    #[inline]
    pub fn selection_end( &self ) -> Option<u32> {
        js! (
            return @{self}.selectionEnd;
        ).try_into().ok()
    }

    /// Sets the offset to the end of the selection.
    /// This attribute only applies when the input is a text, search, url, telephone or password.
    /// 
    // https://html.spec.whatwg.org/#the-input-element:dom-textarea/input-selectionstart
    #[inline]
    pub fn set_selection_end( &self, value: u32 ) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{self}.selectionEnd = @{value};
        ).unwrap()
    }
}

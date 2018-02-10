use webcore::value::{Value, Reference};
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};
use webapi::file_list::FileList;

/// The HTML input element is used to create interactive controls
/// for web-based forms in order to accept data from the user.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/HTML/Element/input)
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "HTMLInputElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct InputElement( Reference );

impl IEventTarget for InputElement {}
impl INode for InputElement {}
impl IElement for InputElement {}
impl IHtmlElement for InputElement {}

impl InputElement {
    /// The value of the control. This attribute is optional except when the input is a radio button or a checkbox.
    #[inline]
    pub fn value( &self ) -> Value {
        js! (
            return @{self}.value;
        )
    }

    /// Sets the value of the control.
    #[inline]
    pub fn set_value< T: Into< Value > >( &self, value: T ) {
        js! { @(no_return)
            @{self}.value = @{value.into()};
        }
    }

    /// The type of control to render. See [Form <input> types](https://developer.mozilla.org/en/docs/Web/HTML/Element/input#Form_<input>_types)
    /// for the individual types, with links to more information about each.
    #[inline]
    pub fn set_kind( &self, kind: &str ) {
        js! { @(no_return)
            @{self}.type = @{kind};
        }
    }

    /// Returns the list of selected files. **Only for inputs of type `file`**.
    #[inline]
    pub fn files( &self ) -> Option< FileList > {
        js! (
            return @{self}.files;
        ).try_into().ok()
    }
}

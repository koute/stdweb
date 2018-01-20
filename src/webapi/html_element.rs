use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::string_map::StringMap;

/// The `IHtmlElement` interface represents any HTML element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)
pub trait IHtmlElement: IElement {
    /// Sets focus on the specified element, if it can be focused.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus)
    fn focus( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.focus();
        }
    }

    /// Removes keyboard focus from the current element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/blur)
    fn blur( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.blur();
        }
    }

    /// Allows access, both in reading and writing, to all of the custom data attributes (data-*)
    /// set on the element, either in HTML or in the DOM.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset)
    fn dataset( &self ) -> StringMap {
        unsafe {
            js!(
                return @{self.as_ref()}.dataset;
            ).into_reference_unchecked().unwrap()
        }
    }

    /// Returns the layout width of an element. Typically, an element's offsetWidth is a
    /// measurement which includes the element borders, the element horizontal padding, the
    /// element vertical scrollbar (if present, if rendered) and the element CSS width.
    fn offset_width( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.offsetWidth;
        ).try_into().unwrap()
    }

    /// Returns the height of the element including vertical padding and borders, as an
    /// integer.
    fn offset_height( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.offsetHeight;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IHtmlElement](trait.IHtmlElement.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)
pub struct HtmlElement( Reference );

impl IEventTarget for HtmlElement {}
impl INode for HtmlElement {}
impl IElement for HtmlElement {}
impl IHtmlElement for HtmlElement {}

reference_boilerplate! {
    HtmlElement,
    instanceof HTMLElement
    convertible to EventTarget
    convertible to Node
    convertible to Element
}

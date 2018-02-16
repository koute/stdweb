use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::string_map::StringMap;

/// The `IHtmlElement` interface represents any HTML element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)
// https://html.spec.whatwg.org/#htmlelement
pub trait IHtmlElement: IElement {
    /// Sets focus on the specified element, if it can be focused.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus)
    // https://html.spec.whatwg.org/#elements-in-the-dom:dom-focus
    fn focus( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.focus();
        }
    }

    /// Removes keyboard focus from the current element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/blur)
    // https://html.spec.whatwg.org/#elements-in-the-dom:dom-blur
    fn blur( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.blur();
        }
    }

    /// Allows access, both in reading and writing, to all of the custom data attributes (data-*)
    /// set on the element, either in HTML or in the DOM.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset)
    // https://html.spec.whatwg.org/#elements-in-the-dom:dom-dataset
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
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-htmlelement-offsetwidth
    fn offset_width( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.offsetWidth;
        ).try_into().unwrap()
    }

    /// Returns the height of the element including vertical padding and borders, as an
    /// integer.
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-htmlelement-offsetheight
    fn offset_height( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.offsetHeight;
        ).try_into().unwrap()
    }

    /// A property which represents the "rendered" text content of a node and its descendants.
    /// It approximates the text the user would get if they highlighted the contents of the element
    /// with the cursor and then copied to the clipboard.
    ///
    /// This feature was originally introduced by Internet Explorer, and was formally specified in the HTML
    /// standard in 2016 after being adopted by all major browser vendors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/innerText)
    // https://html.spec.whatwg.org/#elements-in-the-dom:dom-innertext
    fn inner_text( &self ) -> String {
        js!(
            return @{self.as_ref()}.innerText;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IHtmlElement](trait.IHtmlElement.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLElement")]
#[reference(subclass_of(EventTarget, Node, Element))]
pub struct HtmlElement( Reference );

impl IEventTarget for HtmlElement {}
impl INode for HtmlElement {}
impl IElement for HtmlElement {}
impl IHtmlElement for HtmlElement {}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    fn div() -> Element {
        js!(
            return document.createElement("div");
        ).try_into().unwrap()
    }

    fn text(text: &str) -> Node {
        js!(
            return new Text(@{text});
        ).try_into().unwrap()
    }

    #[test]
    fn test_inner_text() {
        let element: HtmlElement = div().try_into().unwrap();
        assert_eq!(element.inner_text(), "");
        element.append_child(&text("foo "));
        assert_eq!(element.inner_text(), "foo ");
        element.append_child(&text("foo"));
        assert_eq!(element.inner_text(), "foo foo");
    }
}

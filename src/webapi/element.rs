use std::fmt;
use std::error;
use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::token_list::TokenList;
use webapi::node_list::NodeList;

/// A structure denoting that the specified DOM [Element](trait.IElement.html) is in an invalid state.
#[derive(Debug)]
pub struct InvalidStateError( String );
impl error::Error for InvalidStateError {
    fn description( &self ) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for InvalidStateError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "{}", self.0 )
    }
}

/// The `IElement` interface represents an object of a [Document](struct.Document.html).
/// This interface describes methods and properties common to all
/// kinds of elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
pub trait IElement: IEventTarget {
    /// The Element.classList is a read-only property which returns a live
    /// [TokenList](struct.TokenList.html) collection of the class attributes
    /// of the element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)
    fn class_list( &self ) -> TokenList {
        unsafe {
            js!( return @{self.as_ref()}.classList; ).into_reference_unchecked().unwrap()
        }
    }

    /// Returns the first element that is a descendant of the element on which it is
    /// invoked that matches the specified group of selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)
    fn query_selector( &self, selector: &str ) -> Option< Element > {
        // TODO: This can throw an exception in case of an invalid selector;
        //       convert the return type to a Result.
        unsafe {
            js!( return @{self.as_ref()}.querySelector( @{selector} ); ).into_reference_unchecked()
        }
    }

    /// Returns a non-live [NodeList](struct.NodeList.html) of all elements descended
    /// from the element on which it is invoked that matches the specified group of CSS selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)
    fn query_selector_all( &self, selector: &str ) -> NodeList {
        unsafe {
            js!( return @{self.as_ref()}.querySelectorAll( @{selector} ); ).into_reference_unchecked().unwrap()
        }
    }

    /// A property which represents the inner html of a element and its descendants as a DOM string.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)
    fn inner_html( &self ) -> String {
        js!(
            return @{self.as_ref()}.innerHTML;
        ).into_string().unwrap()
    }

    /// Sets the inner html of this element; calling this removes all
    /// of node's children and replaces them with html elements
    /// of the given DOM string. If this document is an XML document and you give
    /// innerHTML an not well formed XML, this will throw an exception.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)
    fn set_inner_html( &self, text: &str ) -> Result< (), InvalidStateError > {
        let status = js! {
            try {
                @{self.as_ref()}.innerHTML = @{text};
                return true;
            } catch( exception ) {
                throw exception;
            }
        };

        if status == true {
            Ok(())
        } else {
            // as per https://www.w3.org/TR/dom/#invalidstateerror
            Err( InvalidStateError( "The object is in an invalid state.".to_owned() ) )
        }
    }
}

/// A reference to a JavaScript object which implements the [IElement](trait.IElement.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
pub struct Element( Reference );

impl IEventTarget for Element {}
impl INode for Element {}
impl IElement for Element {}

reference_boilerplate! {
    Element,
    instanceof Element
    convertible to EventTarget
    convertible to Node
}


#[cfg(web_api_tests)]
mod tests {
    use super::*;
    use webapi::document::document;

    fn div() -> Node {
        js!(
            return document.createElement("div");
        ).try_into().unwrap()
    }

    fn xml() -> Node {
        let xml_text = "<?xml version = \"1.0\"?><foo xmlns:x = \"http://foo.com\" />";
        js!(
            return new DOMParser().parseFromString(@{xml_text}, "text/xml");
        ).try_into().unwrap()
    }

    #[test]
    fn get_inner_html() {
        let parent = div();
        let child = div();
        parent.append_child(&child);
        assert_eq!(parent.inner_html(), "<div></div>");
    }

    #[test]
    fn set_inner_html() {
        let parent = div();
        parent.set_inner_html("<h1>test</h1>");
        assert_eq!(parent.first_child().unwrap().inner_html(), "test");
    }

    #[test]
    fn set_inner_html_xml_exception() {
        let doc = xml();
        let result = parent.last_child().unwrap().set_inner_html("<bar");
        assert_eq!(result.is_ok(),false);
    }
}

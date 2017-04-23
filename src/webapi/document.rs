use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::Element;
use webapi::text_node::TextNode;
use webapi::node_list::NodeList;
use webapi::location::Location;

/// The `Document` interface represents any web page loaded in the browser and
/// serves as an entry point into the web page's content, which is the DOM tree.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document)
pub struct Document( Reference );

impl IEventTarget for Document {}
impl INode for Document {}

reference_boilerplate! {
    Document,
    instanceof Document
    convertible to EventTarget
    convertible to Node
}

/// A global instance of [Document](struct.Document.html).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document)
pub fn document() -> Document {
    unsafe { js!( return document; ).into_reference_unchecked() }.unwrap()
}

impl Document {
    /// Returns the first [Element](struct.Element.html) within the document that matches the specified selector, or group of selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)
    pub fn query_selector( &self, selector: &str ) -> Option< Element > {
        // TODO: This can throw an exception in case of an invalid selector;
        //       convert the return type to a Result.
        unsafe {
            js!( return @{self}.querySelector( @{selector} ); ).into_reference_unchecked()
        }
    }

    /// Returns a list of the elements within the document (using depth-first
    /// pre-order traversal of the document's nodes) that match the
    /// specified group of selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll)
    pub fn query_selector_all( &self, selector: &str ) -> NodeList {
        unsafe {
            js!( return @{self}.querySelectorAll( @{selector} ); ).into_reference_unchecked().unwrap()
        }
    }

    /// Returns a reference to the element by its ID; the ID is a string which can
    /// be used to uniquely identify the element, found in the HTML `id` attribute.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)
    pub fn get_element_by_id( &self, id: &str ) -> Option< Element > {
        unsafe {
            js!( return @{self}.getElementById( @{id} ); ).into_reference_unchecked()
        }
    }

    /// In an HTML document, the Document.createElement() method creates the HTML
    /// element specified by `tag`, or an HTMLUnknownElement if `tag` isn't
    /// recognized. In other documents, it creates an element with a null namespace URI.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    pub fn create_element( &self, tag: &str ) -> Element {
        unsafe {
            js!( return @{self}.createElement( @{tag} ); ).into_reference_unchecked().unwrap()
        }
    }

    /// Creates a new text node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
    pub fn create_text_node( &self, text: &str ) -> TextNode {
        unsafe {
            js!( return @{self}.createTextNode( @{text} ); ).into_reference_unchecked().unwrap()
        }
    }

    /// Returns a [Location](struct.Location.html) object which contains
    /// information about the URL of the document and provides methods
    /// for changing that URL and loading another URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/location)
    pub fn location( &self ) -> Option< Location > {
        unsafe {
            js!(
                return @{self}.location;
            ).into_reference_unchecked()
        }
    }
}

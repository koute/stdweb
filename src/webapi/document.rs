use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::Element;
use webapi::text_node::TextNode;
use webapi::location::Location;
use webapi::parent_node::IParentNode;
use private::UnimplementedException;

/// The `Document` interface represents any web page loaded in the browser and
/// serves as an entry point into the web page's content, which is the DOM tree.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document)
// https://dom.spec.whatwg.org/#document
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "Document")]
#[reference(subclass_of(EventTarget, Node))]
pub struct Document( Reference );

impl IEventTarget for Document {}
impl IParentNode for Document {}
impl INode for Document {}

/// A global instance of [Document](struct.Document.html).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document)
pub fn document() -> Document {
    unsafe { js!( return document; ).into_reference_unchecked() }.unwrap()
}

impl Document {
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
    // https://dom.spec.whatwg.org/#ref-for-dom-document-createelement
    pub fn create_element( &self, tag: &str ) -> Result< Element, UnimplementedException > {
        unsafe {
            Ok( js!( return @{self}.createElement( @{tag} ); ).into_reference_unchecked().unwrap() )
        }
    }

    /// Creates a new text node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
    // https://dom.spec.whatwg.org/#ref-for-dom-document-createtextnode
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

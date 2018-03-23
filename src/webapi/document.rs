use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::Element;
use webapi::html_element::HtmlElement;
use webapi::text_node::TextNode;
use webapi::location::Location;
use webapi::parent_node::IParentNode;
use webapi::non_element_parent_node::INonElementParentNode;
use private::TODO;

/// The `Document` interface represents any web page loaded in the browser and
/// serves as an entry point into the web page's content, which is the DOM tree.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document)
// https://dom.spec.whatwg.org/#document
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Document")]
#[reference(subclass_of(EventTarget, Node))]
pub struct Document( Reference );

impl IEventTarget for Document {}
impl IParentNode for Document {}
impl INode for Document {}

impl INonElementParentNode for Document {}

/// A global instance of [Document](struct.Document.html).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document)
pub fn document() -> Document {
    unsafe { js!( return document; ).into_reference_unchecked() }.unwrap()
}

impl Document {
    /// In an HTML document, the Document.createElement() method creates the HTML
    /// element specified by `tag`, or an HTMLUnknownElement if `tag` isn't
    /// recognized. In other documents, it creates an element with a null namespace URI.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    // https://dom.spec.whatwg.org/#ref-for-dom-document-createelement
    pub fn create_element( &self, tag: &str ) -> Result< Element, TODO > {
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
    // https://html.spec.whatwg.org/#the-document-object:dom-document-location
    pub fn location( &self ) -> Option< Location > {
        unsafe {
            js!(
                return @{self}.location;
            ).into_reference_unchecked()
        }
    }

    /// Returns the <body> or <frameset> node of the current document, or null if no such element exists.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)
    // https://html.spec.whatwg.org/#the-document-object:dom-document-body
    pub fn body( &self ) -> Option< HtmlElement > {
        unsafe {
            js!(
                return @{self}.body;
            ).into_reference_unchecked()
        }
    }

    /// Returns the <head> element of the current document. If there are more than one <head>
    /// elements, the first one is returned.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/head)
    // https://html.spec.whatwg.org/#the-document-object:dom-document-head
    pub fn head( &self ) -> Option< HtmlElement > {
        unsafe {
            js!(
                return @{self}.head;
            ).into_reference_unchecked()
        }
    }

    /// Gets the title of the document.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)
    // https://html.spec.whatwg.org/multipage/semantics.html#the-title-element
    pub fn get_title( &self ) -> String {
        unsafe {
            js!(
                return @{self}.title;
            ).try_into().unwrap()
        }
    }

    /// Sets the title of the document.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)
    // https://html.spec.whatwg.org/multipage/semantics.html#the-title-element
    pub fn set_title( &self, title: &str ) -> String {
        unsafe {
            js!(
                @{self}.title = @{title};
                return @{self}.title;
            ).try_into().unwrap()
        }
    }
}

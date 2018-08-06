use webcore::value::{Reference, Value};
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::Element;
use webapi::html_element::HtmlElement;
use webapi::document_fragment::DocumentFragment;
use webapi::text_node::TextNode;
use webapi::location::Location;
use webapi::parent_node::IParentNode;
use webapi::non_element_parent_node::INonElementParentNode;
use webapi::selection::Range;
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
    /// In an HTML document, the Document.createDocumentFragment() method creates a
    /// new empty DocumentFragment.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/createDocumentFragment)
    // https://dom.spec.whatwg.org/#ref-for-dom-document-createdocumentfragment
    pub fn create_document_fragment( &self ) -> DocumentFragment {
        unsafe {
            js!( return @{self}.createDocumentFragment(); ).into_reference_unchecked().unwrap()
        }
    }

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

    /// Returns the `<body>` or `<frameset>` node of the current document, or null if no such element exists.
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

    /// Returns the `<head>` element of the current document. If there are more than one `<head>`
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
    // https://html.spec.whatwg.org/#the-document-object:document.title
    pub fn title( &self ) -> String {
        unsafe {
            js!(
                return @{self}.title;
            ).try_into().unwrap()
        }
    }

    /// Sets the title of the document.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)
    // https://html.spec.whatwg.org/#the-document-object:document.title
    pub fn set_title( &self, title: &str ) {
        unsafe {
            js!( @(no_return) @{self}.title = @{title}; );
        }
    }

    /// Returns the Element that is the root element of the document (for example, the `<html>`
    /// element for HTML documents).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement)
    // https://dom.spec.whatwg.org/#ref-for-dom-document-documentelement
    pub fn document_element( &self ) -> Option< Element > {
        unsafe {
            js!(
                return @{self}.documentElement;
            ).try_into().unwrap()
        }
    }

    /// Returns the Element that the pointer is locked to, if it is locked to any
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DocumentOrShadowRoot/pointerLockElement)
    // https://w3c.github.io/pointerlock/#dom-documentorshadowroot-pointerlockelement
    pub fn pointer_lock_element( &self ) -> Option< Element > {
        let value = js!(
            return @{self}.pointerLockElement;
        );
        match value {
            Value::Null | Value::Undefined => None,
            Value::Reference(reference) => Some(reference.try_into().unwrap()),
            _ => unreachable!()
        }
    }

    /// Exit the pointer lock on the current element
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPointerLock)
    // https://w3c.github.io/pointerlock/#dom-document-exitpointerlock
    pub fn exit_pointer_lock( &self ) {
        js!( @(no_return)
            @{self}.exitPointerLock();
        );
    }

    /// Returns a new Range object.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/createRange)
    pub fn create_range( &self ) -> Range {
        unsafe {
            js!(
                return @{self}.createRange();
            ).into_reference_unchecked().unwrap()
        }
    }

    /// The caretRangeFromPoint() method of the Document interface returns a Range object for the
    /// document fragment under the specified coordinates.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/caretRangeFromPoint)
    pub fn caret_range_from_point( &self, x: f32, y: f32) -> Option< Range > {
        unsafe {
            js!(
                return @{self}.createRangeFromPoint(@{x}, @{y});
            ).into_reference_unchecked()
        }
    }
}

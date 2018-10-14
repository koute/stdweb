use std::mem;

use webcore::value::Reference;
use webcore::try_from::{TryFrom, TryInto};
use webapi::document::Document;
use webapi::dom_exception::{HierarchyRequestError, NotFoundError, SyntaxError};
use webapi::element::Element;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node_list::NodeList;
use private::TODO;

/// An enum which determines whenever the DOM [Node](trait.INode.html)'s children will also be cloned or not.
///
/// Mainly used in [INode::clone_node](trait.INode.html#method.clone_node).
/// Also used in [Document::import_node](struct.Document.html#method.import_node).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CloneKind {
    /// Will not clone the children.
    Shallow,
    /// Will clone the children.
    Deep
}

/// `INode` is an interface from which a number of DOM API object types inherit.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node)
// https://dom.spec.whatwg.org/#node
pub trait INode: IEventTarget {
    /// Casts a reference to this object into a reference to a [Node](struct.Node.html).
    fn as_node( &self ) -> &Node {
        let reference: &Reference = self.as_ref();
        unsafe {
            mem::transmute( reference )
        }
    }

    /// Adds a node to the end of the list of children of a specified parent node.
    ///
    /// If the given child is a reference to an existing node in the document then
    /// it is moved from its current position to the new position (there is no requirement
    /// to remove the node from its parent node before appending it to some other node).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-appendchild
    fn append_child< T: INode >( &self, child: &T ) {
        js! { @(no_return)
            @{self.as_ref()}.appendChild( @{child.as_ref()} );
        }
    }

    /// Removes a child node from the DOM.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-removechild
    fn remove_child< T: INode >( &self, child: &T ) -> Result< Node, NotFoundError > {
        js_try! (
            return @{self.as_ref()}.removeChild( @{child.as_ref()} );
        ).unwrap()
    }

    /// Returns a duplicate of the node on which this method was called.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-clonenode
    fn clone_node( &self, kind: CloneKind ) -> Result< Self, TODO > {
        let is_deep = match kind {
            CloneKind::Deep => true,
            CloneKind::Shallow => false
        };

        let cloned = js! {
            return @{self.as_ref()}.cloneNode( @{is_deep} );
        };

        Ok( cloned.into_reference().unwrap().downcast::< Self >().unwrap() )
    }

    /// Checks whenever a given node is a descendant of this one or not.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-contains
    fn contains< T: INode >( &self, node: &T ) -> bool {
        js!(
            return @{self.as_ref()}.contains( @{node.as_ref()} );
        ).try_into().unwrap()
    }

    /// Inserts the specified node before the reference node as a child of the current node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-insertbefore
    fn insert_before< T: INode, U: INode >( &self, new_node: &T, reference_node: &U ) -> Result< Node, InsertNodeError > {
        js_try! (
            return @{self.as_ref()}.insertBefore( @{new_node.as_ref()}, @{reference_node.as_ref()} );
        ).unwrap()
    }

    /// Replaces one hild node of the specified node with another.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-replacechild
    fn replace_child< T: INode, U: INode >( &self, new_child: &T, old_child: &U ) -> Result< Node, InsertNodeError > {
        js_try! (
            return @{self.as_ref()}.replaceChild( @{new_child.as_ref()}, @{old_child.as_ref()} );
        ).unwrap()
    }

    /// Returns the parent of this node in the DOM tree.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-parentnode
    fn parent_node( &self ) -> Option< Node > {
        js!(
            return @{self.as_ref()}.parentNode;
        ).try_into().ok()
    }

    /// Returns the node's first child in the tree, or `None` if the node is childless.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/API/Node/firstChild)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-firstchild
    fn first_child( &self ) -> Option< Node > {
        js!(
            return @{self.as_ref()}.firstChild;
        ).try_into().ok()
    }

    /// Returns the node's last child in the tree, or `None` if the node is childless.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/API/Node/lastChild)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-lastchild
    fn last_child( &self ) -> Option< Node > {
        js!(
            return @{self.as_ref()}.lastChild;
        ).try_into().ok()
    }

    /// Returns the node's next sibling in the tree, or `None` if there isn't such a node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/API/Node/nextSibling)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-nextsibling
    fn next_sibling( &self ) -> Option< Node > {
        js!(
            return @{self.as_ref()}.nextSibling;
        ).try_into().ok()
    }

    /// Returns the name of the node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-nodename
    fn node_name( &self ) -> String {
        js!(
            return @{self.as_ref()}.nodeName;
        ).try_into().unwrap()
    }

    /// Returns the type of the node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-nodetype
    fn node_type( &self ) -> NodeType {
        match js!(
            return @{self.as_ref()}.nodeType;
        ).try_into().unwrap() {
            1 => NodeType::Element,
            2 => NodeType::Attribute,
            3 => NodeType::Text,
            4 => NodeType::CDataSection,
            7 => NodeType::ProcessingInstruction,
            8 => NodeType::Comment,
            9 => NodeType::Document,
            10 => NodeType::DocumentType,
            11 => NodeType::DocumentFragment,
            _ => unreachable!("Unexpected nodeType")
        }
    }

    /// Returns the value of the node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-nodevalue
    fn node_value( &self ) -> Option<String> {
        js!(
            return @{self.as_ref()}.nodeValue;
        ).try_into().ok()
    }

    /// Sets the value of the node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-nodevalue
    fn set_node_value( &self, value: Option< &str > ) {
        js! { @(no_return)
            @{self.as_ref()}.nodeValue = @{value};
        }
    }

    /// Returns the `Document` that this node belongs to.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-ownerdocument
    fn owner_document( &self ) -> Option< Document > {
        js!(
            return @{self.as_ref()}.ownerDocument;
        ).try_into().ok()
    }

    /// Returns an `Element` that is the parent of this node. Returns `null` if the node
    /// has no parent or the parent is not an `Element`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-parentelement
    fn parent_element( &self ) -> Option< Element > {
        js!(
            return @{self.as_ref()}.parentElement;
        ).try_into().ok()
    }

    /// Returns the node's previous sibling in the tree, or `None` if there isn't such a node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/API/Node/previousSibling)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-previoussibling
    fn previous_sibling( &self ) -> Option< Node > {
        js!(
            return @{self.as_ref()}.previousSibling;
        ).try_into().ok()
    }

    /// A property which represents the text content of a node and its descendants.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-textcontent
    fn text_content( &self ) -> Option< String > {
        js!(
            return @{self.as_ref()}.textContent;
        ).try_into().unwrap()
    }

    /// Sets the text content of this node; calling thil removes all
    /// of node's children and replaces them with a single text node
    /// with the given value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-textcontent
    fn set_text_content( &self, text: &str ) {
        js! { @(no_return)
            @{self.as_ref()}.textContent = @{text};
        }
    }

    /// Returns a live collection of child nodes of this node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-childnodes
    fn child_nodes( &self ) -> NodeList {
        unsafe {
            js!(
                return @{self.as_ref()}.childNodes;
            ).into_reference_unchecked().unwrap()
        }
    }

    /// Gets the base URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-baseuri
    fn base_uri( &self ) -> String {
        js!(
            return @{self.as_ref()}.baseURI;
        ).try_into().unwrap()
    }

    /// Returns whether this node has children nodes.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-haschildnodes
    fn has_child_nodes( &self ) -> bool {
        js!(
            return @{self.as_ref()}.hasChildNodes();
        ).try_into().unwrap()
    }

    /// Determines whether the given namespace is the default namespace of this node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-isdefaultnamespace
    fn is_default_namespace( &self, namespace: &str ) -> bool {
        js!(
            return @{self.as_ref()}.isDefaultNamespace( @{namespace} );
        ).try_into().unwrap()
    }

    /// Tests whether this node is equal to another node. Two nodes are equal if
    /// they have the same type, defining characteristics, matching attributes,
    /// and so on.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-isequalnode
    fn is_equal_node< T: INode >( &self, node: &T ) -> bool {
        js!(
            return @{self.as_ref()}.isEqualNode( @{node.as_ref()} );
        ).try_into().unwrap()
    }

    /// Test whether two `Node` references are the same.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-issamenode
    fn is_same_node< T: INode >( &self, node: &T ) -> bool {
        js!(
            return @{self.as_ref()}.isSameNode( @{node.as_ref()} );
        ).try_into().unwrap()
    }

    /// Returns the prefix for the given namespace URI, if present.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-lookupprefix
    fn lookup_prefix( &self, namespace: &str ) -> Option<String> {
        js!(
            return @{self.as_ref()}.lookupPrefix( @{namespace} );
        ).try_into().ok()
    }

    /// Returns the namespace URI for the given prefix.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-lookupnamespaceuri
    fn lookup_namespace_uri( &self, prefix: &str ) -> Option<String> {
        js!(
            return @{self.as_ref()}.lookupNamespaceURI( @{prefix} );
        ).try_into().ok()
    }

    /// Merges any adjacent text nodes and removes empty text nodes under this node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize)
    // https://dom.spec.whatwg.org/#ref-for-dom-node-normalize
    fn normalize( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.normalize();
        }
    }
}

/// Errors thrown by `Node` insertion methods.
error_enum_boilerplate! {
    InsertNodeError,
    NotFoundError, HierarchyRequestError
}

/// A reference to a JavaScript object which implements the [INode](trait.INode.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node)
// https://dom.spec.whatwg.org/#interface-node
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Node")]
#[reference(subclass_of(EventTarget))]
pub struct Node( Reference );

impl IEventTarget for Node {}
impl INode for Node {}

impl Node {
    /// Attempt to create the `Node` from raw html. The html string must contain **exactly one**
    /// root node.
    ///
    /// Returns a `SyntaxError` if:
    ///
    /// - There is not **exactly one** root node.
    /// - The html syntax is wrong. However, on most browsers the html parsing algorighm is
    ///   _unbelievably_ forgiving and will just turn your html into text or maybe even an empty
    ///   string.
    ///
    /// It is recommended to have control over the html being given to this function as not
    /// having control is a security concern.
    ///
    /// For more details, see information about setting `innerHTML`:
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML>
    pub fn from_html(html: &str) -> Result<Node, SyntaxError> {
        js_try!(
            var span = document.createElement("span");
            span.innerHTML = @{html};
            if( span.childNodes.length != 1 ) {
                throw new DOMException(
                    "Node::from_html requires a single root node but has: "
                    + span.childNodes.length,
                    "SyntaxError");
            }
            return span.childNodes[0];
        ).unwrap()
    }
}

/// Determines the type of a `Node`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    /// An `Element` such as `<p>` or `<div>`.
    Element,

    /// The actual `Text` of `Element` or `Attr`.
    Text,

    /// A `ProcessingInstruction` of an XML document.
    ProcessingInstruction,

    /// A `Comment` node.
    Comment,

    /// A 'Document' node.
    Document,

    /// A 'DocumentType' node such as `<!DOCTYPE html>`
    DocumentType,

    /// A 'DocumentFragment' node.
    DocumentFragment,

    // The following types are deprecated and should not be used.

    /// Deprecated.
    Attribute,

    /// Deprecated.
    CDataSection,

    /// Deprecated.
    XmlEntityReference,

    /// Deprecated.
    XmlEntity,

    /// Deprecated.
    XmlNotation,
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::document::document;
    use webcore::value::Value;

    fn div() -> Node {
        js!(
            return document.createElement("div");
        ).try_into().unwrap()
    }

    fn text(text: &str) -> Node {
        js!(
            return new Text(@{text});
        ).try_into().unwrap()
    }

    fn comment(text: &str) -> Node {
        js!(
            return document.createComment(@{text});
        ).try_into().unwrap()
    }

    fn processing_instruction(target: &str, data: &str) -> Node {
        js!(
            return document.createProcessingInstruction(@{target}, @{data});
        ).try_into().unwrap()
    }

    fn doc_type() -> Node {
        js!(
            return document.implementation.createDocumentType(
                "svg:svg",
                "-//W3C//DTD SVG 1.1//EN",
                "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"
            );
        ).try_into().unwrap()
    }

    fn document_fragment() -> Node {
        js!(
            return document.createDocumentFragment();
        ).try_into().unwrap()
    }

    fn xml(namespace_prefix: &str, namespace_url: &str) -> Node {
        let xml_text = format!(
            "<?xml version = \"1.0\"?><foo xmlns:{} = \"{}\" />",
            namespace_prefix,
            namespace_url
        );
        js!(
            return new DOMParser().parseFromString(@{xml_text}, "text/xml");
        ).try_into().unwrap()
    }

    #[test]
    fn test_append_child() {
        let parent = div();
        let child = div();
        parent.append_child(&child);
        assert_eq!(parent.first_child().unwrap().as_ref(), child.as_ref());
    }

    #[test]
    fn test_remove_child() {
        let parent = div();
        let child1 = div();
        let child2 = div();
        parent.append_child(&child1);
        parent.append_child(&child2);

        let removed = parent.remove_child(&child1).unwrap();
        assert_eq!(parent.first_child().unwrap().as_ref(), child2.as_ref());
        assert_eq!(removed.as_ref(), child1.as_ref());
        match parent.remove_child(&child1) {
            Err(_) => (),
            _ => panic!("Expected error")
        }

        parent.remove_child(&child2).unwrap();
        assert!(parent.first_child().is_none())
    }

    #[test]
    fn test_clone_node() {
        let node = div();
        let child = text("test");
        node.append_child(&child);

        let clone = node.clone_node(CloneKind::Shallow).unwrap();
        assert_ne!(node.as_ref(), clone.as_ref());
        assert_eq!(clone.node_name(), "DIV");
        assert!(clone.first_child().is_none());

        let clone = node.clone_node(CloneKind::Deep).unwrap();
        assert_ne!(node.as_ref(), clone.as_ref());
        assert_eq!(clone.node_name(), "DIV");
        let clone_child = clone.first_child().unwrap();
        assert_ne!(clone_child.as_ref(), child.as_ref());
        assert_eq!(&clone_child.node_value().unwrap(), "test");
    }

    #[test]
    fn test_contains() {
        let node = div();

        let child1 = div();
        node.append_child(&child1);

        let child2 = div();
        node.append_child(&child2);

        let grandchild = div();
        child1.append_child(&grandchild);

        assert!(node.contains(&node));
        assert!(node.contains(&child1));
        assert!(node.contains(&child2));
        assert!(node.contains(&grandchild));
        assert!(child1.contains(&grandchild));
        assert!(!child1.contains(&child2));
        assert!(!grandchild.contains(&node));
    }

    #[test]
    fn test_insert_before() {
        let node = div();
        let child1 = div();
        let child2 = div();
        let child3 = div();
        node.append_child(&child1);
        node.insert_before(&child2, &child1).unwrap();
        assert_eq!(node.first_child().unwrap().as_ref(), child2.as_ref());

        match node.insert_before(&child3, &child3) {
            Err(InsertNodeError::NotFoundError(_)) => (),
            _ => panic!("Expected NotFoundError")
        }

        match node.insert_before(&doc_type(), &child1) {
            Err(InsertNodeError::HierarchyRequestError(_)) => (),
            _ => panic!("Expected HierarchyRequestError")
        }
    }

    #[test]
    fn test_replace_child() {
        let node = div();
        let child1 = div();
        let child2 = div();
        node.append_child(&child1);
        node.replace_child(&child2, &child1).unwrap();
        assert_eq!(node.first_child().unwrap().as_ref(), child2.as_ref());
        assert!(child1.parent_node().is_none());

        match node.replace_child(&child2, &child1) {
            Err(InsertNodeError::NotFoundError(_)) => (),
            _ => panic!("Expected NotFoundError")
        }

        match node.replace_child(&doc_type(), &child2) {
            Err(InsertNodeError::HierarchyRequestError(_)) => (),
            _ => panic!("Expected HierarchyRequestError")
        }
    }

    #[test]
    fn test_parent_node() {
        let node = div();
        let child = div();
        node.append_child(&child);
        assert!(node.parent_node().is_none());
        assert_eq!(child.parent_node().unwrap().as_ref(), node.as_ref());
    }

    #[test]
    fn test_first_child() {
        let node = div();
        assert!(node.first_child().is_none());

        let child = div();
        node.append_child(&child);
        assert_eq!(node.first_child().unwrap().as_ref(), child.as_ref());
    }

    #[test]
    fn test_last_child() {
        let node = div();
        assert!(node.last_child().is_none());

        let child1 = div();
        node.append_child(&child1);
        assert_eq!(node.last_child().unwrap().as_ref(), child1.as_ref());

        let child2 = div();
        node.append_child(&child2);
        assert_eq!(node.last_child().unwrap().as_ref(), child2.as_ref());
    }

    #[test]
    fn test_next_sibling() {
        let node = div();
        let child1 = div();
        node.append_child(&child1);
        assert!(child1.next_sibling().is_none());

        let child2 = div();
        node.append_child(&child2);
        assert_eq!(child1.next_sibling().unwrap().as_ref(), child2.as_ref());
    }

    #[test]
    fn test_previous_sibling() {
        let node = div();
        let child1 = div();
        let child2 = div();

        node.append_child(&child1);
        assert!(child1.previous_sibling().is_none());
        node.append_child(&child2);
        assert_eq!(child2.previous_sibling().unwrap().as_ref(), child1.as_ref());
    }

    #[test]
    fn test_node_name() {
        assert_eq!(div().node_name(), "DIV");
        assert_eq!(text("x").node_name(), "#text");
        assert_eq!(document_fragment().node_name(), "#document-fragment");
        assert_eq!(doc_type().node_name(), "svg:svg");
        assert_eq!(processing_instruction("foo", "bar").node_name(), "foo");
    }

    #[test]
    fn test_node_type() {
        assert_eq!(div().node_type(), NodeType::Element);
        assert_eq!(text("x").node_type(), NodeType::Text);
        assert_eq!(processing_instruction("foo", "bar").node_type(), NodeType::ProcessingInstruction);
        assert_eq!(comment("foo").node_type(), NodeType::Comment);
        assert_eq!(document().node_type(), NodeType::Document);
        assert_eq!(doc_type().node_type(), NodeType::DocumentType);
        assert_eq!(document_fragment().node_type(), NodeType::DocumentFragment);
    }

    #[test]
    fn test_node_value() {
        let node = text("x");
        assert_eq!(node.node_value().unwrap(), "x");
        node.set_node_value(Some("y"));
        assert_eq!(node.node_value().unwrap(), "y");

        assert_eq!(processing_instruction("foo", "bar").node_value().unwrap(), "bar");
        assert_eq!(comment("foo").node_value().unwrap(), "foo");

        let node: Node = div();
        assert!(node.node_value().is_none());
        node.set_node_value(Some("foo"));
        assert!(node.node_value().is_none());

        assert!(document().node_value().is_none());
        assert!(doc_type().node_value().is_none());
        assert!(document_fragment().node_value().is_none());
    }

    #[test]
    fn test_owner_document() {
        let node = div();
        assert_eq!(node.owner_document().unwrap().as_ref(), document().as_ref());
    }

    #[test]
    fn test_parent_element() {
        let node = div();
        let child = div();
        node.append_child(&child);
        assert_eq!(child.parent_element().unwrap().as_ref(), node.as_ref());
    }

    #[test]
    fn test_text_content() {
        let node: Node = div();
        assert_eq!(node.text_content().unwrap(), "");
        node.append_child(&text("foo "));
        assert_eq!(node.text_content().unwrap(), "foo ");
        node.append_child(&text("foo"));
        assert_eq!(node.text_content().unwrap(), "foo foo");
        node.set_text_content("bar");
        assert_eq!(node.text_content().unwrap(), "bar");
        assert_eq!(node.child_nodes().len(), 1);
    }

    #[test]
    fn test_base_uri() {
        let node = div();
        assert!(!node.base_uri().is_empty());
    }

    #[test]
    fn test_has_child_nodes() {
        let node = div();
        assert!(!node.has_child_nodes());
        node.append_child(&div());
        assert!(node.has_child_nodes());
    }

    #[test]
    fn test_child_nodes() {
        let node = div();
        let node_list = node.child_nodes();
        assert_eq!(node_list.len(), 0);
        assert!(node_list.iter().next().is_none());

        let child1 = text("foo");
        node.append_child(&child1);
        let child2 = text("bar");
        node.append_child(&child2);

        let node_list = node.child_nodes();
        assert_eq!(node_list.len(), 2);
        let mut iter = node_list.iter();
        assert_eq!(iter.next().unwrap().as_ref(), child1.as_ref());
        assert_eq!(iter.next().unwrap().as_ref(), child2.as_ref());
    }

    #[test]
    fn test_is_default_namespace() {
        assert!(!div().is_default_namespace("foo"));
        assert!(div().is_default_namespace("http://www.w3.org/1999/xhtml"));
    }

    #[test]
    fn test_is_equal_node() {
        let node1 = div();
        let node2 = div();
        assert!(node1.is_equal_node(&node2));

        let child1 = div();
        node1.append_child(&child1);
        assert!(!node1.is_equal_node(&node2));

        let child2 = div();
        node2.append_child(&child2);
        assert!(node1.is_equal_node(&node2));
    }

    #[test]
    fn test_is_same_node() {
        let node1 = div();
        assert!(node1.is_same_node(&node1));
        assert!(!node1.is_same_node(&div()));
    }

    #[test]
    fn test_lookup_prefix() {
        let xml = xml("x", "http://foo.com");
        assert!(xml.lookup_prefix("bar").is_none());
        assert_eq!(xml.lookup_prefix("http://foo.com").unwrap(), "x");
    }

    #[test]
    fn test_lookup_namespace_uri() {
        let xml = xml("x", "http://foo.com");
        assert!(xml.lookup_namespace_uri("y").is_none());
        assert_eq!(xml.lookup_namespace_uri("x").unwrap(), "http://foo.com");
    }

    #[test]
    fn test_normalize() {
        let node = div();
        node.append_child(&text("test "));
        node.append_child(&text("123"));
        node.normalize();
        assert_eq!(node.child_nodes().len(), 1);
        let child_text = node.first_child().unwrap().text_content().unwrap();
        assert_eq!(child_text, "test 123");
    }

    #[test]
    fn option_node_is_constructible_from_value() {
        let node: Value = js!( return document.createElement( "div" ) );
        let opt_node: Option< Node > = node.clone().try_into().unwrap();
        assert_eq!( opt_node.unwrap().as_ref(), node.as_ref() );
    }

    #[test]
    fn empty_option_node_is_constructible_from_null_value() {
        let empty_opt_node: Option< Node > = Value::Null.try_into().unwrap();
        assert!( empty_opt_node.is_none() );
    }

    #[test]
    fn empty_option_node_is_constructible_from_undefined_value() {
        let empty_opt_node: Option< Node > = Value::Undefined.try_into().unwrap();
        assert!( empty_opt_node.is_none() );
    }

    #[test]
    fn option_node_from_numeric_value_results_in_an_error() {
        let value: Value = 123_i32.into();
        let empty_opt_node: Result< Option< Node >, _ > = value.try_into();
        assert!( empty_opt_node.is_err() );
    }

    #[test]
    fn from_html() {
        let node = Node::from_html("<div>Some text, horray!</div>").unwrap();
        let text = node.first_child().unwrap();

        assert_eq!(node.node_name(), "DIV");
        assert_eq!(node.last_child().unwrap(), text);

        assert_eq!(text.node_name(), "#text");
        assert_eq!(text.node_value().unwrap(), "Some text, horray!");
        assert!(text.first_child().is_none());

        let err = Node::from_html("<div>foo</div><div>bar</div>").unwrap_err();
        assert!(format!("{}", err).contains("requires a single root node"));
        assert!(Node::from_html("<di").is_err());
    }
}

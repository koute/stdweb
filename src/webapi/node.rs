use std::fmt;
use std::error;
use std::mem;

use webcore::value::{Reference, FromReference};
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node_list::NodeList;

/// A structure denoting that the specified DOM [Node](trait.INode.html) was not found.
#[derive(Debug)]
pub struct NotFoundError( String );
impl error::Error for NotFoundError {
    fn description( &self ) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for NotFoundError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "{}", self.0 )
    }
}

/// An enum which determines whenever the DOM [Node](trait.INode.html)'s children will also be cloned or not.
///
/// Mainly used in [INode::clone_node](trait.INode.html#method.clone_node).
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
pub trait INode: IEventTarget + FromReference {
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
    fn append_child< T: INode >( &self, child: &T ) {
        js! { @(no_return)
            @{self.as_ref()}.appendChild( @{child.as_ref()} );
        }
    }

    /// Removes a child node from the DOM.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)
    fn remove_child< T: INode >( &self, child: &T ) -> Result< (), NotFoundError > {
        // TODO: Return the removed node.
        let status = js! {
            try {
                @{self.as_ref()}.removeChild( @{child.as_ref()} );
                return true;
            } catch( exception ) {
                if( exception instanceof NotFoundError ) {
                    return false;
                } else {
                    throw exception;
                }
            }
        };

        if status == true {
            Ok(())
        } else {
            Err( NotFoundError( "The node to be removed is not a child of this node.".to_owned() ) )
        }
    }

    /// Returns a duplicate of the node on which this method was called.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)
    fn clone_node( &self, kind: CloneKind ) -> Self {
        let is_deep = match kind {
            CloneKind::Deep => true,
            CloneKind::Shallow => false
        };

        let cloned = js! {
            return @{self.as_ref()}.cloneNode( @{is_deep} );
        };

        cloned.into_reference().unwrap().downcast::< Self >().unwrap()
    }

    /// Checks whenever a given node is a descendant of this one or not.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)
    fn contains< T: INode >( &self, node: &T ) -> bool {
        js!(
            return @{self.as_ref()}.contains( @{node.as_ref()} );
        ).try_into().unwrap()
    }

    /// Inserts the specified node before the reference node as a child of the current node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)
    fn insert_before< T: INode, U: INode >( &self, new_node: &T, reference_node: &U ) {
        js! { @(no_return)
            @{self.as_ref()}.insertBefore( @{new_node.as_ref()}, @{reference_node.as_ref()} );
        }
    }

    /// Replaces one hild node of the specified node with another.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)
    fn replace_child< T: INode, U: INode >( &self, new_child: &T, old_child: &U ) {
        js! { @(no_return)
            @{self.as_ref()}.replaceChild( @{new_child.as_ref()}, @{old_child.as_ref()} );
        }
    }

    /// Returns the parent of this node in the DOM tree.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)
    fn parent_node( &self ) -> Option< Node > {
        js!(
            return @{self.as_ref()}.parentNode;
        ).try_into().ok()
    }

    /// Returns the node's first child in the tree, or `None` if the node is childless.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/API/Node/firstChild)
    fn first_child( &self ) -> Option< Node > {
        js!(
            return @{self.as_ref()}.firstChild;
        ).try_into().ok()
    }

    /// A property which represents the "rendered" text content of a node and its descendants.
    /// It approximates the text the user would get if they highlighted the contents of the element
    /// with the cursor and then copied to the clipboard.
    ///
    /// This feature was originally introduced by Internet Explorer, and was formally specified in the HTML
    /// standard in 2016 after being adopted by all major browser vendors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/innerText)
    fn inner_text( &self ) -> String {
        js!(
            return @{self.as_ref()}.innerText;
        ).try_into().unwrap()
    }

    /// A property which represents the text content of a node and its descendants.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)
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
    fn set_text_content( &self, text: &str ) {
        js! { @(no_return)
            @{self.as_ref()}.textContent = @{text};
        }
    }

    /// Returns a live collection of child nodes of this node.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)
    fn child_nodes( &self ) -> NodeList {
        unsafe {
            js!(
                return @{self.as_ref()}.childNodes;
            ).into_reference_unchecked().unwrap()
        }
    }
}

/// A reference to a JavaScript object which implements the [INode](trait.INode.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Node)
pub struct Node( Reference );

impl IEventTarget for Node {}
impl INode for Node {}

reference_boilerplate! {
    Node,
    instanceof Node
    convertible to EventTarget
}

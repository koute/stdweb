use webcore::value::{Value, Reference};
use webcore::try_from::TryInto;
use webcore::reference_type::ReferenceType;
use webapi::node::Node;

/// `NodeList` objects are collections of nodes such as those returned by properties
/// such as [INode::child_nodes](trait.INode.html#method.child_nodes) and the
/// [Document::query_selector_all](struct.Document.html#method.query_selector_all) method.
///
/// In some cases, the `NodeList` is a live collection, which means that changes in the DOM
/// are reflected in the collection - for example [INode::child_nodes](trait.INode.html#method.child_nodes) is live.
///
/// In other cases, the `NodeList` is a static collection, meaning any subsequent change
/// in the DOM does not affect the content of the collection - for example
/// [Document::query_selector_all](struct.Document.html#method.query_selector_all) returns
/// a static `NodeList`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)
// https://dom.spec.whatwg.org/#nodelist
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "NodeList")]
pub struct NodeList( Reference );

impl NodeList {
    /// Returns the number of [Node](struct.Node.html)s contained in this list.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/length)
    // https://dom.spec.whatwg.org/#ref-for-dom-nodelist-length
    pub fn len( &self ) -> u32 {
        js!( return @{self}.length; ).try_into().unwrap()
    }

    /// Returns a node from a NodeList by index.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/item)
    // https://dom.spec.whatwg.org/#ref-for-dom-nodelist-item
    pub fn item( &self, index: u32 ) -> Option< Node > {
        js!(
            return @{self}[ @{index} ];
        ).try_into().unwrap()
    }

    /// Returns an iterator over the list.
    pub fn iter( &self ) -> NodeIter {
        NodeIter {
            list: self.clone(),
            index: 0
        }
    }
}

impl IntoIterator for NodeList {
    type Item = Node;
    type IntoIter = NodeIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        NodeIter {
            list: self,
            index: 0
        }
    }
}

impl< 'a > IntoIterator for &'a NodeList {
    type Item = Node;
    type IntoIter = NodeIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        NodeIter {
            list: self.clone(),
            index: 0
        }
    }
}

#[derive(Debug)]
pub struct NodeIter {
    list: NodeList,
    index: i32
}

impl Iterator for NodeIter {
    type Item = Node;
    fn next( &mut self ) -> Option< Self::Item > {
        let value = js!(
            return @{&self.list}[ @{self.index} ];
        );

        let node = match value {
            Value::Undefined => return None,
            Value::Reference( reference ) => unsafe { Node::from_reference_unchecked( reference ) },
            _ => unreachable!()
        };

        self.index += 1;
        Some( node )
    }
}

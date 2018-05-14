use webcore::reference_type::ReferenceType;

/// The `ChildNode` interface contains methods that are particular to `Node`
/// objects that can have a parent.
///
/// You most likely don't want to `use` this directly; instead
/// you should `use stdweb::traits::*;`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ChildNode)
// https://dom.spec.whatwg.org/#interface-childnode
pub trait IChildNode: ReferenceType {
    /// The `ChildNode.remove()` method removes the object from the tree it belongs to.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ChildNode/remove)
    // https://dom.spec.whatwg.org/#ref-for-dom-childnode-remove
    fn remove( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.remove();
        };
    }
}

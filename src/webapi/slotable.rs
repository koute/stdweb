use webcore::reference_type::ReferenceType;
use webapi::html_elements::SlotElement;

/// The Slotable mixin defines features that allow nodes to become the contents of
/// a `<slot>` element — the following features are included in both Element and Text.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Slotable)
// https://dom.spec.whatwg.org/#slotable
pub trait ISlotable: ReferenceType {
    /// returns a `SlotElement` representing the `<slot>` element the node is inserted in.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Slotable/assignedSlot)
    // https://dom.spec.whatwg.org/#ref-for-dom-slotable-assignedslot
    fn assigned_slot( &self ) -> Option< SlotElement > {
        unsafe {
            js!( return @{self.as_ref()}.assignedSlot; ).into_reference_unchecked()
        }
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::node::{Node, INode, CloneKind};
    use webapi::parent_node::IParentNode;
    use webapi::html_elements::{SlotElement, TemplateElement};
    use webapi::shadow_root::ShadowRootMode;
    use webapi::html_element::HtmlElement;
    use webcore::try_from::TryInto;
    use webapi::element::IElement;

    #[test]
    fn test_assigned_slot() {
        let div: HtmlElement = Node::from_html("<div><span></span></div>")
            .unwrap()
            .try_into()
            .unwrap();
        let span = div.query_selector("span").unwrap().unwrap();
        let tpl: TemplateElement = Node::from_html("<template><slot></slot></template>")
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(span.assigned_slot(), None);

        let shadow_root = div.attach_shadow(ShadowRootMode::Open).unwrap();
        let n = tpl.content().clone_node(CloneKind::Deep).unwrap();
        shadow_root.append_child(&n);

        let slot: SlotElement = shadow_root
            .query_selector("slot")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(span.assigned_slot(), Some(slot));
    }
}

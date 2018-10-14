use webcore::reference_type::ReferenceType;
use webapi::html_elements::SlotElement;

/// The Slotable mixin defines features that allow nodes to become the contents of
/// a <slot> element — the following features are included in both Element and Text.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Slotable)
// https://dom.spec.whatwg.org/#slotable
pub trait ISlotable: ReferenceType {
    /// returns an HTMLSlotElement representing the <slot> element the node is inserted in.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Slotable/assignedSlot)
    // https://dom.spec.whatwg.org/#dom-slotable-assignedslot
    fn assigned_slot( &self ) -> Option< SlotElement > {
        unsafe {
            js!( return @{self.as_ref()}.assignedSlot; ).into_reference_unchecked()
        }
    }
}

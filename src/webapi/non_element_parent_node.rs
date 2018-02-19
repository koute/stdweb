use webcore::reference_type::ReferenceType;
use webapi::element::Element;

/// The `INonElementParentNode` mixin contains methods and properties
/// that are common to `Document` and `DocumentFragment`.
///
/// You most likely don't want to `use` this directly; instead
/// you should `use stdweb::traits::*;`.
// https://dom.spec.whatwg.org/#nonelementparentnode
pub trait INonElementParentNode: ReferenceType {
    /// Returns a reference to the element by its ID; the ID is a string which can
    /// be used to uniquely identify the element, found in the HTML `id` attribute.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)
    // https://dom.spec.whatwg.org/#ref-for-dom-nonelementparentnode-getelementbyid
    fn get_element_by_id( &self, id: &str ) -> Option< Element > {
        unsafe {
            js!( return @{self.as_ref()}.getElementById( @{id} ); ).into_reference_unchecked()
        }
    }
}

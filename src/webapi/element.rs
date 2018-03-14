use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::dom_exception::{InvalidCharacterError, InvalidPointerId};
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::token_list::TokenList;
use webapi::parent_node::IParentNode;

/// The `IElement` interface represents an object of a [Document](struct.Document.html).
/// This interface describes methods and properties common to all
/// kinds of elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
// https://dom.spec.whatwg.org/#element
pub trait IElement: INode + IParentNode {
    /// The Element.classList is a read-only property which returns a live
    /// [TokenList](struct.TokenList.html) collection of the class attributes
    /// of the element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-classlist
    fn class_list( &self ) -> TokenList {
        unsafe {
            js!( return @{self.as_ref()}.classList; ).into_reference_unchecked().unwrap()
        }
    }

    /// The Element.hasAttribute() method returns a Boolean value indicating whether
    /// the specified element has the specified attribute or not.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-hasattribute
    fn has_attribute( &self, name: &str ) -> bool {
        js!(
            return @{self.as_ref()}.hasAttribute( @{name} );
        ).try_into().unwrap()
    }

    /// Element.getAttribute() returns the value of a specified attribute on the element.
    /// If the given attribute does not exist, the value returned will either be
    /// null or "" (the empty string);
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-getattribute
    fn get_attribute( &self, name: &str ) -> Option< String > {
        js!(
            return @{self.as_ref()}.getAttribute( @{name} );
        ).try_into().unwrap()
    }

    /// Sets the value of an attribute on the specified element. If the attribute already
    /// exists, the value is updated; otherwise a new attribute is added with the
    /// specified name and value.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-setattribute
    fn set_attribute( &self, name: &str, value: &str ) -> Result< (), InvalidCharacterError > {
        js_try!(
            return @{self.as_ref()}.setAttribute( @{name}, @{value} );
        ).unwrap()
    }

    /// Element.getAttributeNames() returns the attribute names of the element
    /// as an Array of strings. If the element has no attributes it returns an empty array.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-getattributenames
    fn get_attribute_names( &self ) -> Vec<String> {
        js!(
            return @{self.as_ref()}.getAttributeNames();
        ).try_into().unwrap()
    }

    /// Element.removeAttribute removes an attribute from the specified element.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-removeattribute
    fn remove_attribute( &self, name: &str ) {
        js! { @(no_return)
            @{self.as_ref()}.removeAttribute( @{name} );
        }
    }

    /// The Element.hasAttributes() method returns Boolean value, indicating if
    /// the current element has any attributes or not.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-hasattributes
    fn has_attributes( &self ) -> bool {
        js!(
            return @{self.as_ref()}.hasAttributes();
        ).try_into().unwrap()
    }

    /// Designates a specific element as the capture target of future pointer events.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture)
    // https://w3c.github.io/pointerevents/#dom-element-setpointercapture
    #[inline]
    fn set_pointer_capture( &self, pointer_id: i32 ) -> Result< (), InvalidPointerId > {
        js_try!(
            return @{self.as_ref()}.setPointerCapture( @{pointer_id} );
        ).unwrap()
    }

    /// Releases pointer capture that was previously set for a specific pointer
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture)
    // https://w3c.github.io/pointerevents/#dom-element-releasepointercapture
    #[inline]
    fn release_pointer_capture( &self, pointer_id: i32 ) -> Result< (), InvalidPointerId > {
        js_try!(
            return @{self.as_ref()}.releasePointerCapture( @{pointer_id} );
        ).unwrap()
    }

    /// Returns a boolean indicating if the element has captured the specified pointer
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture)
    // https://w3c.github.io/pointerevents/#dom-element-haspointercapture
    #[inline]
    fn has_pointer_capture( &self, pointer_id: i32 ) -> bool {
        js!( return @{self.as_ref()}.hasPointerCapture( @{pointer_id} ); ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IElement](trait.IElement.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
// https://dom.spec.whatwg.org/#element
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Element")]
#[reference(subclass_of(EventTarget, Node))]
pub struct Element( Reference );

impl IEventTarget for Element {}
impl INode for Element {}
impl IElement for Element {}

impl< T: IElement > IParentNode for T {}

use webcore::value::Reference;
use webcore::try_from::{TryFrom, TryInto};
use webcore::promise::{Promise, TypedPromise};
use webapi::error::TypeError;
use webapi::dom_exception::{InvalidCharacterError, InvalidPointerId, NoModificationAllowedError, SyntaxError};
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::token_list::TokenList;
use webapi::parent_node::IParentNode;
use webapi::child_node::IChildNode;
use webapi::slotable::ISlotable;
use webapi::shadow_root::{ShadowRootMode, ShadowRoot};
use webapi::dom_exception::{NotSupportedError, InvalidStateError};

error_enum_boilerplate! {
    AttachShadowError,
    NotSupportedError, InvalidStateError
}

/// The `IElement` interface represents an object of a [Document](struct.Document.html).
/// This interface describes methods and properties common to all
/// kinds of elements.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element)
// https://dom.spec.whatwg.org/#element
pub trait IElement: INode + IParentNode + IChildNode + ISlotable {
    /// The Element.namespaceURI read-only property returns the namespace URI
    /// of the element, or null if the element is not in a namespace.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-namespaceuri
    fn namespace_uri( &self ) -> Option< String > {
        js!(
            return @{self.as_ref()}.namespaceURI;
        ).try_into().unwrap()
    }

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

    /// Gets the the number of pixels that an element's content is scrolled vertically.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-element-scrolltop%E2%91%A0
    fn scroll_top( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.scrollTop;
        ).try_into().unwrap()
    }

    /// Sets the the number of pixels that an element's content is scrolled vertically.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-element-scrolltop%E2%91%A0
    fn set_scroll_top( &self, value: f64 ) {
        js! { @(no_return)
            @{self.as_ref()}.scrollTop = @{value};
        }
    }

    /// Gets the the number of pixels that an element's content is scrolled to the left.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-element-scrollleft%E2%91%A0
    fn scroll_left( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.scrollLeft;
        ).try_into().unwrap()
    }

    /// Sets the the number of pixels that an element's content is scrolled to the left.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-element-scrollleft%E2%91%A0
    fn set_scroll_left( &self, value: f64 ) {
        js! { @(no_return)
            @{self.as_ref()}.scrollLeft = @{value};
        }
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

    /// Returns the closest ancestor of the element (or the element itself) which matches the selectors 
    /// given in parameter. If there isn't such an ancestor, it returns
    /// `None`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-closest
    fn closest( &self, selectors: &str) -> Result<Option<Element>, SyntaxError> {
        js_try!(
            return @{self.as_ref()}.closest(@{selectors});
        ).unwrap()
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

    /// Insert nodes from HTML fragment into specified position.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
    // https://w3c.github.io/DOM-Parsing/#widl-Element-insertAdjacentHTML-void-DOMString-position-DOMString-text
    fn insert_adjacent_html( &self, position: InsertPosition, html: &str ) -> Result<(), InsertAdjacentError> {
        js_try!( @(no_return)
            @{self.as_ref()}.insertAdjacentHTML( @{position.as_str()}, @{html} );
        ).unwrap()
    }

    /// Insert nodes from HTML fragment before element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
    fn insert_html_before( &self, html: &str ) -> Result<(), InsertAdjacentError> {
        self.insert_adjacent_html(InsertPosition::BeforeBegin, html)
    }

    /// Insert nodes from HTML fragment as the first children of the element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
    fn prepend_html( &self, html: &str ) -> Result<(), InsertAdjacentError> {
        self.insert_adjacent_html(InsertPosition::AfterBegin, html)
    }

    /// Insert nodes from HTML fragment as the last children of the element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
    fn append_html( &self, html: &str ) -> Result<(), InsertAdjacentError> {
        self.insert_adjacent_html(InsertPosition::BeforeEnd, html)
    }

    /// Insert nodes from HTML fragment after element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)
    fn insert_html_after( &self, html: &str ) -> Result<(), InsertAdjacentError> {
        self.insert_adjacent_html(InsertPosition::AfterEnd, html)
    }

    /// The slot property of the Element interface returns the name of the shadow DOM
    /// slot the element is inserted in.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-slot
    fn slot( &self ) -> String {
        js!(
            return @{self.as_ref()}.slot;
        ).try_into().unwrap()
    }

    /// Attach a shadow DOM tree to the specified element and returns a reference to its `ShadowRoot`.
    /// It returns a shadow root if successfully attached or `None` if the element cannot be attached.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-attachshadow
    fn attach_shadow( &self, mode: ShadowRootMode ) -> Result<ShadowRoot, AttachShadowError> {
        js_try!(
            return @{self.as_ref()}.attachShadow( { mode: @{mode.as_str()}} )
        ).unwrap()
    }

    /// Returns the shadow root of the current element or `None`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot)
    // https://dom.spec.whatwg.org/#ref-for-dom-element-shadowroot
    fn shadow_root( &self ) -> Option<ShadowRoot> {
        unsafe {
            js!(
                return @{self.as_ref()}.shadowRoot;
            ).into_reference_unchecked()
        }
    }

    /// Request this element and its children be made fullscreen
    ///
    /// Note: this may only be called during a user interaction.
    /// Not all elements may be full-screened, see JS docs for details.
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)
    // https://fullscreen.spec.whatwg.org/#ref-for-dom-element-requestfullscreen
    #[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
    fn request_fullscreen( &self ) -> TypedPromise<(), TypeError> {
        let promise: Promise = js!( return @{self.as_ref()}.requestFullscreen(); )
            .try_into().unwrap();

        TypedPromise::new( promise )
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
impl< T: IElement > IChildNode for T {}
impl< T: IElement > ISlotable for T {}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InsertPosition {
    /// Insert into the parent directly before the reference element.
    BeforeBegin,
    /// Insert at the start of the reference element.
    AfterBegin,
    /// Insert at the end of the reference element.
    BeforeEnd,
    /// Insert into the parent directly after the reference element.
    AfterEnd,
}

/// Errors thrown by `Element::insert_adjacent_html`.
error_enum_boilerplate! {
    InsertAdjacentError,
    NoModificationAllowedError, SyntaxError
}

impl InsertPosition {
    fn as_str(&self) -> &str {
        match *self {
            InsertPosition::BeforeBegin => "beforebegin",
            InsertPosition::AfterBegin => "afterbegin",
            InsertPosition::BeforeEnd => "beforeend",
            InsertPosition::AfterEnd => "afterend",
        }
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::document::document;
    use webapi::shadow_root::ShadowRootMode;

    fn div() -> Element {
        js!(
            return document.createElement("div");
        ).try_into().unwrap()
    }

    fn h1() -> Element {
        js!(
            return document.createElement("h1");
        ).try_into().unwrap()
    }

    #[test]
    fn test_closest_finds_ancestor() {
        let parent = div();
        let child = h1();
        parent.append_child(&child);

        assert_eq!(child.closest("div").unwrap().unwrap().as_ref(), parent.as_ref());
    }

    #[test]
    fn test_closest_not_found() {
        let parent = div();
        let child = h1();
        parent.append_child(&child);

        assert!(child.closest("p").unwrap().is_none());
    }

    #[test]
    fn test_closest_syntax_error() {
        let parent = div();
        let child = div();
        parent.append_child(&child);

        assert!(child.closest("invalid syntax +#8$()@!(#").is_err());
    }

    #[test]
    fn insert_adjacent_html() {
        let root = document().create_element("div").unwrap();
        let child = document().create_element("span").unwrap();
        child.set_text_content("child");
        root.append_child(&child);

        child.insert_html_before(" <button>before begin</button> foo ").unwrap();
        child.prepend_html("<i>afterbegin").unwrap();
        child.append_html("<h1> Before end</h1>").unwrap();
        child.insert_html_after("after end ").unwrap();

        let html = js!(return @{root}.innerHTML);
        assert_eq!(html, " <button>before begin</button> foo <span><i>afterbegin</i>child<h1> Before end</h1></span>after end ");
    }

    #[test]
    fn insert_adjacent_html_empty() {
        let root = document().create_element("div").unwrap();
        root.append_html("").unwrap();

        let html = js!(return @{root}.innerHTML);
        assert_eq!(html, "");
    }

    #[test]
    fn insert_adjacent_html_not_modifiable() {
        let doc = document().document_element().unwrap();
        assert!(match doc.insert_html_before("foobar").unwrap_err() {
            InsertAdjacentError::NoModificationAllowedError(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_attach_shadow_mode_open() {
        let element = document().create_element("div").unwrap();
        let shadow_root = element.attach_shadow(ShadowRootMode::Open).unwrap();
        assert_eq!(shadow_root.mode(), ShadowRootMode::Open);
        assert_eq!(element.shadow_root(), Some(shadow_root));
    }

    #[test]
    fn test_attach_shadow_mode_closed() {
        let element = document().create_element("div").unwrap();
        let shadow_root = element.attach_shadow(ShadowRootMode::Closed).unwrap();
        assert_eq!(shadow_root.mode(), ShadowRootMode::Closed);
        assert!(element.shadow_root().is_none());
    }
}

use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::reference_type::ReferenceType;
use webapi::element::Element;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::document_fragment::DocumentFragment;

/// The mode associated to a shadow root.
/// Mainly used in [IElement::attach_shadow](trait.IElement.html#method.attach_shadow) and
/// [IShadowRoot::mode](trait.IShadowRoot.html#method.mode).
// https://dom.spec.whatwg.org/#shadowroot-mode
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShadowRootMode {
    Open,
    Closed,
}

impl ShadowRootMode {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ShadowRootMode::Open => "open",
            ShadowRootMode::Closed => "closed",
        }
    }
}

/// The `ShadowRoot` interface of the Shadow DOM API is the root node of a DOM
/// subtree that is rendered separately from a document's main DOM tree.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)
// https://dom.spec.whatwg.org/#interface-shadowroot
pub trait IShadowRoot: ReferenceType {
    /// The mode property of the `ShadowRoot` specifies its mode.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode)
    // https://dom.spec.whatwg.org/#ref-for-dom-shadowroot-mode
    fn mode( &self ) -> ShadowRootMode {
        let mode_string: String = js!( return @{self.as_ref()}.mode; ).try_into().unwrap();

        match mode_string.as_str() {
            "open" => ShadowRootMode::Open,
            "closed" => ShadowRootMode::Closed,
            _ => unreachable!("mode can only be `open` or `closed`"),
        }
    }

    /// The host read-only property of the `ShadowRoot` returns a reference to the DOM element
    /// the ShadowRoot is attached to.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host)
    // https://dom.spec.whatwg.org/#ref-for-dom-shadowroot-host
    fn host( &self ) -> Element {
        js!( return @{self.as_ref()}.host; ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IShadowRoot](trait.IShadowRoot.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)
// https://dom.spec.whatwg.org/#interface-shadowroot
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ShadowRoot")]
#[reference(subclass_of(EventTarget, Node, DocumentFragment))]
pub struct ShadowRoot( Reference );

impl IEventTarget for ShadowRoot {}
impl INode for ShadowRoot {}
impl IShadowRoot for ShadowRoot {}

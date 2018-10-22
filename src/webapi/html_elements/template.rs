use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};
use webapi::document_fragment::DocumentFragment;

/// The HTML `<template>` element represents a mechanism for holding client-side content
/// that is not to be rendered when a page is loaded but may subsequently be instantiated
/// during runtime using JavaScript.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
// https://html.spec.whatwg.org/multipage/scripting.html#the-template-element
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLTemplateElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct TemplateElement( Reference );

impl IEventTarget for TemplateElement {}
impl INode for TemplateElement {}
impl IElement for TemplateElement {}
impl IHtmlElement for TemplateElement {}

impl TemplateElement {
    /// The content of the current template
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/content)
    // https://html.spec.whatwg.org/multipage/scripting.html#the-template-element:documentfragment
    #[inline]
    pub fn content ( &self ) -> DocumentFragment {
        js! (
            return @{self}.content;
        ).try_into().unwrap()
    }
}


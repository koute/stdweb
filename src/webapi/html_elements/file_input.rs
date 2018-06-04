use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::IHtmlElement;
use webapi::html_elements::input::InputElement;
use webapi::file_list::FileList;

/// The HTML file input element is used to select files from the user's device
///
/// [(JavaScript docs)](https://developer.mozilla.org/en/docs/Web/HTML/Element/input)
// https://html.spec.whatwg.org/#htmlinputelement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(input_instance_of = "file")]
#[reference(subclass_of(EventTarget, Node, Element, InputElement))]
pub struct FileInputElement( Reference );

impl IEventTarget for FileInputElement {}
impl INode for FileInputElement {}
impl IElement for FileInputElement {}
impl IHtmlElement for FileInputElement {}

impl FileInputElement {
    /// The file input's selected files.
    // https://html.spec.whatwg.org/multipage/input.html#dom-input-files
    #[inline]
    pub fn files( &self ) -> FileList {
        js! (
            return @{self}.files;
        ).try_into().unwrap()
    }
}

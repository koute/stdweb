use webapi::element::{Element, IElement};
use webapi::event_target::{EventTarget, IEventTarget};
use webapi::html_element::{HtmlElement, IHtmlElement};
use webapi::node::{INode, Node};
use webcore::try_from::TryInto;
use webcore::value::Reference;
use webapi::html_collection::HtmlCollection;
use webapi::html_elements::OptionElement;

/// The HTML <select> element represents a control that provides a menu of options.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
// https://html.spec.whatwg.org/#the-select-element
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLSelectElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct SelectElement(Reference);

impl IEventTarget for SelectElement {}
impl INode for SelectElement {}
impl IElement for SelectElement {}
impl IHtmlElement for SelectElement {}

impl SelectElement {
    /// Returns the `Some(index)` of the first selected item, if any, or `None` if there is no selected item.
    // https://html.spec.whatwg.org/#dom-select-selectedindex
    pub fn selected_index(&self) -> Option<i32> {
        let si = js! (
            return @{self}.selectedIndex;
        ).try_into().unwrap();
        if si < 0 {
            None
        } else {
            Some(si)
        }
    }

    /// Change selected index to the given value.
    // https://html.spec.whatwg.org/#dom-select-selectedindex
    pub fn set_selected_index(&self, selected_index: Option<i32>) {
        let selected_index = selected_index.unwrap_or(-1);
        js!{
            @(no_return)
            @{self}.selectedIndex = @{selected_index};
        }
    }

    /// Returns the `Some(value)` of the first selected item, if any, or `None` if there is no selected item.
    // https://html.spec.whatwg.org/#dom-select-value
    pub fn value(&self) -> Option<String> {
        let value = js!(
            return @{self}.value;
        ).try_into().unwrap();
        if value == "" {
            None
        } else {
            Some(value)
        }
    }

    /// Change the selected value to the given value.
    // https://html.spec.whatwg.org/#dom-select-value
    pub fn set_value(&self, value: Option<String>) {
        let value = value.unwrap_or("".to_string());
        js!{
            @(no_return)
            @{self}.value = @{value};
        }
    }

    /// Indicates whether multiple items can be selected
    // https://html.spec.whatwg.org/#dom-select-multiple
    pub fn multiple(&self) -> bool {
        js!(
            return @{self}.multiple;
        ).try_into().unwrap()
    }

    /// An [HtmlCollection](struct.HtmlCollection.html) representing
    /// the set of <option> elements that are selected.
    // https://html.spec.whatwg.org/#dom-select-selectedoptions
    pub fn selected_options(&self) -> HtmlCollection {
        js!(
            return @{self}.selectedOptions;
        ).try_into().unwrap()
    }

    /// A convenient method to get values of all selected <option> elements
    pub fn selected_values(&self) -> Vec<String> {
        self.selected_options()
            .iter().map(|e|{
                let e: OptionElement = e.try_into().unwrap();
                e.value()
            }).collect::<Vec<String>>()
    }

    /// A convenient method to get indices of all selected <option> elements
    pub fn selected_indices(&self) -> Vec<i32> {
        self.selected_options()
            .iter().map(|e|{
                let e: OptionElement = e.try_into().unwrap();
                e.index()
            }).collect::<Vec<i32>>()
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests{
    use super::SelectElement;
    use webcore::try_from::TryInto;
    #[test]
    fn test_select_one() {
        let html = "<option value='first'>First option</option>\
            <option value='second'>Second option</option>\
            <option value='third' selected>Third option</option>\
            <option value='4th'>4th option</option>";
        let se: SelectElement = js!(
            var se = document.createElement("select");
            se.innerHTML = @{html};
            return se;
        ).try_into().unwrap();

        assert_eq!(se.multiple(), false);

        assert_eq!(se.selected_index(), Some(2));
        assert_eq!(se.value(), Some("third".to_string()));

        se.set_selected_index(Some(1));
        assert_eq!(se.selected_index(), Some(1));
        assert_eq!(se.value(), Some("second".to_string()));

        se.set_selected_index(None);
        assert_eq!(se.selected_index(), None);
        assert_eq!(se.value(), None);

        se.set_value(Some("first".to_string()));
        assert_eq!(se.selected_index(), Some(0));
        assert_eq!(se.value(), Some("first".to_string()));

        se.set_value(None);
        assert_eq!(se.selected_index(), None);
        assert_eq!(se.value(), None);

    }

    #[test]
    fn test_select_multiple(){
         let html = "<option value='first' selected>First option</option>\
            <option value='second'>Second option</option>\
            <option value='third' selected>Third option</option>\
            <option value='4th'>4th option</option>";
        let se: SelectElement = js!(
            var se = document.createElement("select");
            se.multiple = true;
            se.innerHTML = @{html};
            return se;
        ).try_into().unwrap();

        assert_eq!(se.multiple(), true);

        assert_eq!(se.selected_indices(), vec![0,2]);
        assert_eq!(se.selected_values(), vec!["first".to_string(), "third".to_string()]);
    }
}
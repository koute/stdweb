use webapi::element::{Element, IElement};
use webapi::event_target::{EventTarget, IEventTarget};
use webapi::html_element::{HtmlElement, IHtmlElement};
use webapi::node::{INode, Node};
use webcore::try_from::TryInto;
use webcore::value::Reference;
use webapi::html_collection::HtmlCollection;
use webapi::html_elements::OptionElement;

/// Indicates that an invalid value is setted to an `SelectElement`.
/// It means there is no `<option>` element that has the given value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownValueError(String);

impl std::fmt::Display for UnknownValueError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "There is no `<option>` element that has value='{}'", self.0)
    }
}

impl std::error::Error for UnknownValueError {
    fn description(&self) -> &str {
        "There is no `<option>` element that has the given value"
    }
}

/// The HTML `<select>` element represents a control that provides a menu of options.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
// https://html.spec.whatwg.org/#htmlselectelement
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "HTMLSelectElement")]
#[reference(subclass_of(EventTarget, Node, Element, HtmlElement))]
pub struct SelectElement(Reference);

impl IEventTarget for SelectElement {}
impl INode for SelectElement {}
impl IElement for SelectElement {}
impl IHtmlElement for SelectElement {}

impl SelectElement {
    /// Returns the value attribute of the first selected `<option>` element or
    /// if it is missing, the text attribute. If there is no selection, return empty string.
    /// This method is just a wrapper for getting `HTMLSelectElement.value` directly
    // https://html.spec.whatwg.org/#the-select-element:dom-select-value
    pub fn raw_value(&self) -> String {
        js!(
            return @{self}.value
        ).try_into().unwrap()
    }

    /// Set the given value to `HTMLSelectElement.value` directly.
    // https://html.spec.whatwg.org/#the-select-element:dom-select-value
    pub fn set_raw_value(&self, value: &str) {
        js!{
            @(no_return)
            @{self}.value = @{value};
        }
    }
    
    /// Returns the `Some(index)` of the first selected item, if any, or `None` if there is no selected item.
    // https://html.spec.whatwg.org/#the-select-element:dom-select-selectedindex
    pub fn selected_index(&self) -> Option<u32> {
        js! (
            var self = @{self};
            if (self.selectedIndex < 0) {
                return null;
            }else{
                return self.selectedIndex;
            }
        ).try_into().unwrap()
    }

    /// Change selected index to the given value.
    // https://html.spec.whatwg.org/#the-select-element:dom-select-selectedindex
    pub fn set_selected_index(&self, selected_index: Option<u32>) {
        match selected_index {
            Some(si) => js!{
                @(no_return)
                @{self}.selectedIndex = @{si};
            },
            None => js!{
                @(no_return)
                @{self}.selectedIndex = -1;
            }
        };
    }

    /// Returns the `Some(value)` of the first selected item, if any, or `None` if there is no selected item.
    // https://html.spec.whatwg.org/#the-select-element:dom-select-value
    pub fn value(&self) -> Option<String> {
        match self.selected_index(){
            None => None,
            Some(_) => Some(self.raw_value())
        }
    }

    /// Change the selected value to the given value. If you provide an invalid value,
    /// the `<select>` element will have no item selected, and an `UnknownValueError` is returned.
    // https://html.spec.whatwg.org/#the-select-element:dom-select-value
    pub fn set_value(&self, value: Option<&str>) -> Result<(), UnknownValueError> {
        match value{
            Some(value) => {
                self.set_raw_value(value);
                if self.selected_index().is_none(){
                    Err(UnknownValueError(value.to_string()))
                }else{
                    Ok(())
                }
            },
            None => {
                self.set_selected_index(None);
                Ok(())
            }
        }
    }

    /// Indicates whether multiple items can be selected
    // https://html.spec.whatwg.org/#the-select-element:dom-select-multiple
    pub fn multiple(&self) -> bool {
        js!(
            return @{self}.multiple;
        ).try_into().unwrap()
    }

    /// An `HtmlCollection` representing
    /// the set of `<option>` elements that are selected.
    // https://html.spec.whatwg.org/#the-select-element:dom-select-selectedoptions
    pub fn selected_options(&self) -> HtmlCollection {
        js!(
            return @{self}.selectedOptions;
        ).try_into().unwrap()
    }

    /// A convenience method to get values of all selected `<option>` elements
    pub fn selected_values(&self) -> Vec<String> {
        self.selected_options()
            .iter().map(|e|{
                let e: OptionElement = e.try_into().unwrap();
                e.value()
            }).collect::<Vec<String>>()
    }

    /// A convenience method to get indices of all selected `<option>` elements
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
    use super::{SelectElement, UnknownValueError};
    use webapi::node::Node;
    use webcore::try_from::TryInto;
    #[test]
    fn test_select_one() {
        let html = r#"<select><option value='first'>First option</option>
            <option value='second'>Second option</option>
            <option value='third' selected>Third option</option>
            <option value=''>Empty</option></select>"#;
        let se: SelectElement = Node::from_html(html).unwrap().try_into().unwrap();

        assert_eq!(se.multiple(), false);

        assert_eq!(se.selected_index(), Some(2));
        assert_eq!(se.value(), Some("third".to_string()));

        se.set_selected_index(Some(1));
        assert_eq!(se.selected_index(), Some(1));
        assert_eq!(se.value(), Some("second".to_string()));

        se.set_selected_index(None);
        assert_eq!(se.selected_index(), None);
        assert_eq!(se.value(), None);

        let rs = se.set_value(Some("first"));
        assert_eq!(rs, Ok(()));
        assert_eq!(se.selected_index(), Some(0));
        assert_eq!(se.value(), Some("first".to_string()));

        let rs = se.set_value(None);
        assert_eq!(rs, Ok(()));
        assert_eq!(se.selected_index(), None);
        assert_eq!(se.value(), None);

        let rs = se.set_value(Some(""));
        assert_eq!(rs, Ok(()));
        assert_eq!(se.selected_index(), Some(3));
        assert_eq!(se.value(), Some("".to_string()));

        let rs = se.set_value(Some("invalid_option"));
        assert_eq!(rs, Err(UnknownValueError("invalid_option".to_string())));
        assert_eq!(se.selected_index(), None);
        assert_eq!(se.value(), None);
    }

    #[test]
    fn test_select_multiple_noselection(){
         let html = r#"<select multiple><option value='first'>First option</option>
            <option value='second'>Second option</option>
            <option value='third'>Third option</option>
            <option value='4th'>4th option</option></select>"#;
        let se: SelectElement = Node::from_html(html).unwrap().try_into().unwrap();

        assert_eq!(se.multiple(), true);

        let empy_i32_vec: Vec<i32> = Vec::new();
        let empy_string_vec: Vec<String> = Vec::new();
        assert_eq!(se.selected_indices(), empy_i32_vec);
        assert_eq!(se.selected_values(), empy_string_vec);
    }

    #[test]
    fn test_select_multiple(){
         let html = r#"<select multiple><option value='first' selected>First option</option>
            <option value='second'>Second option</option>
            <option value='third' selected>Third option</option>
            <option value='4th'>4th option</option>
            <option value='' selected>Empty</option></select>"#;
        let se: SelectElement = Node::from_html(html).unwrap().try_into().unwrap();

        assert_eq!(se.multiple(), true);

        assert_eq!(se.selected_indices(), vec![0,2,4]);
        assert_eq!(se.selected_values(), vec!["first".to_string(), "third".to_string(), "".to_string()]);
    }
}

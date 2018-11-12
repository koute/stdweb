use webcore::reference_type::ReferenceType;
use webapi::node_list::NodeList;
use webapi::element::Element;
use webapi::dom_exception::SyntaxError;

/// The `ParentNode` mixin contains methods and properties
/// that are common to all types of `Node` objects that can
/// have children.
///
/// You most likely don't want to `use` this directly; instead
/// you should `use stdweb::traits::*;`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ParentNode)
// https://dom.spec.whatwg.org/#parentnode
pub trait IParentNode: ReferenceType {
    /// Returns the first element that is a descendant of the element on which it is
    /// invoked that matches the specified group of selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)
    // https://dom.spec.whatwg.org/#ref-for-dom-parentnode-queryselector
    fn query_selector( &self, selector: &str ) -> Result< Option< Element >, SyntaxError > {
        js_try!(
            return @{self.as_ref()}.querySelector(@{selector});
        ).unwrap()
    }

    /// Returns a non-live [NodeList](struct.NodeList.html) of all elements descended
    /// from the element on which it is invoked that matches the specified group of CSS selectors.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)
    // https://dom.spec.whatwg.org/#ref-for-dom-parentnode-queryselectorall
    fn query_selector_all( &self, selector: &str ) -> Result< NodeList, SyntaxError > {
        js_try!(
            return @{self.as_ref()}.querySelectorAll(@{selector});
        ).unwrap()
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use stdweb::web::document;
    use webapi::node::INode;

    #[test]
    fn test_query_selector_finds_h1() {
        let parent = document().create_element("div").unwrap();
        let child = document().create_element("h1").unwrap();
        parent.append_child(&child);

        assert_eq!(parent.query_selector("h1").unwrap().unwrap().as_ref(), child.as_ref());
    }


    #[test]
    fn test_query_selector_all_finds_h1s() {
        let parent = document().create_element("div").unwrap();
        let child = document().create_element("h1").unwrap();
        let child2 = document().create_element("h1").unwrap();
        parent.append_child(&child);
        parent.append_child(&child2);

        assert_eq!(parent.query_selector_all("h1").unwrap().len(), 2);
    }

    #[test]
    fn test_query_selector_finds_nested_h1() {
        let parent = document().create_element("div").unwrap();
        let child = document().create_element("p").unwrap();
        let title = document().create_element("h1").unwrap();
        child.append_child(&title);
        parent.append_child(&child);

        assert_eq!(parent.query_selector("h1").unwrap().unwrap().as_ref(), title.as_ref());
    }

    #[test]
    fn test_query_selector_all_finds_nested_h1s() {
        let parent = document().create_element("div").unwrap();
        let top_title = document().create_element("h1").unwrap();
        let child = document().create_element("p").unwrap();
        let title = document().create_element("h1").unwrap();
        child.append_child(&title);
        parent.append_child(&child);
        parent.append_child(&top_title);

        assert_eq!(parent.query_selector_all("h1").unwrap().len(), 2);
    }

    #[test]
    fn test_query_selector_not_found() {
        let parent = document().create_element("div").unwrap();
        let child = document().create_element("h1").unwrap();
        parent.append_child(&child);

        assert!(parent.query_selector("p").unwrap().is_none());
    }

    #[test]
    fn test_query_selector_all_not_found() {
        let parent = document().create_element("div").unwrap();
        let child = document().create_element("h1").unwrap();
        parent.append_child(&child);

        assert_eq!(parent.query_selector_all("p").unwrap().len(), 0);
    }

    #[test]
    fn test_query_selector_syntax_error() {
        let parent = document().create_element("div").unwrap();
        let child = document().create_element("h1").unwrap();
        parent.append_child(&child);

        assert!(child.query_selector("invalid syntax +#8$()@!(#").is_err());
    }

    #[test]
    fn test_query_selector_all_syntax_error() {
        let parent = document().create_element("div").unwrap();
        let child = document().create_element("h1").unwrap();
        parent.append_child(&child);

        assert!(child.query_selector_all("invalid syntax +#8$()@!(#").is_err());
    }
}

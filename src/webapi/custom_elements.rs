use webcore::value::Reference;
webapi::error::TypeError;

use webapi::dom_exception::{
    NotSupportedError,
    SyntaxError
};

#[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
use webcore::promise::{Promise, TypedPromise};

/// The CustomElementRegistry interface provides methods for registering custom elements 
/// and querying registered elements. To get an instance of it, use the window.customElements property. 
///
/// [(JavaScript docs)](hhttps://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry)
/// https://html.spec.whatwg.org/multipage/custom-elements.html#customelementregistry
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "CustomElementRegistry")]
pub struct CustomElementRegistry(Reference);


error_enum_boilerplate! {
    /// A enum of the exceptions that CustomElementRegistry.define() may throw
    DefineError,
    /// A TypeError
    TypeError
    /// A NotSupportedError
    NotSupportedError,
    /// A SyntaxError
    SyntaxError,
}

impl CustomElementRegistry {
    /// Defines a new custom element
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)
    /// https://html.spec.whatwg.org/multipage/custom-elements.html#dom-customelementregistry-define
    pub fn define( &self, name: &str, constructor: Any) -> Result<(), DefineError> {
        js!(
            return @{self}.define(name, constructor); 
        ).try_into().unwrap()
    }

    /// Returns the constuctor for the named custom element, or undefined if the custom element is not defined.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/get)
    /// https://html.spec.whatwg.org/multipage/custom-elements.html#dom-customelementregistry-get
    pub fn get( &self, name: &str ) -> Option<Constructor> {
        js!( return @{self}.get(name); ).try_into().unwrap()
    }

    /// Returns a promise that will be fulfilled when a custom element becomes defined with the given name.
    /// (If such a custom element is already defined, the returned promise is immediately fulfilled.)
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/whenDefined)
    /// https://html.spec.whatwg.org/multipage/custom-elements.html#dom-customelementregistry-whendefined
    #[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
    pub fn whenDefined( &self, name: &str ) -> TypedPromise<(), SyntaxError > {
        let promise: Promise = js!( return @{self}.whenDefined(name); ).try_into().unwrap();
        TypedPromise::new(promise)
    })
}

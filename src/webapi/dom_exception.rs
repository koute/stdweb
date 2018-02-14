use webcore::value::Reference;
use webapi::error::{IError, Error};

/// The `IDomException` interface represents an abnormal event which occurs as the result of
/// calling a web API.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)
// https://heycam.github.io/webidl/#idl-DOMException
pub trait IDomException: IError {}

/// A reference to a JavaScript `DOMException` object.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)
// https://heycam.github.io/webidl/#idl-DOMException
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DOMException")]
#[reference(subclass_of(Error))]
pub struct DomException( Reference );

impl IError for DomException {}
impl IDomException for DomException {}

error_boilerplate! { DomException }

/// Occurs when an operation would result in an incorrect node tree.
// https://heycam.github.io/webidl/#hierarchyrequesterror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct HierarchyRequestError( Reference );

impl IError for HierarchyRequestError {}
impl IDomException for HierarchyRequestError {}

error_boilerplate! { HierarchyRequestError, name = "HierarchyRequestError" }

/// Occurs when an object does not support an operation or argument.
// https://heycam.github.io/webidl/#invalidaccesserror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct InvalidAccessError( Reference );

impl IError for InvalidAccessError {}
impl IDomException for InvalidAccessError {}

error_boilerplate! { InvalidAccessError, name = "InvalidAccessError" }

/// Occurs when the specified object cannot be found.
// https://heycam.github.io/webidl/#notfounderror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct NotFoundError( Reference );

impl IError for NotFoundError {}
impl IDomException for NotFoundError {}

error_boilerplate! { NotFoundError, name = "NotFoundError" }

/// Occurs when the requested operation is insecure.
// https://heycam.github.io/webidl/#securityerror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct SecurityError( Reference );

impl IError for SecurityError {}
impl IDomException for SecurityError {}

error_boilerplate! { SecurityError, name = "SecurityError" }

/// Occurs when an argument does not match the expected pattern.
// https://heycam.github.io/webidl/#syntaxerror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct SyntaxError( Reference );

impl IError for SyntaxError {}
impl IDomException for SyntaxError {}

error_boilerplate! { SyntaxError, name = "SyntaxError" }

#[cfg(all(test, feature = "web_test"))]
mod test {
    use super::*;
    use webcore::try_from::TryInto;

    fn new_dom_exception(message: &str, name: &str) -> DomException {
        js!(
            return new DOMException(@{message}, @{name});
        ).try_into().unwrap()
    }

    #[test]
    fn test_error() {
        let name = "HierarchyRequestError";
        // Successful downcast.
        let err: DomException = new_dom_exception("foo", name);
        let err: HierarchyRequestError = err.try_into().expect("Expected HierarchyRequestError");
        assert_eq!(err.name(), name);

        // Failed downcast.
        let err: DomException = new_dom_exception("foo", name);
        let err: Result<SyntaxError, _> = err.try_into();
        assert!(err.is_err());
    }
}

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

error_boilerplate! { HierarchyRequestError, dom_exception = "HierarchyRequestError" }

/// Occurs when an object does not support an operation or argument.
// https://heycam.github.io/webidl/#invalidaccesserror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct InvalidAccessError( Reference );

impl IError for InvalidAccessError {}
impl IDomException for InvalidAccessError {}

error_boilerplate! { InvalidAccessError, dom_exception = "InvalidAccessError" }

/// Occurs when the object can not be modified.
// https://heycam.github.io/webidl/#nomodificationallowederror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct NoModificationAllowedError( Reference );

impl IError for NoModificationAllowedError {}
impl IDomException for NoModificationAllowedError {}

error_boilerplate! { NoModificationAllowedError, dom_exception = "NoModificationAllowedError" }

/// Occurs when the specified object cannot be found.
// https://heycam.github.io/webidl/#notfounderror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct NotFoundError( Reference );

impl IError for NotFoundError {}
impl IDomException for NotFoundError {}

error_boilerplate! { NotFoundError, dom_exception = "NotFoundError" }

/// Occurs when the requested operation is insecure.
// https://heycam.github.io/webidl/#securityerror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct SecurityError( Reference );

impl IError for SecurityError {}
impl IDomException for SecurityError {}

error_boilerplate! { SecurityError, dom_exception = "SecurityError" }

/// Occurs when an argument does not match the expected pattern.
// https://heycam.github.io/webidl/#syntaxerror
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct SyntaxError( Reference );

impl IError for SyntaxError {}
impl IDomException for SyntaxError {}

error_boilerplate! { SyntaxError, dom_exception = "SyntaxError" }

/// Occurs when an argument is out of range.
// https://heycam.github.io/webidl/#indexsizeerror
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct IndexSizeError( Reference );

impl IError for IndexSizeError {}
impl IDomException for IndexSizeError {}

error_boilerplate! { IndexSizeError, dom_exception = "IndexSizeError" }

/// Occurs when an object is in an invalid state.
// https://heycam.github.io/webidl/#invalidstateerror
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct InvalidStateError( Reference );

impl IError for InvalidStateError {}
impl IDomException for InvalidStateError {}

error_boilerplate! { InvalidStateError, dom_exception = "InvalidStateError" }

/// Used to indicate an unsuccessful operation when none of the other NativeError objects are an appropriate indication of the failure cause.
// https://heycam.github.io/webidl/#notsupportederror
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct NotSupportedError( Reference );

impl IError for NotSupportedError {}
impl IDomException for NotSupportedError {}

error_boilerplate! { NotSupportedError, dom_exception = "NotSupportedError" }

/// Used to indicate the string contains one or more characters which are invalid.
// https://heycam.github.io/webidl/#invalidcharactererror
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct InvalidCharacterError( Reference );

impl IError for InvalidCharacterError {}
impl IDomException for InvalidCharacterError {}

error_boilerplate! { InvalidCharacterError, dom_exception = "InvalidCharacterError" }

/// Used to indicate that a pointer id passed as an argument was for some reason invalid.
// https://w3c.github.io/pointerevents/#extensions-to-the-element-interface
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct InvalidPointerId( Reference );

impl IError for InvalidPointerId {}
impl IDomException for InvalidPointerId {}

error_boilerplate! { InvalidPointerId, dom_exception = "InvalidPointerId" }

/// Used to indicate that the operation was aborted.
// https://heycam.github.io/webidl/#aborterror
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct AbortError( Reference );

impl IError for AbortError {}
impl IDomException for AbortError {}

error_boilerplate! { AbortError, dom_exception = "AbortError" }

/// Indicates an xml namespace-related feature was used incorrectly.
// https://heycam.github.io/webidl/#namespaceerror
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error, DomException))]
pub struct NamespaceError( Reference );

impl IError for NamespaceError {}
impl IDomException for NamespaceError {}

error_boilerplate! { NamespaceError, dom_exception = "NamespaceError" }

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

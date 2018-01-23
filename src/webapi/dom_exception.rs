use webcore::value::Reference;
use webapi::error::{IError, Error};

/// The `IDomException` interface represents an abnormal event which occurs as the result of
/// calling a web API.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)
pub trait IDomException: IError {}

/// A reference to a JavaScript `DOMException` object.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)
pub struct DomException( Reference );

// DOMException specification:
// https://heycam.github.io/webidl/#idl-DOMException

impl IError for DomException {}
impl IDomException for DomException {}

reference_boilerplate! {
    DomException,
    instanceof DOMException
    convertible to Error
}
error_boilerplate! { DomException }

/// A trait representing a concrete DOMException.
pub trait ConcreteException: IDomException {
    /// A string representing the error type.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/name)
    const ERROR_NAME: &'static str;
}

// To safely cast from a reference to a specific DomException, we must check that the object is an
// instance of DOMException, and that the error name matches.
impl<T: ConcreteException + ::webcore::value::FromReferenceUnchecked> ::webcore::value::FromReference for T {
    #[inline]
    fn from_reference( reference: Reference ) -> Option< Self > {
        if instanceof!( reference, DOMException ) &&
            js!( return @{reference.clone()}.name; ) == Self::ERROR_NAME {
            unsafe {
                Some( Self::from_reference_unchecked( reference ) )
            }
        } else {
            None
        }
    }
}

/// Occurs when an operation would result in an incorrect node tree.
pub struct HierarchyRequestError( Reference );

impl IError for HierarchyRequestError {}
impl IDomException for HierarchyRequestError {}
impl ConcreteException for HierarchyRequestError {
    const ERROR_NAME: &'static str = "HierarchyRequestError";
}

reference_boilerplate! {
    HierarchyRequestError,
    convertible to Error    
    convertible to DomException
}
error_boilerplate! { HierarchyRequestError }

/// Occurs when an object does not support an operation or argument.
pub struct InvalidAccessError( Reference );

impl IError for InvalidAccessError {}
impl IDomException for InvalidAccessError {}
impl ConcreteException for InvalidAccessError {
    const ERROR_NAME: &'static str = "InvalidAccessError";
}

reference_boilerplate! {
    InvalidAccessError,
    convertible to Error    
    convertible to DomException
}
error_boilerplate! { InvalidAccessError }

/// Occurs when the specified object cannot be found.
pub struct NotFoundError( Reference );

impl IError for NotFoundError {}
impl IDomException for NotFoundError {}
impl ConcreteException for NotFoundError {
    const ERROR_NAME: &'static str = "NotFoundError";
}

reference_boilerplate! {
    NotFoundError,
    convertible to Error    
    convertible to DomException
}
error_boilerplate! { NotFoundError }

/// Occurs when the requested operation is insecure.
pub struct SecurityError( Reference );

impl IError for SecurityError {}
impl IDomException for SecurityError {}
impl ConcreteException for SecurityError {
    const ERROR_NAME: &'static str = "SecurityError";
}

reference_boilerplate! {
    SecurityError,
    convertible to Error    
    convertible to DomException
}
error_boilerplate! { SecurityError }

/// Occurs when an argument does not match the expected pattern.
pub struct SyntaxError( Reference );

impl IError for SyntaxError {}
impl IDomException for SyntaxError {}
impl ConcreteException for SyntaxError {
    const ERROR_NAME: &'static str = "SyntaxError";
}

reference_boilerplate! {
    SyntaxError,
    convertible to Error    
    convertible to DomException
}
error_boilerplate! { SyntaxError }

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
        // Successful downcast.
        let err: DomException = new_dom_exception("foo", HierarchyRequestError::ERROR_NAME);
        let err: HierarchyRequestError = err.try_into().expect("Expected HierarchyRequestError");
        assert_eq!(err.name(), HierarchyRequestError::ERROR_NAME);

        // Failed downcast.
        let err: DomException = new_dom_exception("foo", SecurityError::ERROR_NAME);
        let err: Result<SyntaxError, _> = err.try_into();
        assert!(err.is_err());
    }
}
use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::reference_type::ReferenceType;

/// Represents the JavaScript `Error` interface. An `Error` is thrown whenever a run-time error
/// occurs.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)
pub trait IError: ReferenceType {
    /// Returns a human-readable description of the error.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message)
    #[inline]
    fn message( &self ) -> String {
        js!(
            return @{self.as_ref()}.message;
        ).try_into().unwrap()
    }

    /// Returns a name specifiying the type of error.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name)
    #[inline]
    fn name( &self ) -> String {
        js!(
            return @{self.as_ref()}.name;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript `Error` object. An `Error` is thrown whenever a run-time error
/// occurs.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "Error")]
pub struct Error( Reference );

// Error specification:
// https://www.ecma-international.org/ecma-262/6.0/#sec-error-objects

impl IError for Error {}

error_boilerplate! { Error }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_error() {
        use ::std::fmt::Write;

        let error: Error = js!(
           return new Error("foo");
        ).try_into().unwrap();

        assert_eq!(error.name(), "Error");
        assert_eq!(error.message(), "foo");

        let mut text = String::new();
        write!(&mut text, "{}", error).unwrap();
        assert_eq!(&text, "Error: foo");
        assert_eq!(::std::error::Error::description(&error), "Error");
    }
}

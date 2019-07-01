use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::reference_type::ReferenceType;

/// Represents the JavaScript `Error` interface. An `Error` is thrown whenever a run-time error
/// occurs.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)
// https://www.ecma-international.org/ecma-262/6.0/#sec-error-objects
pub trait IError: ReferenceType {
    /// Returns a human-readable description of the error.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-error.prototype.message
    #[inline]
    fn message( &self ) -> String {
        js!(
            return @{self.as_ref()}.message;
        ).try_into().unwrap()
    }

    /// Returns a name specifiying the type of error.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-error.prototype.name
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
// https://www.ecma-international.org/ecma-262/6.0/#sec-error-objects
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Error")]
pub struct Error( Reference );

impl Error {
    /// Creates a new `Error` with the specified `description`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)
    #[inline]
    pub fn new( description: &str ) -> Self {
        js!( return new Error( @{description} ); ).try_into().unwrap()
    }
}

impl IError for Error {}

error_boilerplate! { Error }

/// Used to indicate an unsuccessful operation when none of the other NativeError objects are an appropriate indication of the failure cause.
// https://tc39.github.io/ecma262/#sec-native-error-types-used-in-this-standard-typeerror
#[derive(Clone, Debug, ReferenceType)]
#[reference(subclass_of(Error))]
#[reference(instance_of = "TypeError")]
pub struct TypeError( Reference );

impl IError for TypeError {}

error_boilerplate! { TypeError }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_error() {
        use std::fmt::Write;

        let error: Error = js!(
           return new Error("foo");
        ).try_into().unwrap();

        assert_eq!(error.name(), "Error");
        assert_eq!(error.message(), "foo");

        let mut text = String::new();
        write!(&mut text, "{}", error).unwrap();
        assert_eq!(&text, "Error: foo");
        assert_eq!(std::error::Error::description(&error), "Error");
    }

    #[test]
    fn test_type_error() {
        let _: TypeError = js!( return new TypeError( "Big bad wolf" ); ).try_into().unwrap();
    }
}

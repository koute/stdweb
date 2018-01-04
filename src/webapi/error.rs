use std::fmt;
use std::error;


/// A DOMException as per https://developer.mozilla.org/en-US/docs/Web/API/DOMException
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DOMException {
    InvalidStateError
}

impl fmt::Display for DOMException {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        let message = error::Error::description( self );
        write!( formatter, "{}", message )
    }
}

impl error::Error for DOMException {
    fn description( &self ) -> &str {
        match *self {
            DOMException::InvalidStateError => "The object is in an invalid state."
        }
    }
}

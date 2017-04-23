use std::fmt;
use std::error;

/// An uninhabited type for use in statically impossible cases.
///
/// Will be replaced by Rust's `!` type once that stabilizes.
pub enum Void {}

impl fmt::Debug for Void {
    fn fmt( &self, _: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        unreachable!();
    }
}

impl fmt::Display for Void {
    fn fmt( &self, _: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        unreachable!();
    }
}

impl error::Error for Void {
    fn description( &self ) -> &str {
        unreachable!();
    }
}

use webcore::value::{Value, Reference};
use webcore::instance_of::InstanceOf;
use webcore::try_from::TryFrom;

/// A trait for types which wrap a reference to a JavaScript object.
pub trait ReferenceType: AsRef< Reference > + InstanceOf + TryFrom< Value > + TryFrom< Reference > {
    /// Converts a given reference into a concrete reference-like wrapper.
    /// Doesn't do any type checking; highly unsafe to use!
    /// # Safety
    /// This function is only safe so long as the given reference is an instance of the JS type the implementor claims to wrap.
    unsafe fn from_reference_unchecked( reference: Reference ) -> Self;
}

use webcore::value::Reference;

/// A trait to check whenever a given [Reference](struct.Reference.html) is of a certain type.
pub trait InstanceOf {
    /// Checks whenever a given [Reference](struct.Reference.html) if of type `Self`.
    fn instance_of( reference: &Reference ) -> bool;
}

impl InstanceOf for Reference {
    #[inline]
    fn instance_of( _: &Reference ) -> bool {
        true
    }
}

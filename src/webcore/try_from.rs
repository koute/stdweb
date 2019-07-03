/// Attempt to construct Self via a conversion.
///
/// This definition is only temporary until Rust's `TryFrom` is stabilized.
pub trait TryFrom< T >: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from( T ) -> Result< Self, Self::Error >;
}

/// An attempted conversion that consumes self, which may or may not be expensive.
///
/// This definition is only temporary until Rust's `TryInto` is stabilized.
pub trait TryInto< T >: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_into( self ) -> Result< T, Self::Error >;
}

impl< T, U > TryInto< U > for T where U: TryFrom< T > {
    type Error = U::Error;
    #[inline]
    fn try_into( self ) -> Result< U, U::Error > {
        U::try_from( self )
    }
}

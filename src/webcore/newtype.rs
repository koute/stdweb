use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::fmt;

// Unfortunately Rust doesn't allow something like this:
//   impl< A, T: Trait< A > > AnotherTrait for T {}
// It will simply return `unconstrained type parameter` error.
//
// To work around this we create a dummy newtype which will
// artificially constraint the extra parameter.
#[doc(hidden)]
pub struct Newtype< A, T >( T, PhantomData< A > );

impl< A, T > Newtype< A, T > {
    #[doc(hidden)]
    #[inline]
    pub fn unwrap_newtype( self ) -> T {
        self.0
    }
}

#[doc(hidden)]
pub trait IntoNewtype< A >: Sized {
    fn into_newtype( self ) -> Newtype< A, Self >;
}

impl< A, T > IntoNewtype< A > for T {
    #[inline]
    fn into_newtype( self ) -> Newtype< A, Self > {
        Newtype( self, PhantomData )
    }
}

impl< A, T > Deref for Newtype< A, T > {
    type Target = T;

    #[inline]
    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

impl< A, T > DerefMut for Newtype< A, T > {
    #[inline]
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.0
    }
}

impl< A, T: Copy > Copy for Newtype< A, T > {}
impl< A, T: Clone > Clone for Newtype< A, T > {
    #[inline]
    fn clone( &self ) -> Self {
        Newtype( self.0.clone(), PhantomData )
    }
}

impl< A, T > AsRef< T > for Newtype< A, T > {
    #[inline]
    fn as_ref( &self ) -> &T {
        &self.0
    }
}

impl< A, T > AsMut< T > for Newtype< A, T > {
    #[inline]
    fn as_mut( &mut self ) -> &mut T {
        &mut self.0
    }
}

impl< A, T: fmt::Debug > fmt::Debug for Newtype< A, T > {
    #[inline]
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> Result< (), fmt::Error > {
        self.0.fmt( formatter )
    }
}

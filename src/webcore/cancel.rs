use std::ops::{Deref, DerefMut};


pub trait Cancel {
    fn cancel( &mut self );
}


#[must_use = "
    The AutoCancel will be automatically cancelled when it is dropped.
    You probably don't want this to happen.
    How to fix this: either use the AutoCancel, or use .leak() which will cause it to not be cancelled (this will leak memory!)
"]
#[derive(Debug)]
pub struct AutoCancel< A: Cancel >( Option< A > );

impl< A: Cancel > AutoCancel< A > {
    #[inline]
    pub fn new( canceler: A ) -> Self {
        AutoCancel( Some( canceler ) )
    }

    #[inline]
    pub fn leak( mut self ) -> A {
        self.0.take().unwrap()
    }
}

impl< A: Cancel > Drop for AutoCancel< A > {
    #[inline]
    fn drop( &mut self ) {
        match self.0 {
            Some( ref mut canceler ) => canceler.cancel(),
            None => {},
        }
    }
}

impl< A: Cancel > Deref for AutoCancel< A > {
    type Target = A;

    #[inline]
    fn deref( &self ) -> &Self::Target {
        match self.0 {
            Some( ref canceler ) => canceler,
            None => unreachable!(),
        }
    }
}

impl< A: Cancel > DerefMut for AutoCancel< A > {
    #[inline]
    fn deref_mut( &mut self ) -> &mut Self::Target {
        match self.0 {
            Some( ref mut canceler ) => canceler,
            None => unreachable!(),
        }
    }
}

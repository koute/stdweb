use std::ops::{Deref, DerefMut};


pub trait Cancel {
    fn cancel( &mut self );
}


#[must_use = "

     The AutoCancel is unused, which causes it to be immediately cancelled.
     You probably don't want that to happen.

     How to fix this:
       1) Store the AutoCancel in a variable or data structure
       2) Use .leak() which will cause it to not be cancelled (this *will* leak memory!)

     See the documentation for more details.
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


#[cfg(test)]
mod tests {
    use super::{Cancel, AutoCancel};

    struct Foo( bool );

    impl Foo {
        fn new() -> AutoCancel< Foo > {
            AutoCancel::new( Foo( false ) )
        }
    }

    impl Cancel for Foo {
        fn cancel( &mut self ) {
            self.0 = true;
        }
    }

    #[test]
    fn unused() {
        Foo::new();
    }
}

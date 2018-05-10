use discard;
use discard::Discard;
use std::ops::{Deref, DerefMut};


/// If you have a value which implements [`Discard`](https://docs.rs/discard/%5E1.0.3/discard/trait.Discard.html), you can use
/// [`DiscardOnDrop::new(value)`](struct.DiscardOnDrop.html#method.new) which will wrap the value.
/// When the wrapper is dropped it will automatically call [`value.discard()`](https://docs.rs/discard/%5E1.0.3/discard/trait.Discard.html#tymethod.discard).
///
/// You can use the [`leak`](#method.leak) method to unwrap it (which returns `value`). This causes
/// it to no longer call [`discard`](https://docs.rs/discard/%5E1.0.3/discard/trait.Discard.html#tymethod.discard) when it is dropped, which
/// means it will usually leak memory unless you manually call [`discard`](https://docs.rs/discard/%5E1.0.3/discard/trait.Discard.html#tymethod.discard).
#[must_use = "

     The DiscardOnDrop is unused, which causes it to be immediately discarded.
     You probably don't want that to happen.

     How to fix this:

       * Store the DiscardOnDrop in a variable or data structure.

       * Or use the leak() method which will cause it to not be
         discarded (this will usually leak memory!)

     See the documentation for more details.
"]
#[derive(Debug)]
pub struct DiscardOnDrop< A: Discard >( discard::DiscardOnDrop< A > );

impl< A: Discard > DiscardOnDrop< A > {
    /// Creates a new `DiscardOnDrop`.
    ///
    /// When the `DiscardOnDrop` is dropped it will automatically call [`discarder.discard()`](https://docs.rs/discard/%5E1.0.3/discard/trait.Discard.html#tymethod.discard).
    #[inline]
    pub fn new( discarder: A ) -> Self {
        DiscardOnDrop( discard::DiscardOnDrop::new( discarder ) )
    }

    /// Returns the wrapped `discarder`.
    ///
    /// It will no longer automatically call [`discarder.discard()`](https://docs.rs/discard/%5E1.0.3/discard/trait.Discard.html#tymethod.discard), so this will usually leak
    /// memory unless you manually call [`discarder.discard()`](https://docs.rs/discard/%5E1.0.3/discard/trait.Discard.html#tymethod.discard).
    #[inline]
    pub fn leak( self ) -> A {
        discard::DiscardOnDrop::leak( self.0 )
    }
}

impl< A: Discard > Deref for DiscardOnDrop< A > {
    type Target = A;

    #[inline]
    fn deref( &self ) -> &Self::Target {
        &*self.0
    }
}

impl< A: Discard > DerefMut for DiscardOnDrop< A > {
    #[inline]
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut *self.0
    }
}


#[cfg(test)]
mod tests {
    use discard::Discard;
    use super::DiscardOnDrop;
    use std::rc::Rc;
    use std::cell::Cell;

    struct Foo( Rc< Cell< bool > > );

    impl Foo {
        fn new() -> Self {
            Foo( Rc::new( Cell::new( false ) ) )
        }

        fn dropped( &self ) -> Rc< Cell< bool > > {
            self.0.clone()
        }

        fn as_mut( &mut self ) -> &mut Self {
            self
        }
    }

    impl Discard for Foo {
        fn discard( self ) {
            self.0.set( true );
        }
    }


    #[test]
    fn unused() {
        Foo::new();
    }

    #[test]
    #[allow(unused_must_use)]
    fn unused_discard_on_drop() {
        DiscardOnDrop::new( Foo::new() );
    }

    #[test]
    fn discard() {
        let foo = Foo::new();

        let dropped = foo.dropped();

        assert_eq!( dropped.get(), false );
        foo.discard();
        assert_eq!( dropped.get(), true );
    }

    #[test]
    fn no_discard() {
        let foo = Foo::new();

        let dropped = foo.dropped();

        assert_eq!( dropped.get(), false );
        drop( foo );
        assert_eq!( dropped.get(), false );
    }

    #[test]
    fn discard_on_drop() {
        let foo = DiscardOnDrop::new( Foo::new() );

        let dropped = foo.dropped();

        assert_eq!( dropped.get(), false );
        drop( foo );
        assert_eq!( dropped.get(), true );
    }

    #[test]
    fn leak() {
        let foo = DiscardOnDrop::new(Foo::new());

        let dropped = foo.dropped();

        assert_eq!( dropped.get(), false );
        drop( foo.leak() );
        assert_eq!( dropped.get(), false );
    }

    #[test]
    fn deref_mut() {
        let mut foo = DiscardOnDrop::new( Foo::new() );

        let dropped = foo.as_mut().dropped();

        assert_eq!( dropped.get(), false );
        drop( foo.leak() );
        assert_eq!( dropped.get(), false );
    }
}

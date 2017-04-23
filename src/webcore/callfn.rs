// Since the {Fn, FnMut, FnOnce} traits are unstable we need
// to define our own versions.
//
// I'd put this in a separate crate, but we need to keep it
// here to allow Rust to do a better job of checking impl exhaustiveness,
// otherwise if we pull these traits from another crate we'll have
// `conflicting implementations of trait` errors.
pub trait CallOnce< Args > {
    type Output;
    fn call_once( self, args: Args ) -> Self::Output;
    fn expected_argument_count() -> usize;
}

pub trait CallMut< Args >: CallOnce< Args > {
    fn call_mut( &mut self, args: Args ) -> Self::Output;
}

pub trait Call< Args >: CallMut< Args > {
    fn call( &self, args: Args ) -> Self::Output;
}

macro_rules! noop {
    ($token:tt) => {}
}

macro_rules! define {
    ($next:tt => $($kind:ident),*) => {
        impl< R, $($kind,)* F: FnOnce( $($kind,)* ) -> R > CallOnce< ($($kind,)*) > for F {
            type Output = R;
            #[inline]
            fn call_once( self, args: ($($kind,)*) ) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($kind,)*) = args;
                self( $($kind),* )
            }

            #[inline]
            fn expected_argument_count() -> usize {
                let mut count = 0;
                $(
                    // I'm too lazy to make a separate macro to count the tokens so we just do this.
                    count += 1;
                    noop!( $kind );
                )*

                $crate::private::noop( &mut count );
                count
            }
        }

        impl< R, $($kind,)* F: FnMut( $($kind,)* ) -> R > CallMut< ($($kind,)*) > for F {
            #[inline]
            fn call_mut( &mut self, args: ($($kind,)*) ) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($kind,)*) = args;
                self( $($kind),* )
            }
        }

        impl< R, $($kind,)* F: Fn( $($kind,)* ) -> R > Call< ($($kind,)*) > for F {
            #[inline]
            fn call( &self, args: ($($kind,)*) ) -> Self::Output {
                #[allow(non_snake_case)]
                let ($($kind,)*) = args;
                self( $($kind),* )
            }
        }

        next! { $next }
    }
}

loop_through_identifiers!( define );

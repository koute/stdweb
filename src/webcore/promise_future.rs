use std;
use std::pin::PinMut;
use webcore::value::{Value, ConversionError};
use webcore::try_from::{TryInto, TryFrom};
use webcore::executor;
use webapi::error;
use futures_core::{Future, TryFuture, Poll};
use futures_core::task::Context;
use futures_util::{FutureExt, TryFutureExt};
use futures_channel::oneshot::Receiver;
use webcore::discard::DiscardOnDrop;
use webcore::serialization::JsSerialize;
use super::promise::{Promise, DoneHandle};


/// Asynchronously runs the [`Future`](https://docs.rs/futures/0.2.*/futures/future/trait.Future.html) on the current thread
/// and then immediately returns. This does *not* block the current thread.
///
/// This function is normally called once in `main`, it is usually not needed to call it multiple times.
///
/// The only way to retrieve the value of the future is to use the various
/// [`FutureExt`](https://docs.rs/futures/0.2.*/futures/future/trait.FutureExt.html) methods, such as
/// [`map`](https://docs.rs/futures/0.2.*/futures/future/trait.FutureExt.html#method.map) or
/// [`inspect`](https://docs.rs/futures/0.2.*/futures/future/trait.FutureExt.html#method.inspect).
///
/// In addition, you must handle all errors yourself. Because the errors happen asynchronously, the only way to catch them is
/// to use a [`FutureExt`](https://docs.rs/futures/0.2.*/futures/future/trait.FutureExt.html) method, such as
/// [`map_err`](https://docs.rs/futures/0.2.*/futures/future/trait.FutureExt.html#method.map_err).
///
/// It is very common to want to print the errors to the console. You can do that by using `.map_err(print_error_panic)`
///
/// # Examples
///
/// Asynchronously run a future in `main`, printing any errors to the console:
///
/// ```rust
/// fn main() {
///     spawn_local(
///         create_some_future()
///             .map_err(print_error_panic)
///     );
/// }
/// ```
///
/// Use the output value of the future:
///
/// ```rust
/// fn main() {
///     spawn_local(
///         create_some_future()
///             .map(|x| {
///                 println!("Future finished with value: {:#?}", x);
///             })
///             .map_err(print_error_panic)
///     );
/// }
/// ```
///
/// Catch errors and handle them yourself:
///
/// ```rust
/// fn main() {
///     spawn_local(
///         create_some_future()
///             .map_err(|e| handle_error_somehow(e))
///     );
/// }
/// ```
#[inline]
pub fn spawn_local< F >( future: F ) where F: Future< Output = () > + 'static {
    // TODO does this need to use PinBox instead ?
    let future: executor::BoxedFuture = Box::new( future ).into();
    executor::spawn_local( future );
}


/// Prints an error to the console and then panics.
///
/// See the documentation for [`spawn_local`](#method.spawn_local) for more details.
///
/// # Panics
/// This function *always* panics.
#[inline]
pub fn print_error_panic< A: JsSerialize >( value: A ) -> ! {
    js! { @(no_return)
        console.error( @{value} );
    }
    panic!();
}

/// Prints an error to the console and then panics.
///
/// See the documentation for [`spawn_local`](#method.spawn_local) for more details.
///
/// # Panics
/// This function *always* panics.
#[inline]
pub fn unwrap_future< F >( future: F ) -> impl Future< Output = F::Ok >
    where F: TryFuture,
          F::Error: JsSerialize {
    future.unwrap_or_else( |x| print_error_panic( x ) )
}


/// This allows you to use a JavaScript [`Promise`](struct.Promise.html) as if it is a Rust [`Future`](https://docs.rs/futures/0.2.*/futures/future/trait.Future.html).
///
/// The preferred way to create a `PromiseFuture` is to use [`value.try_into()`](unstable/trait.TryInto.html) on a JavaScript [`Value`](enum.Value.html).
///
/// # Examples
///
/// Convert a JavaScript `Promise` into a `PromiseFuture`:
///
/// ```rust
/// let future: PromiseFuture<String> = js!( return Promise.resolve("foo"); ).try_into().unwrap();
/// ```
pub struct PromiseFuture< Value, Error = error::Error > {
    pub(crate) future: Receiver< Result< Value, Error > >,
    pub(crate) _done_handle: DiscardOnDrop< DoneHandle >,
}

impl< A, B > std::fmt::Debug for PromiseFuture< A, B > {
    fn fmt( &self, formatter: &mut std::fmt::Formatter ) -> std::fmt::Result {
        formatter.debug_struct( "PromiseFuture" ).finish()
    }
}

impl< A, B > Future for PromiseFuture< A, B > {
    type Output = Result< A, B >;

    #[inline]
    fn poll( mut self: PinMut< Self >, cx: &mut Context ) -> Poll< Self::Output > {
        // TODO maybe remove this unwrap ?
        self.future.poll_unpin( cx ).map( |x| x.unwrap() )
    }
}

impl< A, B > TryFrom< Value > for PromiseFuture< A, B >
    where A: TryFrom< Value > + 'static,
          B: TryFrom< Value > + 'static,
          // TODO remove this later
          A::Error: std::fmt::Debug,
          B::Error: std::fmt::Debug {

    type Error = ConversionError;

    fn try_from( v: Value ) -> Result< Self, Self::Error > {
        let promise: Promise = v.try_into()?;
        Ok( promise.to_future() )
    }
}

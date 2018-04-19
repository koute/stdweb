use std;
use webcore::value::{Value, ConversionError};
use webcore::try_from::{TryInto, TryFrom};
use webapi::error;
use futures_core::{Future, Poll, Async, Never};
use futures_core::task::Context;
use futures_channel::oneshot::Receiver;
use webcore::executor::spawn_local;
use webcore::discard::DiscardOnDrop;
use super::promise::{Promise, DoneHandle};


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

impl PromiseFuture< (), Never > {
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
    /// It is very common to want to print the errors to the console. You can do that by using `.map_err(|e| console!(error, e))`
    ///
    /// # Examples
    ///
    /// Asynchronously run a future in `main`, printing any errors to the console:
    ///
    /// ```rust
    /// fn main() {
    ///     PromiseFuture::spawn_local(
    ///         create_some_future()
    ///             .map_err(|e| console!(error, e))
    ///     );
    /// }
    /// ```
    ///
    /// Inspect the output value of the future:
    ///
    /// ```rust
    /// fn main() {
    ///     PromiseFuture::spawn_local(
    ///         create_some_future()
    ///             .inspect(|x| println!("Future finished: {:#?}", x))
    ///             .map_err(|e| console!(error, e))
    ///     );
    /// }
    /// ```
    ///
    /// Catch errors and handle them yourself:
    ///
    /// ```rust
    /// fn main() {
    ///     PromiseFuture::spawn_local(
    ///         create_some_future()
    ///             .map_err(|e| handle_error_somehow(e))
    ///     );
    /// }
    /// ```
    #[inline]
    pub fn spawn_local< B >( future: B ) where
        B: Future< Item = (), Error = Never > + 'static {
        spawn_local( future );
    }
}

impl< A, B > std::fmt::Debug for PromiseFuture< A, B > {
    fn fmt( &self, formatter: &mut std::fmt::Formatter ) -> std::fmt::Result {
        formatter.debug_struct( "PromiseFuture" ).finish()
    }
}

impl< A, B > Future for PromiseFuture< A, B > {
    type Item = A;
    type Error = B;

    fn poll( &mut self, cx: &mut Context ) -> Poll< Self::Item, Self::Error > {
        // TODO maybe remove this unwrap ?
        match self.future.poll( cx ).unwrap() {
            Async::Ready( Ok( a ) ) => Ok( Async::Ready( a ) ),
            Async::Ready( Err( e ) ) => Err( e ),
            Async::Pending => Ok( Async::Pending ),
        }
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

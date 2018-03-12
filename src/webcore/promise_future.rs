use std;
use webcore::value::{Value, ConversionError};
use webcore::try_from::{TryInto, TryFrom};
use webapi::error;
use futures::{Future, Poll, Async};
use futures::unsync::oneshot::Receiver;
use webcore::promise_executor::spawn;
use super::promise::Promise;


/// This allows you to use a JavaScript [`Promise`](struct.Promise.html) as if it is a Rust [`Future`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html).
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
}

impl PromiseFuture< (), () > {
    /// Asynchronously runs the [`Future`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html) and then immediately returns.
    /// This does not block the current thread. The only way to retrieve the value of the future is to use the various
    /// [`Future`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html) methods, such as
    /// [`map`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html#method.map) or
    /// [`inspect`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html#method.inspect).
    ///
    /// This function requires you to handle all errors yourself. Because the errors happen asynchronously, the only way to catch them is
    /// to use a [`Future`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html) method, such as
    /// [`map_err`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html#method.map_err).
    ///
    /// It is very common to want to print the errors to the console. You can do that by using `.map_err(|e| console!(error, e))`
    ///
    /// This function is normally called once in `main`, it is usually not needed to call it multiple times.
    ///
    /// # Examples
    ///
    /// Asynchronously run a future in `main`, printing any errors to the console:
    ///
    /// ```rust
    /// fn main() {
    ///     PromiseFuture::spawn(
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
    ///     PromiseFuture::spawn(
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
    ///     PromiseFuture::spawn(
    ///         create_some_future()
    ///             .map_err(|e| handle_error_somehow(e))
    ///     );
    /// }
    /// ```
    #[inline]
    pub fn spawn< B >( future: B ) where
        B: Future< Item = (), Error = () > + 'static {
        spawn( future );
    }
}

impl< A, B > std::fmt::Debug for PromiseFuture< A, B > {
    fn fmt( &self, formatter: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( formatter, "PromiseFuture" )
    }
}

impl< A, B > Future for PromiseFuture< A, B > {
    type Item = A;
    type Error = B;

    fn poll( &mut self ) -> Poll< Self::Item, Self::Error > {
        // TODO maybe remove this unwrap ?
        match self.future.poll().unwrap() {
            Async::Ready( Ok( a ) ) => Ok( Async::Ready( a ) ),
            Async::Ready( Err( e ) ) => Err( e ),
            Async::NotReady => Ok( Async::NotReady ),
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

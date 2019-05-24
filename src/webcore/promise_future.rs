use std;
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context};
use webcore::value::{Value, ConversionError};
use webcore::try_from::{TryInto, TryFrom};
use webcore::executor;
use webapi::error;
use futures_core::TryFuture;
use futures_util::{FutureExt, TryFutureExt};
use futures_channel::oneshot::Receiver;
use webcore::discard::DiscardOnDrop;
use webcore::serialization::JsSerialize;
use super::promise::{Promise, DoneHandle};


/// Asynchronously runs the [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html)
/// on the current thread and then immediately returns. This does *not* block the current thread.
///
/// This function should normally be called only once in `main`, it is usually not needed to call it multiple times. If you want to run
/// multiple Futures in parallel you should use [`join`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/macro.join.html)
/// or [`try_join`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/macro.try_join.html).
///
/// ----
///
/// There are two types of Futures:
///
/// 1. By default Futures do not have any sort of error handling, and so they can be spawned directly:
///
///    ```rust
///    use stdweb::spawn_local;
///
///    fn main() {
///        spawn_local(
///            create_some_future()
///        );
///    }
///    ```
///
///    If you want to retrieve the return value of the Future, you can use the various asynchronous
///    [`FutureExt`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.FutureExt.html)
///    methods, such as [`map`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.FutureExt.html#method.map) or
///    [`inspect`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.FutureExt.html#method.inspect):
///
///    ```rust
///    use stdweb::spawn_local;
///    use futures::future::FutureExt;
///
///    fn main() {
///        spawn_local(
///            create_some_future()
///                .map(|x| {
///                    println!("Future finished with value: {:#?}", x);
///                })
///        );
///    }
///    ```
///
/// 2. However, some Futures return `Result`, and in that case you will need to deal with the `Result` somehow.
///
///    This is very common, because JavaScript Promises always return `Result` (because they might error).
///
///    In that case you can use the various asynchronous
///    [`TryFutureExt`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.TryFutureExt.html)
///    methods, such as [`map_ok`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.TryFutureExt.html#method.map_ok),
///    [`map_err`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.TryFutureExt.html#method.map_err), or
///    [`unwrap_or_else`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.TryFutureExt.html#method.unwrap_or_else):
///
///    ```rust
///    use stdweb::spawn_local;
///    use futures::future::TryFutureExt;
///
///    fn main() {
///        spawn_local(
///            create_some_future()
///                .map_ok(|x| {
///                    println!("Future finished with value: {:#?}", x);
///                })
///                .unwrap_or_else(|e| handle_error_somehow(e))
///        );
///    }
///    ```
///
///    It is very common to want to print the errors to the console, and so as a convenience you can use the [`unwrap_future`](fn.unwrap_future.html) function:
///
///    ```rust
///    use stdweb::{spawn_local, unwrap_future};
///    use futures::future::TryFutureExt;
///
///    fn main() {
///        spawn_local(
///            unwrap_future(create_some_future()
///                .map_ok(|x| {
///                    println!("Future finished with value: {:#?}", x);
///                }))
///        );
///    }
///    ```
///
///    If you don't need the return value from the Future, then it is even easier, since you don't need
///    [`map_ok`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.TryFutureExt.html#method.map_ok):
///
///    ```rust
///    use stdweb::{spawn_local, unwrap_future};
///
///    fn main() {
///        spawn_local(
///            unwrap_future(create_some_future())
///        );
///    }
///    ```
#[inline]
pub fn spawn_local< F >( future: F ) where F: Future< Output = () > + 'static {
    // TODO does this need to use PinBox instead ?
    let future: executor::BoxedFuture = Box::new( future ).into();
    executor::spawn_local( future );
}


/// Prints an error to the console and then panics.
///
/// If you're using Futures, it's more convenient to use [`unwrap_future`](fn.unwrap_future.html) instead.
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

/// Takes in an input
/// [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html)
/// (which returns `Result<A, B>`) and returns a new
/// [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html)
/// (which returns `A`).
///
/// If `future` returns `Err(error)`, then it prints `error` to the console and then panics.
///
/// Otherwise if `future` returns `Ok(value)` then the output
/// [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html)
/// returns `value`.
///
/// See the documentation for [`spawn_local`](fn.spawn_local.html) for more details.
///
/// # Panics
/// It panics if `future` returns `Err`.
#[inline]
pub fn unwrap_future< F >( future: F ) -> impl Future< Output = F::Ok >
    where F: TryFuture,
          F::Error: JsSerialize {
    future.unwrap_or_else( |x| print_error_panic( x ) )
}


/// Converts a JavaScript [`Promise`](struct.Promise.html) into a Rust
/// [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html).
///
/// The preferred way to create a `PromiseFuture` is to use [`value.try_into()`](unstable/trait.TryInto.html) on a
/// JavaScript [`Value`](enum.Value.html).
///
/// After creating a `PromiseFuture` you can use all of the
/// [`FutureExt`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.FutureExt.html)
/// and [`TryFutureExt`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.TryFutureExt.html)
/// methods on it, and you can spawn it by using [`spawn_local`](fn.spawn_local.html).
///
/// # Examples
///
/// Convert a JavaScript `Promise` into a `PromiseFuture`:
///
/// ```rust
/// fn foo() -> PromiseFuture<String> {
///     js!( return Promise.resolve("foo"); ).try_into().unwrap()
/// }
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
    fn poll( mut self: Pin< &mut Self >, cx: &mut Context ) -> Poll< Self::Output > {
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

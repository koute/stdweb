use std;
use std::error::Error as _Error;
use std::marker::PhantomData;
use webcore::once::Once;
use webcore::value::{Value, Reference, ConversionError};
use webcore::try_from::{TryInto, TryFrom};
use web::error::Error;
use futures::{Future, Poll, Async};
use futures::unsync::oneshot::{Receiver, channel};
use webcore::promise_executor::spawn;


/// A `Promise` object represents the eventual completion (or failure) of an asynchronous operation, and its resulting value.
///
/// In most situations you shouldn't use this, use [`PromiseFuture`](struct.PromiseFuture.html) instead.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise)
// https://www.ecma-international.org/ecma-262/6.0/#sec-promise-objects
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "Promise")]
pub struct Promise( Reference );

impl Promise {
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise-resolve-functions
    fn is_promise_like( input: &Value ) -> bool {
        (js! {
            var input = @{input};
            // This emulates the `Type(input) is Object` and `IsCallable(input.then)` ECMAScript abstract operations.
            return Object( input ) === input &&
                   typeof input.then === "function";
        }).try_into().unwrap()
    }

    /// This function should rarely be needed, use [`PromiseFuture`](struct.PromiseFuture.html) instead.
    ///
    /// This function is needed if you have a JavaScript value which is a Promise-like object
    /// (it has a `then` method) but it isn't a true `Promise`.
    ///
    /// That situation is rare, but it can happen if you are using a Promise library such as jQuery or
    /// Bluebird.
    ///
    /// In that situation you can use `Promise::convert(value)` to convert it into a true `Promise`.
    ///
    /// If the `input` isn't a Promise-like object then it returns `None`.
    ///
    /// # Examples
    ///
    /// Convert a Promise-like object to a `Promise`:
    ///
    /// ```rust
    /// // jQuery Promise
    /// Promise::convert(js!( return $.get("test.php"); ))
    ///
    /// // Bluebird Promise
    /// Promise::convert(js!( return bluebird_promise.timeout(1000); ))
    /// ```
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/resolve)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise.resolve
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise-resolve-functions
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promiseresolvethenablejob
    pub fn convert( input: Value ) -> Option< Self > {
        // TODO this can probably be made more efficient
        if Promise::is_promise_like( &input ) {
            Some( js!( return Promise.resolve( @{input} ); ).try_into().unwrap() )

        } else {
            None
        }
    }

    /// This method is usually not needed, use [`PromiseFuture`](struct.PromiseFuture.html) instead.
    ///
    /// When the `Promise` either succeeds or fails, it calls the `callback` with the result.
    ///
    /// It does not wait for the `Promise` to succeed / fail (it does not block the thread).
    ///
    /// The `callback` is guaranteed to be called asynchronously even if the `Promise` is already succeeded / failed.
    ///
    /// If the `Promise` never succeeds / fails then the `callback` will never be called, and it will leak memory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// promise.done(|result| {
    ///     match result {
    ///         Ok(success) => { ... },
    ///         Err(error) => { ... },
    ///     }
    /// });
    /// ```
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/then)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-performpromisethen
    pub fn done< A, B >( &self, callback: B )
        where A: TryFrom< Value >,
              A::Error: std::error::Error,
              B: FnOnce( Result< A, Error > ) + 'static {

        let callback = |value: Value, success: bool| {
            let value: Result< A, Error > = if success {
                let value: Result< A, A::Error > = value.try_into();
                value.map_err( |e| Error::new( e.description() ) )
            } else {
                let value: Result< Error, ConversionError > = value.try_into();
                value.map_err( |e| Error::new( e.description() ) ).and_then( Err )
            };

            callback( value );
        };

        js! { @(no_return)
            var callback = @{Once( callback )};

            // TODO don't swallow any errors thrown inside callback
            @{self}.then( function (value) {
                callback( value, true );
            }, function (value) {
                callback( value, false );
            } );
        }
    }

    /// This method should rarely be needed, instead use [`value.try_into()`](unstable/trait.TryInto.html) to convert directly from a [`Value`](enum.Value.html) into a [`PromiseFuture`](struct.PromiseFuture.html).
    ///
    /// This method converts the `Promise` into a [`PromiseFuture`](struct.PromiseFuture.html), so that it can be used as a Rust [`Future`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html).
    ///
    /// # Examples
    ///
    /// ```rust
    /// promise.to_future().map(|x| x + 1)
    /// ```
    // We can't use the IntoFuture trait because Promise doesn't have a type argument
    // TODO explain more why we can't use the IntoFuture trait
    pub fn to_future< A >( &self ) -> PromiseFuture< A >
         where A: TryFrom< Value > + 'static,
               A::Error: std::error::Error {

        let ( sender, receiver ) = channel();

        self.done( |value| {
            // TODO is this correct ?
            match sender.send( value ) {
                Ok( _ ) => {},
                Err( _ ) => {},
            };
        } );

        PromiseFuture {
            future: receiver,
            phantom: PhantomData,
        }
    }
}


/// This allows you to use a JavaScript [`Promise`](struct.Promise.html) as if it is a Rust [`Future`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html).
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
pub struct PromiseFuture< A > {
    future: Receiver< Result< A, Error > >,
    phantom: PhantomData< A >,
}

impl PromiseFuture< () > {
    /// Asynchronously runs the [`Future`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html) and then immediately returns.
    /// This does not block the current thread. The only way to retrieve the value of the future is to use the various
    /// [`Future`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html) methods, such as
    /// [`map`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html#method.map) or
    /// [`inspect`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html#method.inspect).
    ///
    /// This function requires you to handle all errors yourself. Because the errors happen asynchronously, the only way to catch them is
    /// to use a [`Future`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html) method, such as
    /// [`map_err`](https://docs.rs/futures/0.1.18/futures/future/trait.Future.html#method.map_err).
    ///
    /// It is very common to want to print the errors to the console. You can do that by using `.map_err(|e| e.print())`
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
    ///             .map_err(|e| e.print())
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
    ///             .map_err(|e| e.print())
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

impl< A > std::fmt::Debug for PromiseFuture< A > {
    fn fmt( &self, formatter: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( formatter, "PromiseFuture" )
    }
}

impl< A > Future for PromiseFuture< A > {
    type Item = A;
    type Error = Error;

    fn poll( &mut self ) -> Poll< Self::Item, Self::Error > {
        match self.future.poll() {
            Ok( Async::Ready( Ok( a ) ) ) => Ok( Async::Ready( a ) ),
            Ok( Async::Ready( Err( e ) ) ) => Err( e ),
            Ok( Async::NotReady ) => Ok( Async::NotReady ),
            Err( e ) => Err( Error::new( e.description() ) ),
        }
    }
}

impl< A > TryFrom< Value > for PromiseFuture< A >
    where A: TryFrom< Value > + 'static,
          A::Error: std::error::Error {

    type Error = ConversionError;

    fn try_from( v: Value ) -> Result< Self, Self::Error > {
        let promise: Promise = v.try_into()?;
        Ok( promise.to_future() )
    }
}

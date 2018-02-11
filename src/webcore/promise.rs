use std;
use webcore::once::Once;
use webcore::value::{Value, Reference};
use webcore::try_from::{TryInto, TryFrom};

#[cfg(feature = "futures")]
use futures::unsync::oneshot::channel;

#[cfg(feature = "futures")]
use super::promise_future::PromiseFuture;


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
    fn is_thenable( input: &Value ) -> bool {
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
    /// In that situation you can use `Promise::from_thenable(value)` to convert it into a true `Promise`.
    ///
    /// If the `input` isn't a Promise-like object then it returns `None`.
    ///
    /// # Examples
    ///
    /// Convert a Promise-like object to a `Promise`:
    ///
    /// ```rust
    /// // jQuery Promise
    /// Promise::from_thenable(js!( return $.get("test.php"); ))
    ///
    /// // Bluebird Promise
    /// Promise::from_thenable(js!( return bluebird_promise.timeout(1000); ))
    /// ```
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/resolve)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise.resolve
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise-resolve-functions
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promiseresolvethenablejob
    // TODO change this later to use &Reference
    pub fn from_thenable( input: Value ) -> Option< Self > {
        // TODO this can probably be made more efficient
        if Promise::is_thenable( &input ) {
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
    pub fn done< A, B, F >( &self, callback: F )
        where A: TryFrom< Value >,
              B: TryFrom< Value >,
              // TODO these Debug constraints are only needed because of unwrap
              A::Error: std::fmt::Debug,
              B::Error: std::fmt::Debug,
              F: FnOnce( Result< A, B > ) + 'static {

        let callback = |value: Value, success: bool| {
            let value: Result< A, B > = if success {
                // TODO figure out a way to avoid the unwrap
                let value: A = value.try_into().unwrap();
                Ok( value )
            } else {
                // TODO figure out a way to avoid the unwrap
                let value: B = value.try_into().unwrap();
                Err( value )
            };

            callback( value );
        };

        js! { @(no_return)
            var callback = @{Once( callback )};

            // TODO don't swallow any errors thrown inside callback
            @{self}.then( function ( value ) {
                callback( value, true );
            }, function ( value ) {
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
    #[cfg(feature = "futures")]
    pub fn to_future< A, B >( &self ) -> PromiseFuture< A, B >
         where A: TryFrom< Value > + 'static,
               B: TryFrom< Value > + 'static,
               // TODO remove these later
               A::Error: std::fmt::Debug,
               B::Error: std::fmt::Debug {

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
        }
    }
}

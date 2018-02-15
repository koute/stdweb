use std;
use webcore::once::Once;
use webcore::value::{Value, Reference};
use webcore::try_from::{TryInto, TryFrom};

#[cfg(feature = "futures")]
use webcore::serialization::JsSerialize;

#[cfg(feature = "futures")]
use futures::unsync::oneshot::channel;

#[cfg(feature = "futures")]
use futures::future::Future;

#[cfg(feature = "futures")]
use super::promise_future::PromiseFuture;


///
#[derive( Debug, Clone )]
pub struct DoneHandle {
    callback: Value,
    done: Value,
}

impl Drop for DoneHandle {
    fn drop( &mut self ) {
        js! { @(no_return)
            @{&self.done}[0] = true;
            @{&self.callback}.drop();
        }
    }
}


/// A `Promise` object represents the eventual completion (or failure) of an asynchronous operation, and its resulting value.
///
/// In most situations you shouldn't use this, use [`PromiseFuture`](struct.PromiseFuture.html) instead.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise)
// https://www.ecma-international.org/ecma-262/6.0/#sec-promise-objects
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Promise")]
pub struct Promise( Reference );

impl Promise {
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise-resolve-functions
    fn is_thenable( input: &Reference ) -> bool {
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
    /// In that situation you can use `Promise::from_thenable` to convert it into a true `Promise`.
    ///
    /// If the `input` isn't a Promise-like object then it returns `None`.
    ///
    /// # Examples
    ///
    /// Convert a Promise-like object to a `Promise`:
    ///
    /// ```rust
    /// // jQuery Promise
    /// Promise::from_thenable(&js!( return $.get("test.php"); ).try_into().unwrap())
    ///
    /// // Bluebird Promise
    /// Promise::from_thenable(&js!( return bluebird_promise.timeout(1000); ).try_into().unwrap())
    /// ```
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/resolve)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise.resolve
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise-resolve-functions
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promiseresolvethenablejob
    pub fn from_thenable( input: &Reference ) -> Option< Self > {
        // TODO this can probably be made more efficient
        if Promise::is_thenable( input ) {
            Some( js!( return Promise.resolve( @{input} ); ).try_into().unwrap() )

        } else {
            None
        }
    }

    /// This function converts a Rust Future into a JavaScript Promise.
    ///
    /// This is needed when you want to pass a Rust Future into JavaScript.
    ///
    /// If you simply want to use a JavaScript Promise inside Rust, then you
    /// don't need to use this function: you should use
    /// [`PromiseFuture`](struct.PromiseFuture.html) and the
    /// [`Future`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html)
    /// methods instead.
    ///
    /// # Examples
    ///
    /// Convert a Rust Future into a JavaScript Promise:
    ///
    /// ```rust
    /// Promise::from_future(rust_future)
    /// ```
    ///
    /// Export a Rust Future so that it can be used in JavaScript
    /// (this only works with the `wasm32-unknown-unknown` target):
    ///
    /// ```rust
    /// #[js_export]
    /// fn foo() -> Promise {
    ///     Promise::from_future(rust_future)
    /// }
    /// ```
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise#Syntax)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-promise-executor
    #[cfg(feature = "futures")]
    pub fn from_future< A >( future: A ) -> Self
        where A: Future + 'static,
              A::Item: JsSerialize,
              A::Error: JsSerialize {

        #[inline]
        fn call< A: JsSerialize >( f: Reference, value: A ) {
            js! { @(no_return) @{f}( @{value} ); }
        }

        let callback = move |success: Reference, error: Reference| {
            PromiseFuture::spawn(
                future.then( move |result| {
                    match result {
                        Ok( a ) => call( success, a ),
                        Err( a ) => call( error, a ),
                    }
                    Ok( () )
                } )
            );
        };

        js!( return new Promise( @{Once( callback )} ); ).try_into().unwrap()
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
    pub fn done< A, B, F >( &self, callback: F ) -> DoneHandle
        where A: TryFrom< Value >,
              B: TryFrom< Value >,
              // TODO these Debug constraints are only needed because of unwrap
              A::Error: std::fmt::Debug,
              B::Error: std::fmt::Debug,
              F: FnOnce( Result< A, B > ) + 'static {

        let callback = move |value: Value, success: bool| {
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

        let callback = js!( return @{Once( callback )}; );

        let done = js!(
            var callback = @{&callback};
            var done = [ false ];

            // TODO don't swallow any errors thrown inside callback
            @{self}.then( function ( value ) {
                if ( !done[0] ) {
                    callback( value, true );
                }
            }, function ( value ) {
                if ( !done[0] ) {
                    callback( value, false );
                }
            } );

            return done;
        );

        DoneHandle {
            callback,
            done,
        }
    }

    /// This method should rarely be needed, instead use [`value.try_into()`](unstable/trait.TryInto.html) to convert directly from a [`Value`](enum.Value.html) into a [`PromiseFuture`](struct.PromiseFuture.html).
    ///
    /// This method converts the `Promise` into a [`PromiseFuture`](struct.PromiseFuture.html), so that it can be used as a Rust [`Future`](https://docs.rs/futures/0.1.*/futures/future/trait.Future.html).
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

        PromiseFuture {
            future: receiver,
            _done_handle: self.done( |value| {
                // TODO is this correct ?
                match sender.send( value ) {
                    Ok( _ ) => {},
                    Err( _ ) => {},
                };
            } ),
        }
    }
}

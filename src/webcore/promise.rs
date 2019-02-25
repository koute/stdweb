use std;
use std::fmt;
use std::marker::PhantomData;

use discard::Discard;
use webcore::once::Once;
use webcore::value::{Value, Reference};
use webcore::try_from::{TryInto, TryFrom};
use webcore::discard::DiscardOnDrop;

#[cfg(feature = "futures-support")]
use webcore::serialization::JsSerialize;

#[cfg(feature = "futures-support")]
use futures_core::TryFuture;

#[cfg(feature = "futures-support")]
use futures_util::{FutureExt, TryFutureExt};

#[cfg(feature = "futures-support")]
use futures_util::future::ready;

#[cfg(feature = "futures-support")]
use futures_channel::oneshot::channel;

#[cfg(feature = "futures-support")]
use super::promise_future::{PromiseFuture, spawn_local};


/// This is used to cleanup the [`done`](struct.Promise.html#method.done) callback.
///
/// See the documentation for [`done`](struct.Promise.html#method.done) for more information.
#[derive( Debug, Clone )]
pub struct DoneHandle {
    state: Value,
}

impl Discard for DoneHandle {
    fn discard( self ) {
        js! { @(no_return)
            var state = @{&self.state};
            state.cancelled = true;
            state.callback.drop();
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
    /// [`PromiseFuture`](struct.PromiseFuture.html) instead.
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
    #[cfg(feature = "futures-support")]
    pub fn from_future< A >( future: A ) -> Self
        where A: TryFuture + 'static,
              A::Ok: JsSerialize,
              A::Error: JsSerialize {

        let future = future.into_future();

        #[inline]
        fn call< A: JsSerialize >( f: Reference, value: A ) {
            js! { @(no_return) @{f}( @{value} ); }
        }

        let callback = move |success: Reference, error: Reference| {
            spawn_local(
                future.then( move |result| {
                    match result {
                        Ok( a ) => call( success, a ),
                        Err( a ) => call( error, a ),
                    }

                    ready( () )
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
    /// If the `Promise` never succeeds / fails then the `callback` will never be called.
    ///
    /// This method returns a [`DoneHandle`](struct.DoneHandle.html). The [`DoneHandle`](struct.DoneHandle.html)
    /// *exclusively* owns the `callback`, so when the [`DoneHandle`](struct.DoneHandle.html) is dropped it will
    /// drop the `callback` and the `callback` will never be called. This will happen even if the `Promise` is not dropped!
    ///
    /// Dropping the [`DoneHandle`](struct.DoneHandle.html) does ***not*** cancel the `Promise`, because promises
    /// do not support cancellation.
    ///
    /// If you are no longer interested in the `Promise`'s result you can simply drop the [`DoneHandle`](struct.DoneHandle.html)
    /// and then the `callback` will never be called.
    ///
    /// But if you *are* interested in the `Promise`'s result, then you have two choices:
    ///
    /// * Keep the [`DoneHandle`](struct.DoneHandle.html) alive until after the `callback` is called (by storing it in a
    ///   variable or data structure).
    ///
    /// * Use the [`leak`](struct.DiscardOnDrop.html#method.leak) method to leak the [`DoneHandle`](struct.DoneHandle.html).
    ///   If the `Promise` never succeeds or fails then this ***will*** leak the memory of the callback, so only use
    ///   [`leak`](struct.DiscardOnDrop.html#method.leak) if you need to.
    ///
    /// # Examples
    ///
    /// Normal usage:
    ///
    /// ```rust
    /// let handle = promise.done(|result| {
    ///     match result {
    ///         Ok(success) => { ... },
    ///         Err(error) => { ... },
    ///     }
    /// });
    /// ```
    ///
    /// Leak the [`DoneHandle`](struct.DoneHandle.html) and `callback`:
    ///
    /// ```rust
    /// promise.done(|result| {
    ///     match result {
    ///         Ok(success) => { ... },
    ///         Err(error) => { ... },
    ///     }
    /// }).leak();
    /// ```
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/then)
    // https://www.ecma-international.org/ecma-262/6.0/#sec-performpromisethen
    pub fn done< A, B, F >( &self, callback: F ) -> DiscardOnDrop< DoneHandle >
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

        let state = js!(
            var callback = @{Once( callback )};

            var state = {
                cancelled: false,
                callback: callback
            };

            // TODO don't swallow any errors thrown inside callback
            @{self}.then( function ( value ) {
                if ( !state.cancelled ) {
                    callback( value, true );
                }
            }, function ( value ) {
                if ( !state.cancelled ) {
                    callback( value, false );
                }
            } );

            return state;
        );

        DiscardOnDrop::new( DoneHandle {
            state,
        } )
    }

    /// This method converts the `Promise` into a [`PromiseFuture`](struct.PromiseFuture.html), so that it can be used as a Rust
    /// [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html).
    ///
    /// This method should rarely be needed, instead use [`value.try_into()`](unstable/trait.TryInto.html) to convert directly
    /// from a [`Value`](enum.Value.html) into a [`PromiseFuture`](struct.PromiseFuture.html).
    ///
    /// # Examples
    ///
    /// ```rust
    /// promise.to_future().map(|x| x + 1)
    /// ```
    // We can't use the IntoFuture trait because Promise doesn't have a type argument
    // TODO explain more why we can't use the IntoFuture trait
    #[cfg(feature = "futures-support")]
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

/// A statically typed `Promise`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedPromise< T, E >( Promise, PhantomData< (T, E) > );

impl< T, E > TypedPromise< T, E >
    where T: TryFrom< Value >,
          E: TryFrom< Value >
{
    #[inline]
    pub(crate) fn new( promise: Promise ) -> Self {
        TypedPromise( promise, PhantomData )
    }

    /// A strongly typed version of [`Promise.done`](struct.Promise.html#method.done).
    #[inline]
    pub fn done< F >( &self, callback: F ) -> DiscardOnDrop< DoneHandle >
        where F: FnOnce( Result< T, E > ) + 'static,
              T::Error: fmt::Debug,
              E::Error: fmt::Debug
    {
        self.0.done( move |result| callback( result ) )
    }
}

impl< T, E > From< TypedPromise< T, E > > for Promise {
    #[inline]
    fn from( promise: TypedPromise< T, E > ) -> Promise {
        promise.0
    }
}

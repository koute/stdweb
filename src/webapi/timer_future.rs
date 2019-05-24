use std::sync::{Arc, Mutex};
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Waker, Context};
use webcore::once::Once;
use webcore::value::Value;
use futures_core::stream::Stream;
use futures_util::FutureExt;
use futures_channel::oneshot;


#[inline]
fn convert_to_i32( ms: u32 ) -> i32 {
    let ms: i32 = ms as i32;

    assert!( ms >= 0, "ms must be less than 2147483648" );

    ms
}


/// The [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html)
/// which is returned by [`wait`](fn.wait.html).
// This isn't implemented as a PromiseFuture because Promises do not support cancellation
#[derive( Debug )]
pub struct Wait {
    receiver: oneshot::Receiver< () >,
    timer: Value,
}

impl Wait {
    fn new( ms: u32 ) -> Self {
        // We accept a u32 because we don't want negative values, however setTimeout requires it to be i32
        let ms = convert_to_i32( ms );

        let ( sender, receiver ) = oneshot::channel();

        let callback = move || {
            // TODO is this correct ?
            match sender.send( () ) {
                Ok( _ ) => {},
                Err( _ ) => {},
            };
        };

        let timer = js!(
            var callback = @{Once( callback )};

            return {
                callback: callback,
                id: setTimeout( function () {
                    callback();
                }, @{ms} )
            };
        );

        Self {
            receiver,
            timer,
        }
    }
}

impl Future for Wait {
    type Output = ();

    #[inline]
    fn poll( mut self: Pin< &mut Self >, cx: &mut Context ) -> Poll< Self::Output > {
        // TODO is this unwrap correct ?
        self.receiver.poll_unpin( cx ).map( |x| x.unwrap() )
    }
}

impl Drop for Wait {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            var timer = @{&self.timer};
            clearTimeout( timer.id );
            timer.callback.drop();
        }
    }
}

/// Creates a [`Future`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/future/trait.Future.html)
/// which will return `()` after `ms` milliseconds have passed.
///
/// It might return a long time *after* `ms` milliseconds have passed, but it
/// will never return *before* `ms` milliseconds have passed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setTimeout)
// https://html.spec.whatwg.org/multipage/webappapis.html#dom-settimeout
#[inline]
pub fn wait( ms: u32 ) -> Wait {
    Wait::new( ms )
}


#[derive( Debug )]
struct IntervalBufferedState {
    waker: Option< Waker >,
    count: usize,
}

/// The [`Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/stream/trait.Stream.html)
/// which is returned by [`interval_buffered`](fn.interval_buffered.html).
#[derive( Debug )]
pub struct IntervalBuffered {
    state: Arc< Mutex< IntervalBufferedState > >,
    timer: Value,
}

impl IntervalBuffered {
    fn new( ms: u32 ) -> Self {
        // We accept a u32 because we don't want negative values, however setInterval requires it to be i32
        let ms = convert_to_i32( ms );

        let state = Arc::new( Mutex::new( IntervalBufferedState {
            waker: None,
            count: 0,
        } ) );

        let callback = {
            let state = state.clone();

            move || {
                let mut lock = state.lock().unwrap();

                lock.count += 1;

                if let Some( waker ) = lock.waker.take() {
                    drop( lock );
                    waker.wake();
                }
            }
        };

        let timer = js!(
            var callback = @{callback};

            return {
                callback: callback,
                id: setInterval( function () {
                    callback();
                }, @{ms} )
            };
        );

        Self {
            state,
            timer,
        }
    }
}

impl Stream for IntervalBuffered {
    type Item = ();

    fn poll_next( self: Pin< &mut Self >, cx: &mut Context ) -> Poll< Option< Self::Item > > {
        let mut lock = self.state.lock().unwrap();

        if lock.count == 0 {
            lock.waker = Some( cx.waker().clone() );
            Poll::Pending

        } else {
            lock.count -= 1;

            Poll::Ready( Some( () ) )
        }
    }
}

impl Drop for IntervalBuffered {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            var timer = @{&self.timer};
            clearInterval( timer.id );
            timer.callback.drop();
        }
    }
}

/// Creates a [`Stream`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.13/futures/stream/trait.Stream.html)
/// which will continuously output `()` every `ms` milliseconds, until it is dropped.
///
/// It might output `()` a long time *after* `ms` milliseconds have passed, but it
/// will never output `()` *before* `ms` milliseconds have passed.
///
/// If the consumer isn't ready to receive the `()`, it will be put into a queue
/// (this queue is ***very*** fast, it can handle a very large number of elements).
///
/// When the consumer is ready, it will output all of the `()` from the queue.
///
/// That means that if the consumer is too slow, it might receive multiple `()` at the same time.
/// Or it might receive another `()` before `ms` milliseconds have passed for the consumer
/// (because `ms` milliseconds *have* passed for the [`IntervalBuffered`](struct.IntervalBuffered.html)).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setInterval)
// https://html.spec.whatwg.org/multipage/webappapis.html#dom-setinterval
#[inline]
pub fn interval_buffered( ms: u32 ) -> IntervalBuffered {
    IntervalBuffered::new( ms )
}

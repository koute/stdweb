use std::sync::{Arc, Mutex};
use webcore::once::Once;
use webcore::value::Value;
use webapi::error::Error;
use futures_core::{Future, Poll, Async};
use futures_core::task::{Waker, Context};
use futures_core::stream::Stream;
use futures_channel::oneshot;


#[inline]
fn convert_to_i32( ms: u32 ) -> i32 {
    let ms: i32 = ms as i32;

    assert!( ms >= 0, "ms must be less than 2147483648" );

    ms
}


///
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
    type Item = ();
    // TODO use Void instead
    type Error = Error;

    #[inline]
    fn poll( &mut self, cx: &mut Context ) -> Poll< Self::Item, Self::Error > {
        self.receiver.poll( cx ).map_err( |_| unreachable!() )
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

///
#[inline]
pub fn wait( ms: u32 ) -> Wait {
    Wait::new( ms )
}


#[derive( Debug )]
struct IntervalBufferedState {
    waker: Option< Waker >,
    count: usize,
}

///
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
    // TODO use Void instead
    type Error = Error;

    fn poll_next( &mut self, cx: &mut Context ) -> Poll< Option< Self::Item >, Self::Error > {
        let mut lock = self.state.lock().unwrap();

        if lock.count == 0 {
            lock.waker = Some( cx.waker().clone() );
            Ok( Async::Pending )

        } else {
            lock.count -= 1;

            Ok( Async::Ready( Some( () ) ) )
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

///
#[inline]
pub fn interval_buffered( ms: u32 ) -> IntervalBuffered {
    IntervalBuffered::new( ms )
}

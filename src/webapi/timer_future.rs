use webcore::once::Once;
use webcore::try_from::TryInto;
use webcore::value::Value;
use webapi::error::Error;
use futures::{Future, Poll, Stream};
use futures::unsync::oneshot;
use futures::sync::mpsc;


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
    callback: Value,
    timer_id: u32,
}

impl Wait {
    fn new( ms: u32 ) -> Self {
        // We accept a u32 because we don't want negative values, however setTimeout requires it to be i32
        let ms = convert_to_i32( ms );

        let ( sender, receiver ) = oneshot::channel();

        let callback = move || {
            sender.send( () ).unwrap();
        };

        let callback = js!( return @{Once( callback )}; );

        let timer_id = js!(
            return setTimeout( function () {
                @{&callback}();
            }, @{ms} );
        ).try_into().unwrap();

        Self {
            receiver,
            callback,
            timer_id,
        }
    }
}

impl Future for Wait {
    type Item = ();
    // TODO use Void instead
    type Error = Error;

    #[inline]
    fn poll( &mut self ) -> Poll< Self::Item, Self::Error > {
        self.receiver.poll().map_err( |_| unreachable!() )
    }
}

impl Drop for Wait {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            clearTimeout( @{self.timer_id} );
            @{&self.callback}.drop();
        }
    }
}

///
#[inline]
pub fn wait( ms: u32 ) -> Wait {
    Wait::new( ms )
}


///
#[derive( Debug )]
pub struct IntervalBuffered {
    receiver: mpsc::UnboundedReceiver< () >,
    callback: Value,
    timer_id: u32,
}

impl IntervalBuffered {
    fn new( ms: u32 ) -> Self {
        // We accept a u32 because we don't want negative values, however setInterval requires it to be i32
        let ms = convert_to_i32( ms );

        let ( sender, receiver ) = mpsc::unbounded();

        let callback = move || {
            sender.unbounded_send( () ).unwrap();
        };

        let callback = js!( return @{callback}; );

        let timer_id = js!(
            return setInterval( function () {
                @{&callback}();
            }, @{ms} );
        ).try_into().unwrap();

        Self {
            receiver,
            callback,
            timer_id,
        }
    }
}

impl Stream for IntervalBuffered {
    type Item = ();
    // TODO use Void instead
    type Error = Error;

    #[inline]
    fn poll( &mut self ) -> Poll< Option< Self::Item >, Self::Error > {
        self.receiver.poll().map_err( |_| unreachable!() )
    }
}

impl Drop for IntervalBuffered {
    #[inline]
    fn drop( &mut self ) {
        js! { @(no_return)
            clearInterval( @{self.timer_id} );
            @{&self.callback}.drop();
        }
    }
}

///
#[inline]
pub fn interval_buffered( ms: u32 ) -> IntervalBuffered {
    IntervalBuffered::new( ms )
}

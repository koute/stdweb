#[macro_use]
extern crate stdweb;
extern crate futures;

use futures::Future;
use stdweb::unstable::{TryInto};
use stdweb::web::error::Error;
use stdweb::{Null, PromiseFuture};


fn sleep( ms: u32 ) -> PromiseFuture< Null > {
    js!( return new Promise( function ( success, failure ) {
        setTimeout( function () {
            success( null );
        }, @{ms} );
    } ); ).try_into().unwrap()
}


fn log( a: &str ) {
    js! { @(no_return)
        console.log( @{a} );
    }
}


struct MyFuture {
    count: u32,
    receiver: futures::unsync::oneshot::Receiver< () >,
}

impl MyFuture {
    fn new() -> Self {
        let ( sender, receiver ) = futures::unsync::oneshot::channel();

        let callback = || {
            log( "setTimeout done" );

            log( &format!("Sending {:#?}", sender.send( () ) ) );
        };

        log( "setTimeout started" );

        js! { @(no_return)
            setTimeout( function () {
                @{stdweb::Once( callback )}();
            }, 1000 );
        }

        Self {
            count: 0,
            receiver,
        }
    }
}

impl Future for MyFuture {
    type Item = u32;
    type Error = ();

    fn poll( &mut self ) -> futures::Poll< Self::Item, Self::Error > {
        self.count += 1;

        let task = futures::task::current();

        task.notify();
        task.notify();

        match self.receiver.poll() {
            Ok( futures::Async::Ready( () ) ) => Ok( futures::Async::Ready( self.count ) ),
            Ok( futures::Async::NotReady ) => Ok( futures::Async::NotReady ),
            Err( _ ) => Err( () ),
        }
    }
}


fn main() {
    stdweb::initialize();

    PromiseFuture::spawn(
        MyFuture::new().map( |x| {
            log( &format!( "MyFuture count: {}", x ) );
            ()
        } )
    );

    PromiseFuture::spawn(
        sleep( 2000 ).inspect( |_| log( "Timeout 1 done!") ).join(
        sleep( 2000 ).inspect( |_| log( "Timeout 2 done!" ) ) )
            .and_then( |_|
                sleep( 1000 ).inspect( |_| log( "Timeout 3 done!") ) )
            .and_then( |_|
                futures::future::err( Error::new( "Testing error!" ) ) )
            .map_err( |e| e.print() )
    );

    stdweb::event_loop();
}

#![feature(async_await, await_macro)]

#[macro_use]
extern crate stdweb;

use futures::future::{join, try_join};
use stdweb::{PromiseFuture, spawn_local, unwrap_future};
use stdweb::web::wait;
use stdweb::web::error::Error;
use stdweb::unstable::TryInto;


// Converts a JavaScript Promise into a Rust Future
fn javascript_promise() -> PromiseFuture< u32 > {
    js!(
        return new Promise( function ( success, error ) {
            setTimeout( function () {
                success( 50 );
            }, 2000 );
        } );
    ).try_into().unwrap()
}


async fn print( message: &str ) {
    // Waits for 2000 milliseconds
    await!( wait( 2000 ) );
    console!( log, message );
}


async fn future_main() -> Result< (), Error > {
    // Runs Futures synchronously
    await!( print( "Hello" ) );
    await!( print( "There" ) );

    {
        // Runs multiple Futures in parallel
        let ( a, b ) = await!( join(
            print( "Test 1" ),
            print( "Test 2" ),
        ) );

        console!( log, "join", a, b );
    }

    {
        // Runs multiple Futures (which can error) in parallel
        let ( a, b ) = await!( try_join(
            javascript_promise(),
            javascript_promise(),
        ) )?;

        console!( log, "try_join", a, b );
    }

    Ok( () )
}


fn main() {
    stdweb::initialize();

    spawn_local( unwrap_future( future_main() ) );

    stdweb::event_loop();
}

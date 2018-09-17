#![feature(async_await, await_macro, futures_api)]

#[macro_use]
extern crate stdweb;

use stdweb::{PromiseFuture, spawn_local, unwrap_future};
use stdweb::web::wait;
use stdweb::web::error::Error;
use stdweb::unstable::TryInto;


fn javascript_promise() -> PromiseFuture< u32 > {
    js!(
        return new Promise( function ( success, error ) {
            setTimeout( function () {
                success( 50 );
            }, 1000 );
        } );
    ).try_into().unwrap()
}


async fn future_main() -> Result< (), Error > {
    await!( wait( 1000 ) );
    console!( log, "Hello" );
    await!( wait( 1000 ) );
    console!( log, "There" );
    console!( log, await!( javascript_promise() )? );
    Ok( () )
}


fn main() {
    stdweb::initialize();

    spawn_local( unwrap_future( future_main() ) );

    stdweb::event_loop();
}

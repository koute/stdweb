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


fn main() {
    stdweb::initialize();

    PromiseFuture::spawn(
        sleep( 5000 ).inspect( |_| log( "Timeout 1 done!") ).join(
        sleep( 5000 ).inspect( |_| log( "Timeout 2 done!" ) ) )
            .and_then( |_|
                sleep( 5000 ).inspect( |_| log( "Timeout 3 done!") ) )
            .and_then( |_|
                futures::future::err( Error::new( "Testing error!" ) ) )
            .map_err( |e| e.print() )
    );

    stdweb::event_loop();
}

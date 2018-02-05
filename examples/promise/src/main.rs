#[macro_use]
extern crate stdweb;
extern crate futures;

use futures::Future;
use stdweb::unstable::{TryInto};
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
                sleep( 5000 ).inspect( |_| log( "Timeout 3 done!") ) ).map( |_| () )
    );

    stdweb::event_loop();
}

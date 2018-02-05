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


fn main() {
    stdweb::initialize();

    PromiseFuture::spawn(
        sleep( 5000 ).inspect( |_| println!( "Timeout 1 done!") ).join(
        sleep( 5000 ).inspect( |_| println!( "Timeout 2 done!" ) ) )
            .and_then( |_|
                sleep( 5000 ).inspect( |_| println!( "Timeout 3 done!") ) ).map( |_| () )
    );

    stdweb::event_loop();
}

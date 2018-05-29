#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    let message = "Hello, 世界!";
    js! {
        alert( @{message} );
    }

    stdweb::event_loop();
}

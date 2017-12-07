#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();
    js! { console.log( @{"Hello, 世界!"} ) };
    stdweb::event_loop();
}

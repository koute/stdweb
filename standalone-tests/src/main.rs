#![recursion_limit="128"]

// These are standalone tests for wasm32-unknown-unknown.
//
// We can't put these in a normal crate as currently most
// of the `std` on `wasm32-unknown-unknown` is stubbed out,
// so **if** something fails then debugging it is going to be
// a very miserable process.

#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate stdweb_derive;

use std::panic;

#[macro_use]
mod utils;
mod tests;

pub use tests::exports::*;

fn main() {
    panic::set_hook( Box::new( |_| {
        eprintln!( "Encountered a panic!" );
        utils::exit( 1 );
    }));

    tests::run_all_tests();
    eprintln!( "All tests passed!" );
}

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test;

#[macro_use]
extern crate stdweb;

use std::time::Duration;

use stdweb::web::document;
use stdweb::web::event::ClickEvent;
use stdweb::unstable::TryInto;
use stdweb::traits::*;

macro_rules! println {
    ($($token:tt)*) => {
        let string = format!( $($token)+ );
        let console = document().query_selector( "#console" ).unwrap().unwrap();
        console.append_child( &document().create_text_node( &string ) );
        console.append_child( &document().create_text_node( "\n" ) );
        js! {
            console.log( @{string} );
        }
    };
}

mod utils;
use utils::Stopwatch;

struct Timer {
    timestamp: f64
}

impl Stopwatch for Timer {
    fn now() -> Self {
        let timestamp: f64 = js!( return performance.now(); ).try_into().unwrap();
        Timer {
            timestamp: timestamp / 1000_f64
        }
    }

    fn elapsed( &self ) -> Duration {
        let now = Timer::now();
        let difference = now.timestamp - self.timestamp;
        let secs = difference.trunc();
        Duration::new( secs as u64, ((difference - secs) * 1_000_000_000.0) as u32 )
    }
}

struct Bench {
    name: String,
    callback: Box< Fn( &Bench ) + 'static >
}

struct Bencher {
    benches: Vec< Bench >
}

impl Bencher {
    fn new() -> Self {
        Bencher { benches: Vec::new() }
    }

    fn add< R, F: 'static + Fn() -> R >( &mut self, name: &str, callback: F ) {
        self.benches.push( Bench {
            name: name.to_owned(),
            callback: Box::new( move |bench| {
                println!( "Benchmarking '{}'...", bench.name );
                let result = utils::benchmark::< Timer, R, F >( &callback );
                println!( "    {}", result );
            })
        });
    }

    fn run( &self ) {
        for bench in &self.benches {
            (bench.callback)( bench );
        }
    }
}

fn main() {
    let body = document().query_selector( "body" ).unwrap().unwrap();
    let start = document().create_element( "button" ).unwrap();
    start.set_text_content( "Start" );
    body.append_child( &start );
    let pre = document().create_element( "pre" ).unwrap();
    pre.set_attribute( "id", "console" ).unwrap();
    body.append_child( &pre );

    if cfg!( nightly ) {
        js! {
            console.log( "Compiled with the `nightly` feature!" );
        }
    }

    let mut bencher = Bencher::new();
    bencher.add( "call-into-js", || js!( @(no_return) ) );
    bencher.add( "call-into-js-returning-undefined", || js!() );
    bencher.add( "call-into-js-with-string", || js!( @(no_return) var test = @{"Hello world!"}; ) );

    body.add_event_listener( move |_: ClickEvent| {
        bencher.run();
    });
}

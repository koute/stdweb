#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test;

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use std::time::Duration;
use std::collections::HashMap;

use stdweb::Reference;
use stdweb::web::document;
use stdweb::web::event::ClickEvent;
use stdweb::unstable::TryInto;
use stdweb::traits::*;

lazy_static! {
    static ref IS_NODEJS: bool = {
        let is_nodejs: bool = js!(
            return typeof window === "undefined" && typeof process === "object";
        ).try_into().unwrap();

        return is_nodejs;
    };
}

macro_rules! println {
    ($($token:tt)*) => {
        let string = format!( $($token)+ );
        if !*IS_NODEJS {
            let console = document().query_selector( "#console" ).unwrap().unwrap();
            console.append_child( &document().create_text_node( &string ) );
            console.append_child( &document().create_text_node( "\n" ) );
        }
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
        if !*IS_NODEJS {
            let timestamp: f64 = js!( return performance.now(); ).try_into().unwrap();
            Timer {
                timestamp: timestamp / 1000_f64
            }
        } else {
            let timestamp: f64 = js!(
                var t = process.hrtime();
                return t[ 0 ] + t[ 1 ] / 1000000000;
            ).try_into().unwrap();

            Timer { timestamp }
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

    fn add< R, F: 'static + Fn() -> R, G: 'static + Fn() -> F >( &mut self, name: &str, callback: G ) {
        self.benches.push( Bench {
            name: name.to_owned(),
            callback: Box::new( move |bench| {
                println!( "Benchmarking '{}'...", bench.name );
                let callback = callback();
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

fn run_benchmarks< F: FnOnce( &mut Bencher ) >( callback: F ) {
    if !*IS_NODEJS {
        let body = document().query_selector( "body" ).unwrap().unwrap();
        let start = document().create_element( "button" ).unwrap();
        start.set_text_content( "Start" );
        body.append_child( &start );
        let pre = document().create_element( "pre" ).unwrap();
        pre.set_attribute( "id", "console" ).unwrap();
        body.append_child( &pre );
    }

    if cfg!( nightly ) {
        js! {
            console.log( "Compiled with the `nightly` feature!" );
        }
    }

    let mut bencher = Bencher::new();
    callback( &mut bencher );

    if !*IS_NODEJS {
        let body = document().query_selector( "body" ).unwrap().unwrap();
        body.add_event_listener( move |_: ClickEvent| {
            bencher.run();
        });
    } else {
        bencher.run();
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
struct Struct {
    number: i32,
    boolean: bool,
    string: String,
    array: Vec< i32 >,
    object: HashMap< String, i32 >
}

js_serializable!( Struct );

fn main() {
    run_benchmarks( |bencher| {
        bencher.add( "call-into-js", || || js!( @(no_return) ) );
        bencher.add( "call-into-js-returning-undefined", || || js!() );
        bencher.add( "call-into-js-returning-bool", || || js!( return true; ) );
        bencher.add( "call-into-js-returning-string", || || js!( return "Hello world!"; ) );
        bencher.add( "call-into-js-with-string", || || js!( @(no_return) var test = @{"Hello world!"}; ) );
        bencher.add( "call-into-js-with-bool", || || js!( @(no_return) var test = @{true}; ) );
        bencher.add( "call-into-js-with-array", || || js!( @(no_return) var test = @{&[1, 2, 3, 4, 5, 6, 7, 8][..]}; ) );
        bencher.add( "call-into-js-with-object", || {
            let object: HashMap< _, _ > =
                vec![
                    ("One".to_owned(), 1),
                    ("Two".to_owned(), 2),
                    ("Three".to_owned(), 3)
                ].into_iter().collect();

            move || js!( @(no_return) var test = @{&object}; )
        });
        bencher.add( "call-into-js-with-serde-struct", || {
            let structure = Struct {
                number: 123,
                boolean: true,
                string: "Hello world!".to_owned(),
                array: vec![ 1, 2, 3, 4, 5, 6, 7, 8 ],
                object: vec![
                    ("One".to_owned(), 1),
                    ("Two".to_owned(), 2),
                    ("Three".to_owned(), 3)
                ].into_iter().collect()
            };

            move || js!( @(no_return) var test = @{&structure}; )
        });
        bencher.add( "call-into-js-with-reference", || {
            let reference: Reference = js!( return [@{0}]; ).try_into().unwrap();
            move || js!( @(no_return) var test = @{&reference}; )
        });
        bencher.add( "call-into-js-with-reference-while-having-a-lot-of-references", || {
            let references: Vec< Reference > = (0..1000).into_iter().map( |nth| {
                js!( return [@{nth}]; ).try_into().unwrap()
            }).collect();

            move || js!( @(no_return) var test = @{&references[0]}; )
        });
    });
}

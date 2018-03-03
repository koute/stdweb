#[macro_use]
extern crate stdweb;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::web::{get_current_position, watch_position, Position, GeoWatchHandle};

fn main() {
    stdweb::initialize();

    get_current_position(|x: Position| {
        js! {
            console.log("cur pos");
            console.log(@{&x});
        };
    });

    let iter = Rc::new(RefCell::new(0));
    let handle: Arc<Mutex<Option<GeoWatchHandle>>> = Arc::new(Mutex::new(None));
    {
        let h = Arc::clone(&handle);
        *(h.lock().unwrap()) = Some(watch_position(move |x: Position| {
            let mut i = iter.borrow_mut();
            *i += 1;
            js! {
                // Print watch details.
                console.log("watch pos");
                console.log(@{&(*i)});
                console.log(@{&x});
            };
            if *i >= 5 {
                js! {
                    console.log("stop watching");
                };
                let handle = Arc::clone(&handle);
                let mut guard = handle.lock().unwrap();
                if let Some(h) = guard.take() {
                    h.clear_watch();
                }
            }
        }));
    }

    stdweb::event_loop();
}

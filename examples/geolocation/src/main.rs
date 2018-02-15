#[macro_use]
extern crate stdweb;

use std::sync::{Arc, Mutex};
use stdweb::web::{Geolocation, Position, WatchId};

fn main() {
    stdweb::initialize();

    Geolocation::get_current_position(|x: Position| {
        js! {
            console.log("cur pos");
            console.log(@{&x});
        };
    });

    // We create a watch identifier that is wrapped in a mutex / arc so that it can be used both in
    // the creation and in the callback function for watched events.
    let iter = Arc::new(Mutex::new(0));
    let watch_id = Arc::new(Mutex::new(WatchId::default()));
    {
        let awatch_id = Arc::clone(&watch_id);
        let mut id = awatch_id.lock().unwrap();
        *id = Geolocation::watch_position(move |x: Position| {
            let iter = Arc::clone(&iter);
            let mut i = iter.lock().unwrap();
            let i_cur = *i;

            js! {
                // Print watch details.
                console.log("watch pos");
                console.log(@{&i_cur});
                console.log(@{&x});
            };

            *i += 1;
            if *i >= 5 {
                js! {
                    console.log("stop watching");
                };
                // If the number of iterations greater than, equal to 5 then we're going to clear
                // the watch and bail out of this system.
                let watch_id = watch_id.clone();
                let id = watch_id.lock().unwrap();
                Geolocation::clear_watch(&*id);
            }
        });
    }

    stdweb::event_loop();
}

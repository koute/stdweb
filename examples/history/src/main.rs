#[macro_use]
extern crate stdweb;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;

use stdweb::unstable::TryInto;
use stdweb::serde::Serde;
use stdweb::web::event::PopStateEvent;
use stdweb::web::{
    IEventTarget,
    History,
    set_timeout,
    window,
};

#[derive(Debug, Deserialize, Serialize)]
struct ExampleState {
    numero: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WrongState {
    cheese: String,
}

js_serializable!(ExampleState);

fn pop_listener(event: PopStateEvent) {
    let state: Option<Serde<ExampleState>> = event.state().try_into().ok();
    let state_str = format!("state_str({:?})", state);
    js!(console.log("pop event!!", @{state_str}, @{state}));
    let wrong_state: Option<Serde<WrongState>> = event.state().try_into().ok();
    let wrong_state_str = format!("wrong_state_str({:?})", wrong_state);
    js!(console.log("should be null", @{wrong_state_str}));
}

fn main() {
    stdweb::initialize();
    let history: History = window().history();
    let state = ExampleState { numero: 1 };
    window().add_event_listener(pop_listener);
    let history_length = history.len() as u32;
    console!(log, "number of history entries: ", history_length);
    history.push_state(state, "", Some("cat_pics.html"));
    set_timeout(|| window().history().back().unwrap(), 2000);
    set_timeout(|| window().history().forward().unwrap(), 5000);
    set_timeout(|| window().history().back().unwrap(), 8000);
    set_timeout(move || {
        let history_length = window().history().len() as u32;
        console!(log, "number of history entries: ", history_length);
    }, 9000);
}

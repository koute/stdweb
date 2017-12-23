#[macro_use]
extern crate stdweb;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json as json;

use stdweb::unstable::TryInto;
use stdweb::web::{
    History,
    set_timeout,
    window,
};

#[derive(Serialize)]
struct ExampleState {
    numero: i32,
}

js_serializable!(ExampleState);

use std::rc::Rc;

fn main() {
    stdweb::initialize();
    let history: History = window().history();
    let state = ExampleState { numero: 1 };
    history.push_state(state, None, "meow.html".to_string());
    set_timeout(move || window().history().back(), 2000);
    set_timeout(move || window().history().forward(), 5000);
}

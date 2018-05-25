#[macro_use]
extern crate stdweb;

use stdweb::web::{
    IEventTarget,
    IGamepad,
    window,
};

use stdweb::web::event::{
    GamepadConnectedEvent,
    IGamepadEvent,
};

fn main() {
    stdweb::initialize();

    let message = "hello rust!";

    js! {
        alert( @{message} );
    }

    window().add_event_listener( move |e: GamepadConnectedEvent| {
        let message = format!("gamepad \"{}\" connected in rust!", e.gamepad().id());

        js! {
            alert( @{message} );
        }
    });

    stdweb::event_loop();
}

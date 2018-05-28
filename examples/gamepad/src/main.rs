extern crate stdweb;

use stdweb::traits::*;
use stdweb::web::{
    document,
    Element,
    Gamepad,
    GamepadMappingType,
    IEventTarget,
    window,
};
use stdweb::web::event::{
    GamepadConnectedEvent,
    GamepadDisconnectedEvent,
    IGamepadEvent,
};

/// Create an element and set its content.
fn elem_content(elem_type: &str, content: &str) -> Element {
    let elem = document().create_element(elem_type).unwrap();
    elem.set_text_content(content);
    elem
}

/// Write a new line to the "log" div.
fn log(msg: &str) {
    let log_div = document().query_selector("#log").unwrap().unwrap();

    log_div.append_child(&elem_content("p", msg));
}

fn get_pad_title(pad: &Gamepad) -> Element {
    let div = document().create_element("div").unwrap();

    div.append_child(&elem_content("h2",
        &format!("Pad {}: {}", pad.index(), pad.id())
    ));

    div.append_child(&elem_content("h3",
        if pad.connected() { "Connected" } else { "Disconnected" }
    ));

    div.append_child(&elem_content("h3",
        &format!("Mapping: {}", match pad.mapping() { GamepadMappingType::Standard => "standard", _ => "non-standard" })
    ));

    div.append_child(&elem_content("h3",
        &format!("Last updated: {:.0}", pad.timestamp())
    ));

    div
}

fn get_pad_axes(pad: &Gamepad) -> Element {
    let div = document().create_element("div").unwrap();

    for (i, a) in pad.axes().into_iter().enumerate() {
        let elem = elem_content("p",
            &format!("Axis {}: {}", i, a)
        );

        if a != 0.0 {
            elem.set_attribute("class", "gp-pressed").unwrap();
        }

        div.append_child(&elem);
    }

    div
}

fn get_pad_buttons(pad: &Gamepad) -> Element {
    let div = document().create_element("div").unwrap();

    for (i, b) in pad.buttons().into_iter().enumerate() {
        let elem = elem_content("p",
            &format!("Button {}: Pressed = {}; Value = {}", i, b.pressed(), b.value())
        );

        if b.value() != 0.0 {
            elem.set_attribute("class", "gp-pressed").unwrap();
        }

        div.append_child(&elem);
    }

    div
}

fn get_pad_state(pad: &Option<Gamepad>) -> Element {
    let elem = document().create_element("div").unwrap();

    match pad {
        Some(pad) => {
            elem.append_child(&get_pad_title(&pad));
            elem.append_child(&get_pad_axes(&pad));
            elem.append_child(&get_pad_buttons(&pad));
        },
        None => {
            elem.append_child(&elem_content("h2", "No pad"));
        }
    }

    elem
}

/// Update gamepad state view
fn animate() {
    let list = document().create_element("ul").unwrap();

    for pad in Gamepad::get_all() {
        let item = document().create_element("li").unwrap();
        item.append_child(&get_pad_state(&pad));
        list.append_child(&item);
    }

    let state = document().query_selector("#state").unwrap().unwrap();

    state.set_text_content("");
    state.append_child(&list);

    // queue another animate() on the next frame
    window().request_animation_frame(|_| animate());
}

fn main() {
    stdweb::initialize();

    log("Waiting for gamepad connection...");

    animate();

    window().add_event_listener( move |e: GamepadConnectedEvent| {
        let pad = e.gamepad();
        let message = format!("gamepad \"{}\" connected", pad.id());
        log(&message);
    });

    window().add_event_listener( move |e: GamepadDisconnectedEvent| {
        let pad = e.gamepad();
        let message = format!("gamepad \"{}\" disconnected", pad.id());
        log(&message);
    });

    stdweb::event_loop();
}

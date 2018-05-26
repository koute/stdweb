extern crate stdweb;

use std::rc::Rc;
use std::cell::RefCell;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    Gamepad,
    GamepadMappingType,
    HtmlElement,
    IEventTarget,
    IGamepad,
    IGamepadButton,
    window,
};
use stdweb::web::event::{
    GamepadConnectedEvent,
    GamepadDisconnectedEvent,
    IGamepadEvent,
};

fn log(msg: &str) {
    let log_div: HtmlElement = document().query_selector( ".log" ).unwrap().unwrap().try_into().unwrap();

    let p = document().create_element("p").unwrap();
    p.set_text_content(msg);
    log_div.append_child(&p);
}

struct State {
    gamepads: Vec<Gamepad>,
    div: HtmlElement,
}

impl State {

    fn new(div: HtmlElement) -> Self {
        Self {
            gamepads: vec![],
            div,
        }
    }

    fn add(&mut self, pad: Gamepad) {
        self.gamepads.push(pad);
    }

    fn remove(&mut self, pad: Gamepad) {
        self.gamepads.retain(|p| p.index() != pad.index());
    }

    fn animate(&self, rc: Rc<RefCell<Self>>) {

        let list = document().create_element("ul").unwrap();

        for (i, pad) in self.gamepads.iter().enumerate() {
            let item = document().create_element("li").unwrap();

            let title = format!("pad[{}] \"{}\" {}; {} mapping; last update = {:.0}",
                i,
                pad.id(),
                if pad.connected() { "connected" } else { "disconnected" },
                match pad.mapping() { GamepadMappingType::Standard => "standard", _ => "nonstandard" },
                pad.timestamp()
            );

            let title_p = document().create_element("p").unwrap();
            title_p.set_text_content(&title);

            item.append_child(&title_p);

            for (i, a) in pad.axes().into_iter().enumerate() {
                let axis = document().create_element("p").unwrap();
                axis.set_text_content(&format!("axis[{}] = {}", i, a));
                item.append_child(&axis);
            }

            for (i, b) in pad.buttons().into_iter().enumerate() {
                let button = document().create_element("p").unwrap();
                button.set_text_content(&format!("button[{}] pressed = {}; value = {}",
                    i,
                    b.pressed(),
                    b.value()
                ));
                item.append_child(&button);
            }

            list.append_child(&item);
        }

        self.div.set_text_content("");
        self.div.append_child(&list);

        window().request_animation_frame(move |_| {
            rc.borrow().animate(rc.clone());
        });
    }
}

fn main() {
    stdweb::initialize();

    log("Waiting for gamepad connection...");

    let state_div: HtmlElement = document().query_selector( ".state" ).unwrap().unwrap().try_into().unwrap();

    let state_rc = Rc::new(RefCell::new(State::new(state_div)));

    state_rc.borrow().animate(state_rc.clone());

    let state_rc1 = state_rc.clone();

    window().add_event_listener( move |e: GamepadConnectedEvent| {
        let pad = e.gamepad();

        let message = format!("gamepad \"{}\" connected", pad.id());
        log(&message);

        state_rc1.borrow_mut().add(pad);
    });

    window().add_event_listener( move |e: GamepadDisconnectedEvent| {
        let pad = e.gamepad();

        let message = format!("gamepad \"{}\" disconnected", pad.id());
        log(&message);

        state_rc.borrow_mut().remove(pad);
    });

    stdweb::event_loop();
}

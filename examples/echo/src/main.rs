extern crate stdweb;

use std::rc::Rc;

use stdweb::unstable::TryInto;
use stdweb::web::{
    IEventTarget,
    INode,
    HtmlElement,
    document,
    WebSocket,
};

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    IMessageEvent,
    KeyPressEvent,
    SocketOpenEvent,
    SocketCloseEvent,
    SocketErrorEvent,
    SocketMessageEvent,
};

use stdweb::web::html_element::InputElement;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    stdweb::initialize();

    let output_div: HtmlElement = document().query_selector( ".output" ).unwrap().try_into().unwrap();
    let output_msg = Rc::new(move |msg: &str| {
        let elem = document().create_element("p");
        elem.set_text_content(msg);
        if let Some(child) = output_div.first_child() {
            output_div.insert_before(&elem, &child);
        } else {
            output_div.append_child(&elem);
        }
    });

    output_msg("> Connecting...");

    let ws = WebSocket::new("wss://echo.websocket.org").unwrap();

    ws.add_event_listener( enclose!( (output_msg) move |_: SocketOpenEvent| {
        output_msg("> Opened connection");
    }));

    ws.add_event_listener( enclose!( (output_msg) move |_: SocketErrorEvent| {
        output_msg("> Connection Errored");
    }));

    ws.add_event_listener( enclose!( (output_msg) move |event: SocketCloseEvent| {
        output_msg(&format!("> Connection Closed: {}", event.reason()));
    }));

    ws.add_event_listener( enclose!( (output_msg) move |event: SocketMessageEvent| {
        output_msg(&event.data().into_text().unwrap());
    }));

    let text_entry: InputElement = document().query_selector( ".form input" ).unwrap().try_into().unwrap();
    text_entry.add_event_listener( enclose!( (text_entry) move |event: KeyPressEvent| {
        if event.key() == "Enter" {
            event.prevent_default();

            let text: String = text_entry.value().try_into().unwrap();
            if text.is_empty() == false {
                text_entry.set_value("");
                ws.send_text(&text);
            }
        }
    }));

    stdweb::event_loop();
}

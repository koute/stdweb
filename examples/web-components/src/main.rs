extern crate stdweb;

use stdweb::unstable::TryInto;
use stdweb::traits::*;
use stdweb::web;
use web::html_element::TemplateElement;
use web::{CloneKind, ShadowRootMode};

fn main() {
    stdweb::initialize();
    let document = web::document();

    // let div1 = document.querySelector("#div1");
    let div1 = document.query_selector("#div1").unwrap().unwrap();
    // let tpl = document.querySelector("#tpl");
    let tpl: TemplateElement = document
        .query_selector("#tpl")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    // let shadowRoot1 = div1.attachShadow( { mode: "open" } );
    let shadow_root1 = div1.attach_shadow(ShadowRootMode::Open).unwrap();

    // let n = tpl.content.cloneNode(true);
    let n = tpl.content().clone_node(CloneKind::Deep).unwrap();

    // shadowRoot1.appendChild(n);
    shadow_root1.append_child(&n);

    stdweb::event_loop();
}

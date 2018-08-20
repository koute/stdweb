#[macro_use]
extern crate stdweb;

use std::sync::{Arc, Mutex};
use stdweb::traits::*;
use stdweb::web::{
    document,
    IParentNode,
};
use stdweb::web::event::{
    DataTransfer,
    DragOverEvent,
    DragStartEvent,
    DragDropEvent,
    EffectAllowed,
    DropEffect,
};
use stdweb::unstable::TryInto;

pub fn remove_str(vec: &mut Vec<String>, item: &str) -> Option<String> {
    let pos = vec.iter().position(|x| *x == *item)?;
    Some(vec.remove(pos))
}

fn show_latest_drop(dt: DataTransfer) {
    dt.items().index(0).unwrap().get_as_string(|s| {
        let latest_drop_elem =
            document()
                .query_selector(".latest-drop")
                .unwrap()
                .unwrap();
        js! {
            @{latest_drop_elem.as_ref()}.innerHTML = @{s} + " just moved to the other team!";
        };
    });
}

fn add_event_listeners<F>(team: &'static str, change_team: F)
    where F : Fn(&str) + 'static {

    let chars_elem = document().query_selector(&format!(".{}-chars", team)).unwrap().unwrap();
    chars_elem.add_event_listener(|e: DragStartEvent| {
        let target = e.target().unwrap();
        let char_name: String = js!(return @{target.as_ref()}.textContent).try_into().unwrap();
        e.data_transfer().unwrap().set_effect_allowed(EffectAllowed::CopyMove);
        e.data_transfer().unwrap().set_data("text/plain", char_name.as_ref());
    });

    let dropzone_elem = document().query_selector(&format!(".{}-dropzone", team)).unwrap().unwrap();
    dropzone_elem.add_event_listener(|e: DragOverEvent| {
        e.prevent_default();
        e.data_transfer().unwrap().set_drop_effect(DropEffect::Move);
    });

    dropzone_elem.add_event_listener(move |e: DragDropEvent| {
        e.prevent_default();
        let content = e.data_transfer().unwrap().get_data("text/plain");
        show_latest_drop(e.data_transfer().unwrap());
        change_team(&content);
    });
}

fn render(team_a: &Vec<String>, team_b: &Vec<String>) {
    let inner_html = |vec: &Vec<String>|
        vec
            .iter()
            .map(|x| format!("<div class=\"char\" draggable=\"true\">{}</div>", x))
            .collect::<Vec<String>>()
            .join("\n");
    ;

    let team_a_elem = document().query_selector(".team-a-chars").unwrap().unwrap();
    let team_b_elem = document().query_selector(".team-b-chars").unwrap().unwrap();
    js!(@{team_a_elem.as_ref()}.innerHTML = @{inner_html(team_a)});
    js!(@{team_b_elem.as_ref()}.innerHTML = @{inner_html(team_b)});
}

fn drag_and_drop_elements_example() {
    let team_a_arc = Arc::new(Mutex::new(vec![
        String::from("Mario"),
        String::from("Fox"),
    ]));

    let team_b_arc = Arc::new(Mutex::new(vec![
        String::from("Marth"),
        String::from("Captain Falcon"),
    ]));


    let change_team = |name: &str,
                       team_a: &mut Vec<String>,
                       team_b: &mut Vec<String>,
                       to_a: bool| {
        remove_str(team_a, name);
        remove_str(team_b, name);
        if to_a {
            team_a.push(String::from(name));
        } else {
            team_b.push(String::from(name));
        }
        render(team_a, team_b);
    };

    let team_a = team_a_arc.clone();
    let team_b = team_b_arc.clone();
    add_event_listeners("team-a", move |name: &str| {
        let mut team_a = team_a.lock().unwrap();
        let mut team_b = team_b.lock().unwrap();
        change_team(name, &mut *team_a, &mut *team_b, true);
    });

    let team_a = team_a_arc.clone();
    let team_b = team_b_arc.clone();
    add_event_listeners("team-b", move |name: &str| {
        let mut team_a = team_a.lock().unwrap();
        let mut team_b = team_b.lock().unwrap();
        change_team(name, &mut *team_a, &mut *team_b, false);
    });

    let team_a = team_a_arc.clone();
    let team_b = team_b_arc.clone();
    render(&*team_a.lock().unwrap(), &*team_b.lock().unwrap());
}

fn drop_filesystem_example() {
    let dropzone = || document().query_selector("#filesystem-dropzone").unwrap().unwrap();
    dropzone().add_event_listener(move |e: DragOverEvent| {
        e.prevent_default();
        js!(@{e.as_ref()}.currentTarget.style.backgroundColor = "lightblue");
        e.data_transfer().unwrap().set_drop_effect(DropEffect::Move);
    });
    dropzone().add_event_listener(move |e: DragDropEvent| {
        e.prevent_default();
        js!(@{e.as_ref()}.currentTarget.style.backgroundColor = "transparent");
        for x in e.data_transfer().unwrap().files() {
            let div = document().create_element("div").unwrap();
            js!(@{div.as_ref()}.innerHTML = "- " + @{x.name()});
            dropzone().append_child(&div)
        }
    });
}

fn main() {
    stdweb::initialize();

    drag_and_drop_elements_example();

    drop_filesystem_example();

    stdweb::event_loop();
}

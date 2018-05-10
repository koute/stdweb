#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    HtmlElement,
    Element,
    document,
    window
};

use stdweb::web::event::{
    DoubleClickEvent,
    ClickEvent,
    KeyPressEvent,
    ChangeEvent,
    BlurEvent,
    HashChangeEvent
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

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    title: String,
    completed: bool
}

#[derive(Serialize, Deserialize)]
struct State {
    todo_list: Vec< Todo >
}

impl State {
    fn new() -> Self {
        State {
            todo_list: Vec::new()
        }
    }
}

type StateRef = Rc< RefCell< State > >;

fn start_editing( state: &StateRef, index: usize, li: &HtmlElement, label: &HtmlElement ) {
    li.class_list().add( "editing" ).unwrap();

    let edit: InputElement = document().create_element( "input" ).unwrap().try_into().unwrap();
    edit.class_list().add( "edit" ).unwrap();
    edit.set_raw_value( &label.inner_text() );
    edit.add_event_listener( enclose!( (edit) move |event: KeyPressEvent| {
        if event.key() == "Enter" {
            edit.blur();
        }
    }));

    edit.add_event_listener( enclose!( (state, li, edit) move |_: BlurEvent| {
        li.class_list().remove( "editing" ).unwrap();
        li.remove_child( &edit ).unwrap();
        state.borrow_mut().todo_list[ index ].title = edit.raw_value();
        update_dom( &state );
    }));

    li.append_child( &edit );
    edit.focus();
}

fn create_entry( state: &StateRef, index: usize, text: &str ) -> HtmlElement {
    let li: HtmlElement = document().create_element( "li" ).unwrap().try_into().unwrap();
    let div = document().create_element( "div" ).unwrap();
    let checkbox: InputElement = document().create_element( "input" ).unwrap().try_into().unwrap();
    let label: HtmlElement = document().create_element( "label" ).unwrap().try_into().unwrap();
    let button = document().create_element( "button" ).unwrap();

    div.class_list().add( "view" ).unwrap();

    checkbox.class_list().add( "toggle" ).unwrap();
    js! { @{&checkbox}.type = "checkbox"; }
    checkbox.add_event_listener( enclose!( (state, checkbox) move |_: ChangeEvent| {
        let checked: bool = js!( return @{&checkbox}.checked; ).try_into().unwrap();
        state.borrow_mut().todo_list[ index ].completed = checked;
        update_dom( &state );
    }));

    label.append_child( &document().create_text_node( text ) );
    label.add_event_listener( enclose!( (state, li, label) move |_: DoubleClickEvent| {
        start_editing( &state, index, &li, &label );
    }));

    button.class_list().add( "destroy" ).unwrap();
    button.add_event_listener( enclose!( (state) move |_: ClickEvent| {
        state.borrow_mut().todo_list.remove( index );
        update_dom( &state );
    }));

    li.append_child( &div );
    div.append_child( &checkbox );
    div.append_child( &label );
    div.append_child( &button );

    li
}

fn update_dom( state: &StateRef ) {
    // Ideally you'd use some kind of DOM diffing here;
    // since it's supposed to be a simple example we opt
    // for the nuclear option and just rebuild everything
    // from scratch.

    fn only_active( todo: &Todo ) -> bool { todo.completed == false }
    fn only_completed( todo: &Todo ) -> bool { todo.completed == true }
    fn all( _: &Todo ) -> bool { true }

    // See which filter we're supposed to use based on the URL.
    let hash = document().location().unwrap().hash().unwrap();
    let filter = match hash.as_str() {
        "#/active" => only_active,
        "#/completed" => only_completed,
        _ => all
    };

    let filter_anchor_selector = match hash.as_str() {
        "#/active" | "#/completed" => hash.as_str(),
        _ => "#/"
    };

    // Select the filter "button".
    let filter_anchors = document().query_selector_all( ".filters a" ).unwrap();
    for anchor in &filter_anchors {
        let anchor: Element = anchor.try_into().unwrap();
        anchor.class_list().remove( "selected" ).unwrap();
    }

    let filter_anchor_selector = format!( ".filters a[href='{}']", filter_anchor_selector );
    let selected_anchor: Element = document().query_selector( filter_anchor_selector.as_str() ).unwrap().unwrap();
    selected_anchor.class_list().add( "selected" ).unwrap();

    // Clear previous entries in the list.
    let list = document().query_selector( ".todo-list" ).unwrap().unwrap();
    while let Some( child ) = list.first_child() {
        list.remove_child( &child ).unwrap();
    }

    // Fill out the list.
    let state_borrow = state.borrow();
    for (index, todo) in state_borrow.todo_list.iter().enumerate().filter( |&(_, todo)| filter( todo ) ) {
        let entry_node = create_entry( state, index, todo.title.as_str() );
        if todo.completed {
            entry_node.class_list().add( "completed" ).unwrap();
            let checkbox = entry_node.query_selector( "input[type='checkbox']" ).unwrap();
            js!( @{checkbox}.checked = true; );
        }
        list.append_child( &entry_node );
    }

    // Display the amount of active TODOs lefs.
    let items_left = state_borrow.todo_list.iter().filter( |todo| {
        todo.completed == false
    }).count();

    let counter_display = document().query_selector( ".todo-count" ).unwrap().unwrap();
    if items_left == 1 {
        counter_display.set_text_content( "1 item left" );
    } else {
        counter_display.set_text_content( format!( "{} items left", items_left ).as_str() );
    }

    // Hide the list if we don't have any TODOs.
    let main = document().query_selector( ".main" ).unwrap();
    if state_borrow.todo_list.is_empty() {
        js!( @{main}.style = "display: none;" );
    } else {
        js!( @{main}.style = "display: block;" );
    }

    // Save the state into local storage.
    let state_json = serde_json::to_string( &*state_borrow ).unwrap();
    window().local_storage().insert( "state", state_json.as_str() ).unwrap();
}

fn main() {
    stdweb::initialize();

    let state = window().local_storage().get( "state" ).and_then( |state_json| {
        serde_json::from_str( state_json.as_str() ).ok()
    }).unwrap_or_else( State::new );
    let state = Rc::new( RefCell::new( state ) );

    let title_entry: InputElement = document().query_selector( ".new-todo" ).unwrap().unwrap().try_into().unwrap();
    title_entry.add_event_listener( enclose!( (state, title_entry) move |event: KeyPressEvent| {
        if event.key() == "Enter" {
            event.prevent_default();

            let title: String = title_entry.raw_value();
            if title.is_empty() == false {
                state.borrow_mut().todo_list.push( Todo {
                    title: title,
                    completed: false
                });

                title_entry.set_raw_value( "" );
                update_dom( &state );
            }
        }
    }));

    let clear_completed = document().query_selector( ".clear-completed" ).unwrap().unwrap();
    clear_completed.add_event_listener( enclose!( (state) move |_: ClickEvent| {
        state.borrow_mut().todo_list.retain( |todo| todo.completed == false );
        update_dom( &state );
    }));

    window().add_event_listener( enclose!( (state) move |_: HashChangeEvent| {
        update_dom( &state );
    }));

    update_dom( &state );
    stdweb::event_loop();
}

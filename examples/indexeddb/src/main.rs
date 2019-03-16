#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;

extern crate serde;

use std::collections::HashMap;
use std::cell::RefCell;

use stdweb::Value;

use stdweb::traits::*;
use stdweb::web::{
    Element,
    window,
    document,
    IEventTarget,
};

use stdweb::web::html_element::InputElement;

use stdweb::web::event::IEvent;

use stdweb::web::event::{
    IDBSuccessEvent,
    IDBVersionChangeEvent,
    IDBCompleteEvent,
    IDBErrorEvent,
    SubmitEvent,
    ClickEvent
};

use stdweb::web::indexeddb::{
    IDBOpenDBRequest,
    IDBDatabase,
    IDBRequest,
    IDBRequestSharedMethods,
    IDBObjectStoreIndexSharedMethods,
    IDBCursorWithValue,
    IDBCursorSharedMethods,
    IDBTransactionMode
};

use stdweb::unstable::TryInto;

#[derive(Serialize, Deserialize)]
struct Note {
    //id: u32,
    title: String,
    body: String
}

js_serializable!( Note );
js_deserializable!( Note );

thread_local!(static DB: RefCell<Option<IDBDatabase>> = RefCell::new(None));

fn display_data_inner(db: &IDBDatabase) {
    let list = document().query_selector("ul").unwrap().unwrap();
    // Here we empty the contents of the list element each time the display is updated
    // If you ddn't do this, you'd get duplicates listed each time a new note is added
    while list.first_child().is_some() {
       list.remove_child(&list.first_child().unwrap()).unwrap();
    }
    // Open our object store and then get a cursor - which iterates through all the
    // different data items in the store
    let object_store = db.transaction(vec!["notes"], IDBTransactionMode::ReadOnly).object_store("notes").unwrap();
    object_store.open_cursor(None, None).unwrap()
        .add_event_listener( move |e: IDBSuccessEvent| {
            // Get a reference to the cursor
            let db_request: IDBRequest = e.target().unwrap().try_into().unwrap();
            //let cursor: IDBCursorWithValue = db_request.result().try_into().unwrap();
            let maybe_cursor: Result<IDBCursorWithValue, stdweb::private::ConversionError> = db_request.result().unwrap().try_into();
            
            // If there is still another data item to iterate through, keep running this code
            if let Ok(cursor) = maybe_cursor {
                // Create a list item, h3, and p to put each data item inside when displaying it
                // structure the HTML fragment, and append it inside the list
                let list_item = document().create_element("li").unwrap();
                let h3 = document().create_element("h3").unwrap();
                let para = document().create_element("p").unwrap();
                
                list_item.append_child(&h3);
                list_item.append_child(&para);
                list.append_child(&list_item);

                let note: Note = cursor.value().try_into().unwrap();
                
                // Put the data from the cursor inside the h3 and para
                h3.set_text_content(&note.title);
                para.set_text_content(&note.body);
                
                // Store the ID of the data item inside an attribute on the list_item, so we know
                // which item it corresponds to. This will be useful later when we want to delete items
                let id: u32 = cursor.key().try_into().unwrap();
                list_item.set_attribute("data-note-id", &format!("{}", id)).unwrap();
                // Create a button and place it inside each list_item
                let delete_btn = document().create_element("button").unwrap();
                list_item.append_child(&delete_btn);
                delete_btn.set_text_content("Delete");
                
                // Set an event handler so that when the button is clicked, the deleteItem()
                // function is run
                delete_btn.add_event_listener( delete_item );

                // Iterate to the next item in the cursor
                cursor.advance(1).unwrap(); // Todo this was continue
                
            } else {
                // Again, if list item is empty, display a 'No notes stored' message
                if list.first_child().is_none() {
                    let list_item = document().create_element("li").unwrap();
                    list_item.set_text_content("No notes stored.");
                    list.append_child(&list_item);
                }
                // if there are no more cursor items to iterate through, say so
                console!(log, "Notes all displayed");
            }
        });}

fn display_data() {
        DB.with(|db_cell| {
            if let Some(ref db) = *db_cell.borrow_mut()  {
                display_data_inner(db);
            }})
}

// Define the deleteItem() function
fn delete_item( e: ClickEvent ) {
    // retrieve the name of the task we want to delete. We need
    // to convert it to a number before trying it use it with IDB; IDB key
    // values are type-sensitive.
    let button: Element = e.target().unwrap().try_into().unwrap();
    let note: Element = button.parent_node().unwrap().try_into().unwrap();
    let note_id = note.get_attribute("data-note-id").unwrap().parse::<u32>().unwrap();
    
    // open a database transaction and delete the task, finding it using the id we retrieved above
    DB.with(|db_cell| {
        if let Some(ref db) = *db_cell.borrow_mut()  {
            let transaction = db.transaction(vec!["notes"], IDBTransactionMode::ReadWrite);
            let object_store = transaction.object_store("notes").unwrap();
            object_store.delete(note_id.try_into().unwrap()).unwrap();
            
            // report that the data item has been deleted
            transaction.add_event_listener( move |_e: IDBCompleteEvent| {
                // delete the parent of the button
                // which is the list item, so it is no longer displayed
                //let node: Node = e.target().unwrap().try_into().unwrap();
                note.parent_node().unwrap().remove_child(&note).unwrap();
                console!(log, "Note ", note_id,  "deleted.");

                // Again, if list item is empty, display a 'No notes stored' message
                let list = document().query_selector("ul").unwrap().unwrap();
                if ! list.first_child().is_some() {
                    let list_item = document().create_element("li").unwrap();
                    list_item.set_text_content("No notes stored.");
                    list.append_child(&list_item);
                }
            });
        }});
}

fn main() {
    stdweb::initialize();
    
    // Open our database; it is created if it doesn't already exist
    // (see onupgradeneeded below)
    let request = window().indexed_db().open("notes", 1);

    // onerror handler signifies that the database didn't open successfully
    request.add_event_listener( | _e: IDBErrorEvent| {
        js!(
            console.log("Database failed to open");
        );
    });

    // onsuccess handler signifies that the database opened successfully
    request.add_event_listener( move |event: IDBSuccessEvent| {
        js!(
            console.log("Database opened succesfully");
        );

        let db_request: IDBOpenDBRequest = event.target().unwrap().try_into().unwrap();
        // Store the opened database object in the db variable. This is used a lot below
        let db : IDBDatabase = db_request.database_result().unwrap();

        DB.with(|db_cell| {
            db_cell.replace(Some(db));
        });
        // Run the displayData() function to display the notes already in the IDB
        display_data();
    });
    
    request.add_event_listener( |event: IDBVersionChangeEvent| {
    	let db_request: IDBOpenDBRequest = event.target().unwrap().try_into().unwrap();
        let db_: IDBDatabase = db_request.result().unwrap().try_into().unwrap();

        // Create an object_store to store our notes in (basically like a single table)
        let object_store = db_.create_object_store("notes", true, "").unwrap();
        
        // Define what data items the object_store will contain
        let mut title_options = HashMap::new();
        title_options.insert("unique", false);
        object_store.create_index("title", "title", Value::from(title_options));
        
        let mut body_options = HashMap::new();
        body_options.insert("unique", false);
        object_store.create_index("body", "body", Value::from(body_options));

        js!(
            console.log("Database setup complete");
        );

    });

    let form = document().query_selector("form").unwrap().unwrap();
    form.add_event_listener( move |e: SubmitEvent | {
        // prevent default - we don't want the form to submit in the conventional way
        e.prevent_default();
        
        // grab the values entered into the form fields and store them in an object ready for being inserted into the DB
        let title_input: InputElement = document().query_selector("#title").unwrap().unwrap().try_into().unwrap();
        let body_input: InputElement = document().query_selector("#body").unwrap().unwrap().try_into().unwrap();
        let new_item = Note{ title: title_input.raw_value(), body: body_input.raw_value() };

        DB.with(|db_cell| {
            if let Some(ref db) = *db_cell.borrow_mut() {
                // open a read/write db transaction, ready for adding the data
                let transaction = db.transaction(vec!["notes"], IDBTransactionMode::ReadWrite);
        
                // call an object store that's already been added to the database
                let object_store = transaction.object_store("notes").unwrap();
        
                // Make a request to add our new_item object to the object store
                let request = object_store.add(new_item.try_into().unwrap(), None).unwrap();
                
                request.add_event_listener( move |_e: IDBSuccessEvent| {
                    // Clear the form, ready for adding the next entry
                    title_input.set_raw_value("");
                    body_input.set_raw_value("");
                });
        
                // Report on the success of the transaction completing, when everything is done
                transaction.add_event_listener( |_e: IDBCompleteEvent| {
                    console!(log, "Transaction completed: database modification finished.");
                    
                    // update the display of data to show the newly added item, by running displayData() again.
                    display_data();
                });
        
                transaction.add_event_listener( |_e: IDBErrorEvent| {
                    console!(log, "Transaction not opened due to error");
                });
            }});        
    });
}

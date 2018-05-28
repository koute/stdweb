#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;

extern crate serde;

use std::collections::HashMap;
use std::cell::RefCell;
use std::str::FromStr;

use stdweb::Value;

use stdweb::traits::*;
use stdweb::web::{
    HtmlElement,
    Element,
    window,
    document,
    IEventTarget,
    EventTarget,
    Node
};

use stdweb::web::html_element::InputElement;

use stdweb::web::event::{ConcreteEvent, IEvent};

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
    DBRequest,
    IDBCursor,
    IDBObjectStore,
    IDBObjectStoreIndexSharedMethods,
    IDBCursorWithValue,
    IDBCursorSharedMethods
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
    console!(log, "a");
    let list = document().query_selector("ul").unwrap().unwrap();
    console!(log, "b");
    // Here we empty the contents of the list element each time the display is updated
    // If you ddn't do this, you'd get duplicates listed each time a new note is added
    while list.first_child().is_some() {
        list.remove_child(&list.first_child().unwrap());
    }
    console!(log, "c");
    // Open our object store and then get a cursor - which iterates through all the
    // different data items in the store
    let object_store = db.transaction("notes", "readonly").object_store("notes");
    console!(log, "5");
    object_store.open_cursor(None, None)
        .add_event_listener( move |e: IDBSuccessEvent| {
            console!(log, "6");
            // Get a reference to the cursor
            let db_request: DBRequest = e.target().unwrap().try_into().unwrap();
            console!(log, "7");
            //let cursor: IDBCursorWithValue = db_request.result().try_into().unwrap();
            let maybe_cursor: Result<IDBCursorWithValue, stdweb::private::ConversionError> = db_request.result().try_into();
                        
            // If there is still another data item to iterate through, keep running this code
            if let Ok(cursor) = maybe_cursor {
                console!(log, "8");    
                // Create a list item, h3, and p to put each data item inside when displaying it
                // structure the HTML fragment, and append it inside the list
                let listItem = document().create_element("li").unwrap();
                let h3 = document().create_element("h3").unwrap();
                let para = document().create_element("p").unwrap();
                
                listItem.append_child(&h3);
                listItem.append_child(&para);
                list.append_child(&listItem);

                let note: Note = cursor.value().try_into().unwrap();
                
                // Put the data from the cursor inside the h3 and para
                h3.set_text_content(&note.title);
                para.set_text_content(&note.body);
                
                // Store the ID of the data item inside an attribute on the listItem, so we know
                // which item it corresponds to. This will be useful later when we want to delete items
                console!(log, "9");
                let id: u32 = cursor.key().try_into().unwrap();
                console!(log, "10");
                listItem.set_attribute("data-note-id", &format!("{}", id));
                console!(log, "11");
                // Create a button and place it inside each listItem
                let deleteBtn = document().create_element("button").unwrap();
                listItem.append_child(&deleteBtn);
                deleteBtn.set_text_content("Delete");
                
                // Set an event handler so that when the button is clicked, the deleteItem()
                // function is run
                deleteBtn.add_event_listener( delete_item );

                // Iterate to the next item in the cursor
                cursor.advance(1); // Todo this was continue
                
            } else {
                // Again, if list item is empty, display a 'No notes stored' message
                if(list.first_child().is_none()) {
                    let listItem = document().create_element("li").unwrap();
                    listItem.set_text_content("No notes stored.");
                    list.append_child(&listItem);
                }
                // if there are no more cursor items to iterate through, say so
                console!(log, "Notes all displayed");
            }
        });}

fn display_data() {
        DB.with(|db_cell| {
            if let Some(ref db) = *db_cell.borrow_mut()  {
                console!(log, "3");
                display_data_inner(db);
                console!(log, "4");
            }})
}

// Define the deleteItem() function
fn delete_item( e: ClickEvent ) {
    // retrieve the name of the task we want to delete. We need
    // to convert it to a number before trying it use it with IDB; IDB key
    // values are type-sensitive.
    let button: Element = e.target().unwrap().try_into().unwrap();
    let note: Element = button.parent_node().unwrap().try_into().unwrap();
    let noteId = note.get_attribute("data-note-id").unwrap().parse::<u32>().unwrap();
    
    // open a database transaction and delete the task, finding it using the id we retrieved above
    DB.with(|db_cell| {
        if let Some(ref db) = *db_cell.borrow_mut()  {
            let transaction = db.transaction("notes", "readwrite");
            let objectStore = transaction.object_store("notes");
            let request = objectStore.delete(noteId.try_into().unwrap());
            
            // report that the data item has been deleted
            console!(log, 20);
            js!{
                @{transaction.as_ref()}.oncomplete = function(e) {
                    console.log(e);
                };

            };
            transaction.add_event_listener( move |e: IDBCompleteEvent| {
                console!(log, 21);
                // delete the parent of the button
                // which is the list item, so it is no longer displayed
                //let node: Node = e.target().unwrap().try_into().unwrap();
                note.parent_node().unwrap().remove_child(&note);
                console!(log, "Note ", noteId,  "deleted.");

                // Again, if list item is empty, display a 'No notes stored' message
                let list = document().query_selector("ul").unwrap().unwrap();
                if(!list.first_child().is_some()) {
                    let listItem = document().create_element("li").unwrap();
                    listItem.set_text_content("No notes stored.");
                    list.append_child(&listItem);
                }
            });
        }});
}

fn main() {
    stdweb::initialize();
    
    let submit_btn = document().query_selector("form button");

 
    
    // Open our database; it is created if it doesn't already exist
    // (see onupgradeneeded below)
    let request = window().indexed_db().open("notes", 1);

    // onerror handler signifies that the database didn't open successfully
    request.add_event_listener( |event: IDBErrorEvent| {
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
        let db : IDBDatabase = db_request.result().try_into().unwrap();

        DB.with(|db_cell| {
            db_cell.replace(Some(db));
        });
        // Run the displayData() function to display the notes already in the IDB
        console!(log, "1");
        display_data();
        console!(log, "2");
    });
    
    request.add_event_listener( |event: IDBVersionChangeEvent| {
    	let db_request: IDBOpenDBRequest = event.target().unwrap().try_into().unwrap();
        let db_: IDBDatabase = db_request.result().try_into().unwrap();

        // Create an objectStore to store our notes in (basically like a single table)
        // including a auto-incrementing key
        let mut store_options = HashMap::new();
        //store_options.insert("keyPath", "id");
        store_options.insert("autoIncrement", "true");
        let object_store = db_.create_object_store("notes", Value::from(store_options));
        
        // Define what data items the objectStore will contain
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
        let newItem = Note{ title: title_input.raw_value(), body: body_input.raw_value() };

        DB.with(|db_cell| {
            if let Some(ref db) = *db_cell.borrow_mut() {
                // open a read/write db transaction, ready for adding the data
                let transaction = db.transaction("notes", "readwrite");
        
                // call an object store that's already been added to the database
                let objectStore = transaction.object_store("notes");
        
                // Make a request to add our newItem object to the object store
                let request = objectStore.add(newItem.try_into().unwrap(), None);
                
                request.add_event_listener( move |e: IDBSuccessEvent| {
                    // Clear the form, ready for adding the next entry
                    title_input.set_raw_value("");
                    body_input.set_raw_value("");
                });
        
                // Report on the success of the transaction completing, when everything is done
                transaction.add_event_listener( |e: IDBCompleteEvent| {
                    console!(log, "Transaction completed: database modification finished.");
                    
                    // update the display of data to show the newly added item, by running displayData() again.
                    display_data();
                });
        
                transaction.add_event_listener( |e: IDBErrorEvent| {
                    console!(log, "Transaction not opened due to error");
                });



            }});









        
    });
    
}

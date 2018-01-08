#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;

extern crate serde;

use std::collections::HashMap;

use stdweb::Value;

use stdweb::web::{
    window,
    IEventTarget,
    EventTarget
};

use stdweb::web::event::{ConcreteEvent, IEvent};

use stdweb::web::event::{
    IDBSuccessEvent,
    IDBVersionChangeEvent,
    IDBCompleteEvent
};

use stdweb::web::indexeddb::{
    IDBOpenDBRequest,
    IDBDatabase,
    IDBRequest,
    DBRequest
};

use stdweb::unstable::TryInto;

#[derive(Serialize, Deserialize)]
struct Customer {
    ssn: String,
    name: String,
    age: i32,
    email: String
}

js_serializable!( Customer );
js_deserializable!( Customer );

fn main() {
    stdweb::initialize();

    let request = window().indexed_db().open("db", None);

    request.add_event_listener( |event: IDBVersionChangeEvent| {
        let db_request: IDBOpenDBRequest = event.target().unwrap().try_into().unwrap();
        let db: IDBDatabase = db_request.result().try_into().unwrap();

        let mut store_options = HashMap::new();
        store_options.insert("keyPath", "ssn");
        let object_store = db.create_object_store("customers", Value::from(store_options));
        
        let mut name_options = HashMap::new();
        name_options.insert("unique", false);
        object_store.create_index("name", "name", Value::from(name_options));

        let mut email_options = HashMap::new();
        email_options.insert("unique", true);
        object_store.create_index("email", "email", Value::from(email_options));

        object_store.transaction().add_event_listener( move |event: IDBCompleteEvent| {
            
            let customers = vec![
                Customer{ ssn: "444-44-4444".to_string(), name: "Bill".to_string(), age: 35, email: "bill@company.com".to_string() },
                Customer{ ssn: "555-55-5555".to_string(), name: "Donna".to_string(), age: 32, email: "donna@home.org".to_string() }
            ];

            let customer_object_store = db.transaction("customers", "readwrite").object_store("customers");

            for customer in &customers {
                customer_object_store.add(customer.try_into().unwrap(), None);
            }
        });
        
    });
    
    stdweb::event_loop();
}

use webcore::value::Value;
use webcore::value::Reference;
use webcore::try_from::{TryFrom, TryInto};
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::dom_exception::{DomException, InvalidStateError, TypeError, TransactionInactiveError};

/// Used to represent the state of an IDBRequest.
///
/// [(JavaScript docx)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState)
#[derive(Debug)]
pub enum IDBRequestReadyState {
    /// The request is pending.
    Pending,
    /// The request is done.
    Done
}

/// Represents the different types the source arrtibute of an IDBRequest
/// can take.
#[derive(Debug)]
pub enum IDBRequestSource {
    /// Indicates no source exists, such as when calling `indexedDB.open`
    None,
    Store(IDBObjectStore),
    Index(IDBIndex),
    Cursor(IDBCursor)
}

/// IDBRequestSharedMethode represents the methode that are shared between
/// IDBOpenDBRequest and IDBRequest.
pub trait IDBRequestSharedMethods : IEventTarget {

    /// The result read-only property of the `IDBRequest` interface returns the result of the request,
    /// or if the request failed InvalidStateError.
    ///
    /// [(JavaScript docx)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/result)
    fn result( &self ) -> Result<Value, InvalidStateError> {
        js_try!( return @{self.as_ref()}.result; ).unwrap()
    }

    /// Returns the error in the event of an unsuccessful request.
    ///
    /// [(JavaScript docx)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/error)
    fn error(&self) ->  Option<DomException> {
        js!( @{self.as_ref()}.error;).try_into().unwrap()
    }
    
    /// Returns the source of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source)
    fn source( &self ) -> IDBRequestSource {
        let t: i32 = js!{
            if (@{self.as_ref()}.source instanceof IDBObjectStore) {
                return 0;
            } else  if (@{self.as_ref()}.source instanceof IDBIndex) {
                return 1;
            } else if (@{self.as_ref()}.source instanceof IDBCursor) {
                return 2;
            } else {
                return 3;
            }
        }.try_into().unwrap();
        match t {
            0 => IDBRequestSource::Store(js!(return @{self.as_ref()}.source;).try_into().unwrap()),
            1 => IDBRequestSource::Index(js!(return @{self.as_ref()}.source;).try_into().unwrap()),
            2 => IDBRequestSource::Cursor(js!(return @{self.as_ref()}.source;).try_into().unwrap()),
            3 => IDBRequestSource::None,
            _ => panic!()
        }
    }
    
    /// The `transaction` read-only property of the `IDBRequest` interface
    /// returns the transaction for the request, that is, the transaction
    /// the request is being made inside.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/transaction)
    fn transaction( &self ) -> Option<IDBTransaction> {
        let transaction : Value = js! (
            return @{self.as_ref()}.transaction;
        );
        match transaction {
            Value::Undefined => None,
            Value::Null => None,
            _ => Some(transaction.try_into().unwrap())
        }
    }
    
    /// The `ready_state` read-only property of the `IDBRequest` interface
    /// returns the state of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState)
    fn ready_state( &self ) -> IDBRequestReadyState {
        let ready_state : String = js! (
            return @{self.as_ref()}.readyState;
        ).try_into().unwrap();
        
        if ready_state.eq("pending") {
            return IDBRequestReadyState::Pending;
        } else if ready_state.eq("done") {
            return IDBRequestReadyState::Done;
        } else {
            panic!("Got {} as an IDBRequestReadyState.", ready_state);
        }        
    }
    
}

/// The `IDBReques`t interface of the IndexedDB API provides access to results
/// of asynchronous requests to databases and database objects using event
/// handlers. Events that are received are IDBSuccessEvent and IDBErrorEvent.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBRequest")]
#[reference(subclass_of(EventTarget))]
pub struct IDBRequest( Reference );

impl IEventTarget for IDBRequest {}
impl IDBRequestSharedMethods for IDBRequest {}

/// Provides access to the results of requests to open or delete databases.
/// Receives `IDBBlockedEvent` and `IDBVersionChangeEvent` as well as events received by `IDBRequest`.
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBOpenDBRequest")]
#[reference(subclass_of(EventTarget))]
pub struct IDBOpenDBRequest( Reference );

impl IEventTarget for IDBOpenDBRequest {}
impl IDBRequestSharedMethods for IDBOpenDBRequest {}

impl IDBOpenDBRequest {

    /// Returns the value property as an `IDBDatabase`, or an `InvalidStateError`.
    pub fn database_result(&self) -> Result<IDBDatabase, InvalidStateError> {
        match self.result() {
            Ok(value) => Ok(value.try_into().unwrap()),
            Err(error) => Err(error)
        }
    }
}

/// The `IDBFactory` interface of the IndexedDB API lets applications asynchronously access the indexed databases. The object that implements the interface is `window.indexedDB`. 
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBFactory")]
pub struct IDBFactory( Reference );

impl IDBFactory {

    /// Requests opening a connection to a database.
    ///
    /// version can be None.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)
    pub fn open<T: Into<Option<u32>>>( &self, name: &str, version: T) -> IDBOpenDBRequest {
        match version.into() {
            None => js! (
                return @{self.as_ref()}.open(@{name});
            ).try_into().unwrap(),
            Some(version) => js! (
                return @{self.as_ref()}.open(@{name}, @{version});
            ).try_into().unwrap()
        }
    }

    

    /// Requests the deletion of a database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)
    pub fn delete_database( &self, name: &str) -> IDBOpenDBRequest {
        js! (
            return @{self.as_ref()}.deleteDatabase(@{name});
        ).try_into().unwrap()
    }

    /// Compares two values as keys to determine equality and ordering for `IndexedDB` operations.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/cmp)
    pub fn cmp( &self, first: Value, second: Value) -> i16 {
        js!(
            return @{self.as_ref()}.cmp(@{first.as_ref()}, @{second.as_ref()});
        ).try_into().unwrap()
    }
    
}

/// The IDBCursorDirection enum indicates the direction in which a cursor is traversing the data.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)
#[derive(Debug)]
pub enum IDBCursorDirection {
    /// This direction causes the cursor to be opened at the start of the source.
    Next,
    /// This direction causes the cursor to be opened at the start of the source. For every key with duplicate values, only the first record is yielded.
    NextUnique,
    /// This direction causes the cursor to be opened at the end of the source.
    Prev,
    /// This direction causes the cursor to be opened at the end of the source. For every key with duplicate values, only the first record is yielded.
    PrevUnique
}

fn cursor_direction_to_string( direction: IDBCursorDirection) -> String {
    match direction {
        IDBCursorDirection::Next => "next".to_string(),
        IDBCursorDirection::NextUnique => "nextunique".to_string(),
        IDBCursorDirection::Prev => "prev".to_string(),
        IDBCursorDirection::PrevUnique => "prevunique".to_string()
    }
}

fn string_to_cursor_direction( direction: &str) -> IDBCursorDirection {
    if direction.eq("next") {
        return IDBCursorDirection::Next;
    } else if direction.eq("nextunique") {
        return IDBCursorDirection::NextUnique;
    } else if direction.eq("prev") {
        return IDBCursorDirection::Prev;
    } else if direction.eq("prevunique") {
        return IDBCursorDirection::PrevUnique;
    } else {
        unreachable!("Unknown index direction \"{}\".", direction);
    }
}

/// This enum is used to represent the vlaue of the soure property of
/// a `IDBCursor`.
#[derive(Debug)]
pub enum IDBCursorSource {
    Store(IDBObjectStore),
    Index(IDBIndex)
}

error_enum_boilerplate! {
    /// An enum of the exceptions that IDBCursorSharedMethods.advance() may throw
    AdvanceError,
    /// This IDBCursor's transaction is inactive.
    TransactionInactiveError,
    /// The value passed into the count parameter was zero or a negative number.
    TypeError,
    /// The cursor is currently being iterated or has iterated past its end.
    InvalidStateError
}

/// This trait implements all the methods that are shared between
/// `IDBCursor` and `IDBCursorWithValue`.
pub trait IDBCursorSharedMethods: AsRef< Reference >  {
    
    /// The source read-only property of the `IDBCursor` interface returns
    /// the `IDBObjectStore` or `IDBIndex` that the cursor is iterating over.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/source)
    fn source( &self ) -> IDBCursorSource {
        if js!( return @{self.as_ref()}.source instanceof IDBObjectStore; ).try_into().unwrap() {
            IDBCursorSource::Store(js!( return @{self.as_ref()}.source ).try_into().unwrap())
        } else if js!( return @{self.as_ref()}.source instanceof IDBIndex;).try_into().unwrap() {
            IDBCursorSource::Index(js!( return @{self.as_ref()}.source ).try_into().unwrap())
        } else {
            panic!()
        }
    }
    
    /// The `direction` read-only property of the `IDBCursor` interface is
    /// an enum that represents the direction of traversal of the
    /// cursor (set using `IDBObjectStore.openCursor` for example).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)
    fn direction( &self ) -> IDBCursorDirection {
        let direction: String = js! ( return @{self.as_ref()}.direction; ).try_into().unwrap();
        return string_to_cursor_direction(&direction);
    }

    /// The `key` read-only property of the `IDBCursor` interface returns the key
    /// for the record at the cursor's position. If the cursor is outside its range,
    /// this is set to undefined. The cursor's key can be any data type.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/key)
    fn key( &self ) -> Value {
        js!(
            return @{self.as_ref()}.key; )
            .try_into().unwrap()
    }
    
    /// The `primary_key` read-only property of the `IDBCursor` interface returns
    /// the cursor's current effective key. If the cursor is currently being
    /// iterated or has iterated outside its range, this is set to undefined.
    ///The cursor's primary key can be any data type.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/primaryKey)
    fn primary_key( &self ) -> Value {
        js!(
            return @{self.as_ref()}.primaryKey; )
            .try_into().unwrap()
    }

    /// 
    ///
    ///
    fn advance( &self, count: u32) -> Result<(), AdvanceError> {
        js_try!( @{self.as_ref()}.advance(@{count}); ).unwrap()
    }
    
    
    //void continue(optional any key);
    ///
    ///
    ///
    fn advance_to_match<K: Into<Option<Value>>>( &self, key: K) {
        match key.into() {
            None => js! { @{self.as_ref()}.continue(); },
            Some(key) => js! { @{self.as_ref()}.continue(@{key.as_ref()}); }
        };
    }
    
    //void continuePrimaryKey(any key, any primaryKey);

    //[NewObject] IDBRequest update(any value);
    ///
    ///
    ///
    fn update( &self, value: Value) -> IDBRequest {
        js! ( return @{self.as_ref()}.update(@{value.as_ref()}); ).try_into().unwrap()
    }

    //[NewObject] IDBRequest delete();
    ///
    ///
    ///
    fn delete( &self ) -> IDBRequest {
        js!( return @{self.as_ref()}.delete(); ).try_into().unwrap() 
    }
}

///
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBCursor")]
pub struct IDBCursor( Reference );

impl IDBCursorSharedMethods for IDBCursor {}

///
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBCursorWithValue")]
pub struct IDBCursorWithValue( Reference );

impl IDBCursorSharedMethods for IDBCursorWithValue {}

impl IDBCursorWithValue {

    ///
    pub fn value( &self ) -> Value {
        js! (
            return @{self}.value
            ).try_into().unwrap()
    }
}

/// This trait contains mothods that are Identicle in both IDBIndex IDBObjectStore
pub trait IDBObjectStoreIndexSharedMethods: AsRef< Reference > {

    // attribute DOMString name;
    /// Returns the name of this object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    fn name( &self ) -> String {
        js! (
            return @{self.as_ref()}.name;
        ).try_into().unwrap()
    }

    /// Returns the name of this object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    fn set_name( &self, name: &str) {
        js! {
            @{self.as_ref()}.name = @{name};
        };
    }

    // [NewObject] IDBRequest get(any query);
    /// This is for retrieving specific records from an object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)
    fn get( &self, query: Value) -> IDBRequest {
        js! (
            return @{self.as_ref()}.get(@{query.as_ref()});
        ).try_into().unwrap()
    }
    
    // [NewObject] IDBRequest getKey(any query);
    /// This is for retrieving specific records from an object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)
    fn get_key( &self, query: Value) -> IDBRequest {
        js! (
            return @{self.as_ref()}.getKey(@{query.as_ref()});
        ).try_into().unwrap()
    }
    
    // [NewObject] IDBRequest getAll(optional any query,                                optional [EnforceRange] unsigned long count);
    /// 
    ///
    ///
    fn get_all<Q: Into<Option<Value>>, C: Into<Option<u32>>>( &self, query: Q, count: C) -> IDBRequest {
        match query.into() {
            None => js! ( return @{self.as_ref()}.getAll(); ),
            Some(query) => {
                match count.into() {
                    None => js! ( return @{self.as_ref()}.getAll(@{query.as_ref()}); ),
                    Some(count) => js! ( return @{self.as_ref()}.getAll(@{query.as_ref()}, @{count}); )
                }
            }
        }.try_into().unwrap()
    }
    
    
    // [NewObject] IDBRequest getAllKeys(optional any query,                                    optional [EnforceRange] unsigned long count);
    ///
    ///
    ///
    fn get_all_keys<Q: Into<Option<Value>>, C: Into<Option<u32>>>( &self, query: Q, count: C) -> IDBRequest {
        match query.into() {
            None => js! ( return @{self.as_ref()}.getAllKeys(); ),
            Some(query) => {
                match count.into() {
                    None => js! ( return @{self.as_ref()}.getAllKeys(@{query.as_ref()}); ),
                    Some(count) => js! ( return @{self.as_ref()}.getAllKeys(@{query.as_ref()}, @{count}); )
                }
            }
        }.try_into().unwrap()
    }
    
    // [NewObject] IDBRequest count(optional any query);
    ///
    ///
    ///
    fn count<Q: Into<Option<Value>>>( &self, query: Q) -> IDBRequest {
        match query.into() {
            None => js! (
                return @{self.as_ref()}.count();
            ),
            Some(query) => js! (
                return @{self.as_ref()}.count(@{query.as_ref()});
            )
        }.try_into().unwrap()
    }

    //    [NewObject] IDBRequest openCursor(optional any query,                                    optional IDBCursorDirection direction = "next");
    ///
    ///
    ///
    fn open_cursor<Q: Into<Option<Value>>, D: Into<Option<IDBCursorDirection>>>( &self, query: Q, direction: D) -> IDBRequest {
        match query.into() {
            None => js! ( return @{self.as_ref()}.openCursor(); ),
            Some(query) => {
                match direction.into() {
                    None => js! ( return @{self.as_ref()}.openCursor(@{query.as_ref()}); ),
                    Some(direction) => js! ( return @{self.as_ref()}.openCursor(@{query.as_ref()}, @{cursor_direction_to_string(direction)}); )
                }
            }
        }.try_into().unwrap()
    }
    
    // [NewObject] IDBRequest openKeyCursor(optional any query,                                       optional IDBCursorDirection direction = "next");
    ///
    ///
    ///
    fn open_key_cursor<Q: Into<Option<Value>>, D: Into<Option<IDBCursorDirection>>>( &self, query: Q, direction: D) -> IDBRequest {
        match query.into() {
            None => js! ( return @{self.as_ref()}.openKeyCursor(); ),
            Some(query) => {
                match direction.into() {
                    None => js! ( return @{self.as_ref()}.openKeyCursor(@{query.as_ref()}); ),
                    Some(direction) => js! ( return @{self.as_ref()}.openKeyCursor(@{query.as_ref()}, @{cursor_direction_to_string(direction)}); )
                }
            }
        }.try_into().unwrap()
    }

}

/// Provides asynchronous access to an index in a database.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBIndex")]
pub struct IDBIndex( Reference );

impl IDBObjectStoreIndexSharedMethods for IDBIndex {}

impl IDBIndex {
    //attribute DOMString name;
    // Implemented in trait.
    
    //[SameObject] readonly attribute IDBObjectStore objectStore;
    //readonly attribute any keyPath;
    
    //readonly attribute boolean multiEntry;
    /// Affects how the index behaves when the result of evaluating the index's key path yields an array. If `true`, there is one record in the index for each item in an array of keys. If `false`, then there is one record for each key that is an array.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/multiEntry)
    pub fn multi_entry( &self ) -> bool {
        js! (
            return @{self.as_ref()}.multiEntry;
        ).try_into().unwrap()
    }
    
    //readonly attribute boolean unique;
    /// If `true`, this index does not allow duplicate values for a key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/unique)
    pub fn unique( &self ) -> bool {
        js! (
            return @{self.as_ref()}.unique;
        ).try_into().unwrap()
    }

    // The rest of this is implemented in the trait
    //[NewObject] IDBRequest get(any query);
    //[NewObject] IDBRequest getKey(any query);
    //[NewObject] IDBRequest getAll(optional any query, optional [EnforceRange] unsigned long count);
    //[NewObject] IDBRequest getAllKeys(optional any query, optional [EnforceRange] unsigned long count);
    //[NewObject] IDBRequest count(optional any query);
    //[NewObject] IDBRequest openCursor(optional any query, optional IDBCursorDirection direction = "next");
    //[NewObject] IDBRequest openKeyCursor(optional any query, optional IDBCursorDirection direction = "next");
}

/// The `IDBObjectStore` interface of the IndexedDB API represents an object store in a database
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBObjectStore")]
pub struct IDBObjectStore( Reference );

impl IDBObjectStoreIndexSharedMethods for IDBObjectStore {}

impl IDBObjectStore {
   
    
    // readonly attribute any keyPath;
    // Todo, how am I wrapping this.
    
    // readonly attribute DOMStringList indexNames;
    // TODO: how am I wrapping this
    
    // [SameObject] readonly attribute IDBTransaction transaction;
    /// The `IDBTransaction` object to which this object store belongs.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)
    pub fn transaction( &self ) -> IDBTransaction {
        js! (
            return @{self.as_ref()}.transaction;
        ).try_into().unwrap()
    }
    
    // readonly attribute boolean autoIncrement;
    /// Returns the value of the auto increment flag for this object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)
    fn auto_increment( &self ) -> bool {
        js! (
            return @{self.as_ref()}.autoIncrement;
        ).try_into().unwrap()
    }
    
    // [NewObject] IDBRequest put(any value, optional any key);
    /// Updates a given record in a database, or inserts a new record if the given item does not already exist.
    /// The key is only needed if 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)
    pub fn put<T: Into<Option<Value>>>( &self, value: Value, key: T) -> IDBRequest {
        match key.into() {
            None => js! (
                return @{self.as_ref()}.put(@{value.as_ref()});
            ),
            Some(key) => js! (
                return @{self.as_ref()}.put(@{value.as_ref()}, @{key.as_ref()});
            )
        }.try_into().unwrap()
    }
    
    // [NewObject] IDBRequest add(any value, optional any key);
    /// Returns an `IDBRequest` object, and, in a separate thread, creates a structured clone of the value, and stores the cloned value in the object store. This is for adding new records to an object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)
    pub fn add<T: Into<Option<Value>>>( &self, value: Value, key: T) -> IDBRequest {
        match key.into() {
            None => js! (
                return @{self.as_ref()}.add(@{value.as_ref()});
            ),
            Some(key) => js! (
                return @{self.as_ref()}.add(@{value.as_ref()}, @{key.as_ref()});
            )
        }.try_into().unwrap()
    }

    // [NewObject] IDBRequest delete(any query);
    /// returns an `IDBRequest` object, and, in a separate thread, deletes the specified record or records.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)
    pub fn delete( &self, query: Value) -> IDBRequest {
        js! (
            return @{self.as_ref()}.delete(@{query.as_ref()});
        ).try_into().unwrap()
    }
    
    // [NewObject] IDBRequest clear();
    /// Returns an IDBRequest object, and clears this object store in a separate thread
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)
    pub fn clear( &self ) -> IDBRequest {
        js! (
            return @{self.as_ref()}.clear();
        ).try_into().unwrap()
    }
    
    // IDBIndex index(DOMString name);
    /// opens a named index in the current object store
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)
    pub fn index( &self, name: &str) -> IDBIndex {
        js! (
            return @{self.as_ref()}.index(@{name});
        ).try_into().unwrap()
    }

    // [NewObject] IDBIndex createIndex(DOMString name, (DOMString or sequence<DOMString>) keyPath, optional IDBIndexParameters options);
    /// Creates and returns a new `IDBIndex` object in the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
    pub fn create_index( &self, name: &str, key_path: &str, options: Value) -> IDBIndex { // TODO, how am I doing the optinal options?
        js! (
            return @{self.as_ref()}.createIndex(@{name}, @{key_path}, @{options.as_ref()});
        ).try_into().unwrap()
    }
    
    //  void deleteIndex(DOMString name);
    /// Destroys the index with the specified name in the connected database, used during a version upgrade.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)
    fn delete_index( &self, name: &str) {
        js! {
            return @{self.as_ref()}.deleteIndex(@{name});
        }
    }
}

/* dictionary IDBIndexParameters {
  boolean unique = false;
  boolean multiEntry = false;
};*/


pub enum IDBTransactionMode {
  ReadOnly,
  Readwrite,
  VersionChange
}

/// The `IDBTransaction` interface of the IndexedDB API provides a static, asynchronous transaction on a database using event handlers.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBTransaction")]
pub struct IDBTransaction( Reference );

impl IEventTarget for IDBTransaction {}

impl IDBTransaction {
    // readonly attribute DOMStringList objectStoreNames;
    // Todo, how am I wrapping DOMStringList
    
    // readonly attribute IDBTransactionMode mode;
    // Todo, should I use an enum or a string
    
    // [SameObject] readonly attribute IDBDatabase db;
    /// Returns the database connection with which this transaction is associated.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db)
    pub fn db( &self ) -> IDBDatabase {
        js! (
            return @{self}.db();
        ).try_into().unwrap()
    }
    
    // readonly attribute DOMException error;
    
    // IDBObjectStore objectStore(DOMString name);
    /// This is a method
    pub fn object_store( &self, name: &str) -> IDBObjectStore {
        js! (
            return @{self.as_ref()}.objectStore(@{name});
        ).try_into().unwrap()
    }
    
    // void abort();
    // Todo, do I need to implement this or do I get it for free from IEventTarget
    // ///
    // ///
    // /// [(JavaScript docs)]
    
    // Event handlers:
    // attribute EventHandler onabort;
    // attribute EventHandler oncomplete;
    // attribute EventHandler onerror;
}

/// The `IDBDatabase` interface of the IndexedDB API provides a connection to a database.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBDatabase")]
pub struct IDBDatabase( Reference );

impl IEventTarget for IDBDatabase {}

impl IDBDatabase {
    
    // readonly attribute DOMString name;
    /// Returns the the name of the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)
    pub fn name( &self ) -> String {
        js! (
            return @{self.as_ref()}.name;
        ).try_into().unwrap()
    }
    
    // readonly attribute unsigned long long version;
    /// Returns the version of the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)
    pub fn version( &self ) -> u64 {
        js! (
            return @{self.as_ref()}.version;
            ).try_into().unwrap()
    }
    
    // readonly attribute DOMStringList objectStoreNames;
    // TODO: how should I expose DomStringList

    // [NewObject] IDBTransaction transaction((DOMString or sequence<DOMString>) storeNames, optional IDBTransactionMode mode = "readonly");
    /// Immediately returns a transaction object (`IDBTransaction`) containing the `IDBTransaction.object_store` method, which you can use to access your object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    pub fn transaction( &self, store_name: &str, mode: &str) -> IDBTransaction {
        js! (
            //return @{self.as_ref()}.transaction(@{store_name}, @{mode});
            return @{self.as_ref()}.transaction(@{store_name}, @{mode});
        ).try_into().unwrap()
    }
    
    //void close();
    /// Returns immediately and closes the connection in a separate thread.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)
    pub fn close( &self ) {
        js! {
            @{self.as_ref()}.close();
        }
    }

    // [NewObject] IDBObjectStore createObjectStore(DOMString name, optional IDBObjectStoreParameters options);
    /// Creates and returns a new object store or index. TODO: why does this say index
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)
    pub fn create_object_store( &self, name: &str, options: Value) -> IDBObjectStore {
        js! (
            return @{self.as_ref()}.createObjectStore(@{name}, @{options.as_ref()});
        ).try_into().unwrap()
    }
    
    // void deleteObjectStore(DOMString name);
    /// Destroys the object store with the given name.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)
    pub fn delete_object_store( &self, name: &str ) {
        js! {
            @{self.as_ref()}.deleteObjectStore(@{name});
        }
    }

    // Event handlers:
    // attribute EventHandler onabort;
    // attribute EventHandler onclose;
    // attribute EventHandler onerror;
    // attribute EventHandler onversionchange;
    
}

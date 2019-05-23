use webcore::value::{Value, Reference};
use webcore::try_from::{TryFrom, TryInto};
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::dom_exception::{DomException, InvalidStateError, TransactionInactiveError, DataError, InvalidAccessError, ReadOnlyError, DataCloneError, ConstraintError, NotFoundError};
use webapi::error::TypeError;
use webapi::dom_string_list::DOMStringList;

/// Used to represent the state of an DbRequest.
///
/// [(JavaScript docx)](https://developer.mozilla.org/en-US/docs/Web/API/DbRequest/readyState)
#[derive(Debug)]
pub enum DbRequestReadyState {
    /// The request is pending.
    Pending,
    /// The request is done.
    Done
}

/// Represents the different types the source arrtibute of an DbRequest
/// can take.
#[derive(Debug)]
pub enum DbRequestSource {
    /// Indicates no source exists, such as when calling `indexedDB.open`
    Store(DbObjectStore),
    Index(DbIndex),
    Cursor(DbCursor)
}

/// DbRequestSharedMethode represents the methode that are shared between
/// DbOpenDBRequest and DbRequest.
pub trait DbRequestSharedMethods : IEventTarget {

    /// The result read-only property of the `DbRequest` interface returns the result of the request,
    /// or if the request failed InvalidStateError.
    ///
    /// [(JavaScript docx)](https://developer.mozilla.org/en-US/docs/Web/API/DbRequest/result)
    fn result( &self ) -> Result<Value, InvalidStateError> {
        js_try!( return @{self.as_ref()}.result; ).unwrap()
    }

    /// Returns the error in the event of an unsuccessful request.
    ///
    /// [(JavaScript docx)](https://developer.mozilla.org/en-US/docs/Web/API/DbRequest/error)
    fn error(&self) ->  Option<DomException> {
        js!( @{self.as_ref()}.error;).try_into().unwrap()
    }
    
    /// Returns the source of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbRequest/source)
    fn source( &self ) -> DbRequestSource {
        let t: i32 = js!{
            if (@{self.as_ref()}.source instanceof DbObjectStore) {
                return 0;
            } else  if (@{self.as_ref()}.source instanceof DbIndex) {
                return 1;
            } else if (@{self.as_ref()}.source instanceof DbCursor) {
                return 2;
            } else {
                panic!()
            }
        }.try_into().unwrap();
        match t {
            0 => DbRequestSource::Store(js!(return @{self.as_ref()}.source;).try_into().unwrap()),
            1 => DbRequestSource::Index(js!(return @{self.as_ref()}.source;).try_into().unwrap()),
            2 => DbRequestSource::Cursor(js!(return @{self.as_ref()}.source;).try_into().unwrap()),
            _ => panic!()
        }
    }
    
    /// The `transaction` read-only property of the `DbRequest` interface
    /// returns the transaction for the request, that is, the transaction
    /// the request is being made inside.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbRequest/transaction)
    fn transaction( &self ) -> Option<DbTransaction> {
        let transaction : Value = js! (
            return @{self.as_ref()}.transaction;
        );
        match transaction {
            Value::Undefined => None,
            Value::Null => None,
            _ => Some(transaction.try_into().unwrap())
        }
    }
    
    /// The `ready_state` read-only property of the `DbRequest` interface
    /// returns the state of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbRequest/readyState)
    fn ready_state( &self ) -> DbRequestReadyState {
        let ready_state : String = js! (
            return @{self.as_ref()}.readyState;
        ).try_into().unwrap();
        
        if ready_state.eq("pending") {
            return DbRequestReadyState::Pending;
        } else if ready_state.eq("done") {
            return DbRequestReadyState::Done;
        } else {
            panic!("Got {} as an DbRequestReadyState.", ready_state);
        }        
    }
    
}

/// The `DbReques`t interface of the IndexedDB API provides access to results
/// of asynchronous requests to databases and database objects using event
/// handlers. Events that are received are DbSuccessEvent and DbErrorEvent.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbRequest)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBRequest")]
#[reference(subclass_of(EventTarget))]
pub struct DbRequest( Reference );

impl IEventTarget for DbRequest {}
impl DbRequestSharedMethods for DbRequest {}

/// Provides access to the results of requests to open or delete databases.
/// Receives `DbBlockedEvent` and `DbVersionChangeEvent` as well as events received by `DbRequest`.
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbOpenDBRequest)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBOpenDBRequest")]
#[reference(subclass_of(EventTarget))]
pub struct DbOpenDBRequest( Reference );

impl IEventTarget for DbOpenDBRequest {}
impl DbRequestSharedMethods for DbOpenDBRequest {}

impl DbOpenDBRequest {

    /// Returns the value property as an `DbDatabase`, or an `InvalidStateError`.
    pub fn database_result(&self) -> Result<DbDatabase, InvalidStateError> {
        match self.result() {
            Ok(value) => Ok(value.try_into().unwrap()),
            Err(error) => Err(error)
        }
    }
}

/// The `DbFactory` interface of the IndexedDB API lets applications asynchronously access the indexed databases. The object that implements the interface is `window.indexedDB`. 
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbFactory)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBFactory")]
pub struct DbFactory( Reference );

impl DbFactory {

    /// Requests opening a connection to a database.
    ///
    /// version can be None.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbFactory/open)
    pub fn open<T: Into<Option<u32>>>( &self, name: &str, version: T) -> DbOpenDBRequest {
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
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbFactory/deleteDatabase)
    pub fn delete_database( &self, name: &str) -> DbOpenDBRequest {
        js! (
            return @{self.as_ref()}.deleteDatabase(@{name});
        ).try_into().unwrap()
    }

    /// Compares two values as keys to determine equality and ordering for `IndexedDB` operations.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbFactory/cmp)
    pub fn cmp( &self, first: Value, second: Value) -> i16 {
        js!(
            return @{self.as_ref()}.cmp(@{first.as_ref()}, @{second.as_ref()});
        ).try_into().unwrap()
    }
    
}

/// The DbCursorDirection enum indicates the direction in which a cursor is traversing the data.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/direction)
#[derive(Debug)]
pub enum DbCursorDirection {
    /// This direction causes the cursor to be opened at the start of the source.
    Next,
    /// This direction causes the cursor to be opened at the start of the source. For every key with duplicate values, only the first record is yielded.
    NextUnique,
    /// This direction causes the cursor to be opened at the end of the source.
    Prev,
    /// This direction causes the cursor to be opened at the end of the source. For every key with duplicate values, only the first record is yielded.
    PrevUnique
}

fn cursor_direction_to_string( direction: DbCursorDirection) -> String {
    match direction {
        DbCursorDirection::Next => "next".to_string(),
        DbCursorDirection::NextUnique => "nextunique".to_string(),
        DbCursorDirection::Prev => "prev".to_string(),
        DbCursorDirection::PrevUnique => "prevunique".to_string()
    }
}

fn string_to_cursor_direction( direction: &str) -> DbCursorDirection {
    if direction.eq("next") {
        return DbCursorDirection::Next;
    } else if direction.eq("nextunique") {
        return DbCursorDirection::NextUnique;
    } else if direction.eq("prev") {
        return DbCursorDirection::Prev;
    } else if direction.eq("prevunique") {
        return DbCursorDirection::PrevUnique;
    } else {
        unreachable!("Unknown index direction \"{}\".", direction);
    }
}

/// This enum is used to represent the vlaue of the soure property of
/// a `DbCursor`.
#[derive(Debug)]
pub enum DbCursorSource {
    Store(DbObjectStore),
    Index(DbIndex)
}

error_enum_boilerplate! {
    /// An enum of the exceptions that DbCursorSharedMethods.advance()
    /// and DbCursorSharedMethods.next may throw.
    DbAdvanceError,
    /// This DbCursor's transaction is inactive.
    TransactionInactiveError,
    /// The value passed into the parameter was zero or a negative number.
    TypeError,
    /// The cursor is currently being iterated or has iterated past its end.
    InvalidStateError
}

error_enum_boilerplate! {
    DbContinuePrimaryKeyError,
    /// This DbCursor's transaction is inactive.
    TransactionInactiveError,
    /// The key parameter may have any of the following conditions:
    /// * The key is not a valid key.
    /// * The key is less than or equal to this cursor's position and the cursor's direction is next or nextunique.
    /// * The key is greater than or equal to this cursor's position and this cursor's direction is prev or prevunique.
    DataError,
    ///	The cursor is currently being iterated or has iterated past its end.
    InvalidStateError,
    ///	The cursor's direction is not prev or next.
    InvalidAccessError
}

error_enum_boilerplate! {
    DbUpdateError,
    /// This DbCursor's transaction is inactive.
    TransactionInactiveError,
    /// The transaction mode is read only.
    ReadOnlyError,
    /// The cursor was created using DbIndex.openKeyCursor, is currently being iterated, or has iterated past its end.
    InvalidStateError,
    /// The underlying object store uses in-line keys and the property in the value at the object store's key path does not match the key in this cursor's position.
    DataError,
    ///The data being stored could not be cloned by the internal structured cloning algorithm.
    DataCloneError
}

///
error_enum_boilerplate! {
    ///
    DbAddError,
    /// This DbCursor's transaction is inactive.
    TransactionInactiveError,
    /// The transaction mode is read only.
    ReadOnlyError,
    /// The cursor was created using DbIndex.openKeyCursor, is currently being iterated, or has iterated past its end.
    InvalidStateError,
    /// The underlying object store uses in-line keys and the property in the value at the object store's key path does not match the key in this cursor's position.
    DataError,
    ///The data being stored could not be cloned by the internal structured cloning algorithm.
    DataCloneError,
    /// An operation failed because the primary key constraint was violated (due to an already existing record with the same primary key value).
    ConstraintError
}

error_enum_boilerplate! {
    DbCursorDeleteError,
    /// This DbCursor's transaction is inactive.
    TransactionInactiveError,
    /// The transaction mode is read-only.
    ReadOnlyError,
    /// The cursor was created using Dbindex.openKeyCursor, is currently being iterated, or has iterated past its end.
    InvalidStateError
}

error_enum_boilerplate! {
    DbClearError,
    /// The transaction associated with this operation is in read-only mode.
    ReadOnlyError,
    /// This DbObjectStore's transaction is inactive.
    TransactionInactiveError
}

/// This trait implements all the methods that are shared between
/// `DbCursor` and `DbCursorWithValue`.
pub trait DbCursorSharedMethods: AsRef< Reference >  {
    
    /// The source read-only property of the `DbCursor` interface returns
    /// the `DbObjectStore` or `DbIndex` that the cursor is iterating over.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/source)
    fn source( &self ) -> DbCursorSource {
        if js!( return @{self.as_ref()}.source instanceof DbObjectStore; ).try_into().unwrap() {
            DbCursorSource::Store(js!( return @{self.as_ref()}.source ).try_into().unwrap())
        } else if js!( return @{self.as_ref()}.source instanceof DbIndex;).try_into().unwrap() {
            DbCursorSource::Index(js!( return @{self.as_ref()}.source ).try_into().unwrap())
        } else {
            panic!()
        }
    }
    
    /// The `direction` read-only property of the `DbCursor` interface is
    /// an enum that represents the direction of traversal of the
    /// cursor (set using `DbObjectStore.openCursor` for example).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/direction)
    fn direction( &self ) -> DbCursorDirection {
        let direction: String = js! ( return @{self.as_ref()}.direction; ).try_into().unwrap();
        return string_to_cursor_direction(&direction);
    }

    /// The `key` read-only property of the `DbCursor` interface returns the key
    /// for the record at the cursor's position. If the cursor is outside its range,
    /// this is set to undefined. The cursor's key can be any data type.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/key)
    fn key( &self ) -> Value {
        js!(
            return @{self.as_ref()}.key; )
            .try_into().unwrap()
    }
    
    /// The `primary_key` read-only property of the `DbCursor` interface returns
    /// the cursor's current effective key. If the cursor is currently being
    /// iterated or has iterated outside its range, this is set to undefined.
    ///The cursor's primary key can be any data type.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/primaryKey)
    fn primary_key( &self ) -> Value {
        js!(
            return @{self.as_ref()}.primaryKey; )
            .try_into().unwrap()
    }

    /// The advance() method of the DbCursor interface sets the number of times
    /// a cursor should move its position forward. 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/advance) 
    fn advance( &self, count: u32) -> Result<(), DbAdvanceError> {
        js_try!( @{self.as_ref()}.advance(@{count}); ).unwrap()
    }
        
    /// The next() method of the DbCursor interface advances the cursor to the
    /// next position along its direction, to the item whose key matches the optional
    /// key parameter. If no key (None) is specified, the cursor advances to the immediate
    /// next position, based on its direction.
    ///
    /// This function stands in for continue in the javascript interface. Continue
    /// is a keyword in rust and so needed to be renamed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/continue)
    fn next<K: Into<Option<Value>>>( &self, key: K) -> Result<(), DbAdvanceError> {
        match key.into() {
            None => js_try!( @{self.as_ref()}.continue(); ).unwrap(),
            Some(key) => js_try! ( @{self.as_ref()}.continue(@{key.as_ref()}); ).unwrap()
        }
    }

    /// The continuePrimaryKey() method of the DbCursor interface advances
    /// the cursor to the to the item whose key matches the key parameter as
    /// well as whose primary key matches the primary key parameter.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/continuePrimaryKey)
    fn continue_primary_key( &self, key: Value, primary_key: Value) -> Result<(), DbContinuePrimaryKeyError> {
        js_try!( @{self.as_ref()}.continuePrimaryKey(@{key}, @{primary_key}); ).unwrap()
    }

    /// The update() method of the DbCursor interface returns an DbRequest
    /// object, and, in a separate thread, updates the value at the current
    /// position of the cursor in the object store. If the cursor points to
    /// a record that has just been deleted, a new record is created.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/update)
    fn update( &self, value: Value) -> Result<DbRequest, DbUpdateError> {
        js_try!( return @{self.as_ref()}.update(@{value.as_ref()}); ).unwrap()
    }

    /// The delete() method of the DbCursor interface returns an DbRequest
    /// object, and, in a separate thread, deletes the record at the cursor's
    /// position, without changing the cursor's position. Once the record is
    /// deleted, the cursor's value is set to null.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor/delete)
    fn delete( &self ) -> Result<DbRequest, DbCursorDeleteError> {
        js_try!( return @{self.as_ref()}.delete(); ).unwrap() 
    }
}

/// The DbCursor interface of the IndexedDB API represents a cursor for
/// traversing or iterating over multiple records in a database.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursor)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBCursor")]
pub struct DbCursor( Reference );

impl DbCursorSharedMethods for DbCursor {}

/// The DbCursorWithValue interface of the IndexedDB API represents a cursor
/// for traversing or iterating over multiple records in a database. It is
/// the same as the DbCursor, except that it includes the value property.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursorWithValue)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBCursorWithValue")]
pub struct DbCursorWithValue( Reference );

impl DbCursorSharedMethods for DbCursorWithValue {}

impl DbCursorWithValue {

    /// Returns the value of the current cursor.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbCursorWithValue/value)
    pub fn value( &self ) -> Value {
        js! (
            return @{self}.value
            ).try_into().unwrap()
    }
}

/// The DbKeyRange interface of the IndexedDB API represents a continuous interval
/// over some data type that is used for keys. Records can be retrieved from
/// DbObjectStore and DbIndex objects using keys or a range of keys. You can limit
/// the range using lower and upper bounds. For example, you can iterate over all
/// values of a key in the value range A–Z.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBKeyRange")]
pub struct DbKeyRange( Reference );

impl DbKeyRange {

    // Static construction methods:
    
    /// The only() method of the DbKeyRange interface creates a new key range
    /// containing a single value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/only)
    pub fn only( value: Value ) -> Result<DbKeyRange, DataError> {
        js_try! ( return DbKeyRange.only(@{value}); ).unwrap()
    }
    
    /// The lower_bound() method of the DbKeyRange interface creates a new key range
    /// with only a lower bound. if open is false it includes the lower endpoint
    /// value and is closed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/lowerBound)
    pub fn lower_bound( lower: Value, open: bool ) -> Result<DbKeyRange, DataError> {
        js_try! ( return DbKeyRange.lowerBound(@{lower}, @{open}); ).unwrap()
    }
    
    /// The upper_bound() method of the DbKeyRange interface creates a new key range
    /// with only an apper bound. if open is false it includes the upper endpoint
    /// value and is closed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/upperBound)
    pub fn upper_bound( upper: Value, open: bool ) -> Result<DbKeyRange, DataError> {
        js_try! ( return DbKeyRange.upperBound(@{upper}, @{open}); ).unwrap()
    }
    
    /// The bound() method of the DbKeyRange interface creates a new key range
    /// with the specified upper and lower bounds. The bounds can be open (that
    /// is, the bounds exclude the endpoint values) or closed (that is, the bounds
    /// include the endpoint values).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/bound)
    pub fn bound (lower: Value, upper: Value, lower_open: bool, upper_open: bool) -> Result<DbKeyRange, DataError> {
        js_try! (
            return DbKeyRange.boundound(@{lower}, @{upper}, @{lower_open}, @{upper_open});
        ).unwrap()
    }
        
    /// Lower bound of the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/lower)
    pub fn lower( &self ) -> Value {
        js!( return @{self}.lower; ).try_into().unwrap()
    }

    /// Upper bound of the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/upper)
    pub fn upper( &self ) -> Value {
        js!( return @{self}.upper; ).try_into().unwrap()
    }

    /// Returns false if the lower-bound value is included in the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/lowerOpen)
    pub fn lower_open( &self ) -> bool {
        js!( return @{self}.lowerOpen; ).try_into().unwrap()
    }

    /// Returns false if the upper-bound value is included in the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/upperOpen)
    pub fn upper_open( &self ) -> bool {
        js!( return @{self}.upperOpen; ).try_into().unwrap()
    }

    /// The includes() method of the DbKeyRange interface returns a boolean
    /// indicating whether a specified key is inside the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbKeyRange/includes)
    pub fn includes( &self, value: Value ) -> Result<bool, DataError> {
        js_try! ( return @{self}.includes(@{value}); ).unwrap()
    }
}

#[derive(Debug)]
pub enum DbKeyOrKeyRange {
    None,
    Value(Value),
    Range(DbKeyRange)
}

error_enum_boilerplate! {
    DbSetNameError,

    /// The index, or its object store, has been deleted; or the current transaction
    /// is not an upgrade transaction. You can only rename indexes during upgrade
    /// transactions; that is, when the mode is "versionchange".
    InvalidStateError,

    /// The current transaction is not active.
    TransactionInactiveError,
        
    /// An index is already using the specified name
    ConstraintError
}

error_enum_boilerplate! {
    /// This Error is raised by various methods ised to query object stores
    /// and indexes.
    DbQueryError,
    
    ///  This DbIndex's transaction is inactive.
    TransactionInactiveError,
    
    /// The key or key range provided contains an invalid key.
    DataError,
    
    ///  The DbIndex has been deleted or removed.
    InvalidStateError
}

error_enum_boilerplate! {
    DbIndexError,
    /// The source object store has been deleted, or the transaction for the object store has finished.
    InvalidStateError,
    /// There is no index with the given name (case-sensitive) in the database.
    NotFoundError
         
}

/// This trait contains mothods that are Identicle in both DbIndex DbObjectStore
pub trait DbObjectStoreIndexSharedMethods: AsRef< Reference > {

    /// Returns the name of this index or object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/name)
    fn name( &self ) -> String {
        js! (
            return @{self.as_ref()}.name;
        ).try_into().unwrap()
    }

    /// Returns the name of this index or object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/name)
    fn set_name( &self, name: &str) -> Result<(), DbSetNameError> {
        js_try! ( @{self.as_ref()}.name = @{name}; ).unwrap()
    }

    /// The key_path read-only property of the DbObjectStore interface returns the
    /// key path of this object store. Or in the case of an DbIndex, the current
    /// object store.
    fn key_path( &self ) -> Value {
        js!( return @{self.as_ref()}.keyPath; ).try_into().unwrap()
    }

    /// This is for retrieving specific records from an object store or index.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/get)
    fn get<Q: Into<DbKeyOrKeyRange>>( &self, query: Q) -> Result<DbRequest, DbQueryError> {
        match query.into() {
            DbKeyOrKeyRange::None => js_try! (
                return @{self.as_ref()}.get();
            ),
            DbKeyOrKeyRange::Value(value) => js_try! (
                return @{self.as_ref()}.get(@{value.as_ref()});
            ),
            DbKeyOrKeyRange::Range(range) => js_try! (
                return @{self.as_ref()}.get(@{range.as_ref()});
            )
        }.unwrap()
    }

    /// Returns an DbRequest object, and, in a separate thread retrieves and
    /// returns the record key for the object matching the specified parameter.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/getKey)
    fn get_key<Q: Into<DbKeyOrKeyRange>>( &self, query: Q) -> Result<DbRequest, DbQueryError> {
        match query.into() {
            DbKeyOrKeyRange::None => js_try! (
                return @{self.as_ref()}.getKey();
            ),
            DbKeyOrKeyRange::Value(value) => js_try! (
                return @{self.as_ref()}.getKey(@{value.as_ref()});
            ),
            DbKeyOrKeyRange::Range(range) => js_try! (
                return @{self.as_ref()}.getKey(@{range.as_ref()});
            )
        }.unwrap()
    }
    
    /// The get_ll() method retrieves all objects that are inside the index or
    /// object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex/getAll)
    fn get_all<Q: Into<DbKeyOrKeyRange>, C: Into<Option<u32>>>( &self, query: Q, count: C) -> Result<DbRequest, DbQueryError> {
        match query.into() {
            DbKeyOrKeyRange::None => js_try! ( return @{self.as_ref()}.getAll(); ),
            DbKeyOrKeyRange::Value(value) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAll(@{value.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAll(@{value.as_ref()}, @{count}); )
                }
            },
            DbKeyOrKeyRange::Range(range) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAll(@{range.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAll(@{range.as_ref()}, @{count}); )
                }
            }
        }.unwrap()
    }
    
    // Acording to the mozilla documentation the DbIndex version does not
    // Throw DataError.
    /// The get_all_keys() method returns an DbRequest object retrieves record keys
    /// for all objects matching the specified parameter or all objects if no
    /// parameters are given.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/getAllKeys)
    fn get_all_keys<Q: Into<DbKeyOrKeyRange>, C: Into<Option<u32>>>( &self, query: Q, count: C) -> Result<DbRequest, DbQueryError> {
        match query.into() {
            DbKeyOrKeyRange::None => js_try! ( return @{self.as_ref()}.getAllKeys(); ),
            DbKeyOrKeyRange::Value(value) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAllKeys(@{value.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAllKeys(@{value.as_ref()}, @{count}); )
                }
            },
            DbKeyOrKeyRange::Range(range) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAllKeys(@{range.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAllKeys(@{range.as_ref()}, @{count}); )
                }
            }
        }.unwrap()
    }
    
    /// Returns an DbRequest object, and, in a separate thread, returns the total number of records that match the provided key or DbKeyRange
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex/count)
    fn count<Q: Into<DbKeyOrKeyRange>>( &self, query: Q) -> Result<DbRequest, DbQueryError> {
        match query.into() {
            DbKeyOrKeyRange::None => js_try! (
                return @{self.as_ref()}.count();
            ),
            DbKeyOrKeyRange::Value(value) => js_try! (
                return @{self.as_ref()}.count(@{value.as_ref()});
            ),
            DbKeyOrKeyRange::Range(range) => js_try! (
                return @{self.as_ref()}.count(@{range.as_ref()});
            )
        }.unwrap()
    }

    /// The open_cursor() method returns an DbRequest object, and, in a separate
    /// thread, creates a cursor over the specified key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex/openCursor)
    fn open_cursor<Q: Into<Option<DbKeyRange>>, D: Into<Option<DbCursorDirection>>>( &self, range: Q, direction: D) -> Result<DbRequest, DbQueryError> {
        match range.into() {
            None => js_try! ( return @{self.as_ref()}.openCursor(); ),
            Some(range) => {
                match direction.into() {
                    None => js_try! ( return @{self.as_ref()}.openCursor(@{range.as_ref()}); ),
                    Some(direction) => js_try! ( return @{self.as_ref()}.openCursor(@{range.as_ref()}, @{cursor_direction_to_string(direction)}); )
                }
            }
        }.unwrap()
    }
    
    /// The open_key_cursor() method returns an DbRequest object, and, in a
    /// separate thread, creates a cursor over the specified key range, as arranged
    /// by this index.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex/openKeyCursor)
    fn open_key_cursor<Q: Into<Option<DbKeyRange>>, D: Into<Option<DbCursorDirection>>>( &self, range: Q, direction: D) -> Result<DbRequest, DbQueryError> {
        match range.into() {
            None => js_try! ( return @{self.as_ref()}.openKeyCursor(); ),
            Some(range) => {
                match direction.into() {
                    None => js_try! ( return @{self.as_ref()}.openKeyCursor(@{range.as_ref()}); ),
                    Some(direction) => js_try! ( return @{self.as_ref()}.openKeyCursor(@{range.as_ref()}, @{cursor_direction_to_string(direction)}); )
                }
            }
        }.unwrap()
    }

}

/// Provides asynchronous access to an index in a database.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBIndex")]
pub struct DbIndex( Reference );

impl DbObjectStoreIndexSharedMethods for DbIndex {}

impl DbIndex {
    
    /// The object_store property of the DbIndex interface returns the name of the object store referenced by the current index.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex/objectStore)
    pub fn object_store( &self ) -> DbObjectStore {
        js! ( return @{self.as_ref()}.objectStore ).try_into().unwrap()
    }
    
    /// Affects how the index behaves when the result of evaluating the index's key path yields an array. If `true`, there is one record in the index for each item in an array of keys. If `false`, then there is one record for each key that is an array.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex/multiEntry)
    pub fn multi_entry( &self ) -> bool {
        js! (
            return @{self.as_ref()}.multiEntry;
        ).try_into().unwrap()
    }
    
    /// If `true`, this index does not allow duplicate values for a key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbIndex/unique)
    pub fn unique( &self ) -> bool {
        js! (
            return @{self.as_ref()}.unique;
        ).try_into().unwrap()
    }

}

error_enum_boilerplate! {
    DbObjectStoreDeleteError,
    /// This object store's transaction is inactive.
    TransactionInactiveError,
    /// The object store's transaction mode is read-only.
    ReadOnlyError,
    /// The object store has been deleted.
    InvalidStateError,
    /// The key is not a valid key or a key range.
    DataError
}

/// The `DbObjectStore` interface of the IndexedDB API represents an object store in a database
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBObjectStore")]
pub struct DbObjectStore( Reference );

impl DbObjectStoreIndexSharedMethods for DbObjectStore {}

impl DbObjectStore {
       
    /// The index_names read-only property of the `DbObjectStore` interface returns a list of th
    /// names of indexes on objects in this object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/indexNames)
    pub fn index_names( &self ) -> DOMStringList {
        js! ( return @{self}.indexNames ).try_into().unwrap()
    }
    
    /// The `DbTransaction` object to which this object store belongs.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/transaction)
    pub fn transaction( &self ) -> DbTransaction {
        js! (
            return @{self.as_ref()}.transaction;
        ).try_into().unwrap()
    }
    
    /// Returns the value of the auto increment flag for this object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/autoIncrement)
    pub fn auto_increment( &self ) -> bool {
        js! (
            return @{self.as_ref()}.autoIncrement;
        ).try_into().unwrap()
    }
    
    /// Updates a given record in a database, or inserts a new record if the given item does not already exist.
    /// The key is only needed if 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/put)
    pub fn put<T: Into<Option<Value>>>( &self, value: Value, key: T) -> Result<DbRequest, DbUpdateError> {
        match key.into() {
            None => js_try! (
                return @{self.as_ref()}.put(@{value.as_ref()});
            ),
            Some(key) => js_try! (
                return @{self.as_ref()}.put(@{value.as_ref()}, @{key.as_ref()});
            )
        }.unwrap()
    }
    
    /// Returns an `DbRequest` object, and, in a separate thread, creates a structured clone of the value, and stores the cloned value in the object store. This is for adding new records to an object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/add)
    pub fn add<T: Into<Option<Value>>>( &self, value: Value, key: T) -> Result<DbRequest, DbAddError> {
        match key.into() {
            None => js_try! (
                return @{self.as_ref()}.add(@{value.as_ref()});
            ),
            Some(key) => js_try! (
                return @{self.as_ref()}.add(@{value.as_ref()}, @{key.as_ref()});
            )
        }.unwrap()
    }

    /// returns an `DbRequest` object, and, in a separate thread, deletes the specified record or records.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/delete)
    pub fn delete( &self, query: Value) -> Result<DbRequest, DbObjectStoreDeleteError> {
        js_try! (
            return @{self.as_ref()}.delete(@{query.as_ref()});
        ).unwrap()
    }
    
    /// Returns an DbRequest object, and clears this object store in a separate thread
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/clear)
    pub fn clear( &self ) -> Result<DbRequest, DbClearError> {
        js_try! (
            return @{self.as_ref()}.clear();
        ).unwrap()
    }
    
    /// opens a named index in the current object store
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/index)
    pub fn index( &self, name: &str) -> Result<DbIndex, DbIndexError> {
        js_try! (
            return @{self.as_ref()}.index(@{name});
        ).unwrap()
    }

    // [NewObject] DbIndex createIndex(DOMString name, (DOMString or sequence<DOMString>) keyPath, optional DbIndexParameters options);
    /// Creates and returns a new `DbIndex` object in the connected database.
    ///
    /// Note that this method must be called only from a VersionChange
    /// transaction mode callback.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/createIndex)
    pub fn create_index( &self, name: &str, key_path: &str, options: Value) -> DbIndex { // TODO, how am I doing the optinal options?
        js! (
            return @{self.as_ref()}.createIndex(@{name}, @{key_path}, @{options.as_ref()});
        ).try_into().unwrap()
    }
    
    /// Destroys the index with the specified name in the connected database, used during a version upgrade.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbObjectStore/deleteIndex)
    pub fn delete_index( &self, name: &str) {
        js! {
            return @{self.as_ref()}.deleteIndex(@{name});
        }
    }
}

/* dictionary DbIndexParameters {
  boolean unique = false;
  boolean multiEntry = false;
};*/

/// An DbTransactionMode object defining the mode for isolating access to
/// data in the current object stores.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction/mode)
#[derive(Debug)]
pub enum DbTransactionMode {
    /// Allows data to be read but not changed.
    ReadOnly,
    /// Allows reading and writing of data in existing data stores to be changed.
    ReadWrite,
    /// Allows any operation to be performed, including ones that delete and
    /// create object stores and indexes. This mode is for updating the version
    /// number of transactions that were started using DbDatabase.set_version().
    /// Transactions of this mode cannot run concurrently with other transactions.
    /// Transactions in this mode are known as "upgrade transactions."
    VersionChange
}

fn transaction_mode_to_string( mode: DbTransactionMode ) -> String {
    match mode {
        DbTransactionMode::ReadOnly => "readonly".to_string(),
        DbTransactionMode::ReadWrite => "readwrite".to_string(),
        DbTransactionMode::VersionChange => "versionchange".to_string()
    }
}

fn string_to_transaction_mode( mode: &str ) -> DbTransactionMode {
    if mode.eq("readonly") {
        return DbTransactionMode::ReadOnly;
    } else if mode.eq("readwrite") {
        return DbTransactionMode::ReadWrite;
    } else if mode.eq("versionchange") {
        return DbTransactionMode::VersionChange;
    } else {
        unreachable!("Unknown transaction mode \"{}\".", mode);
    }
}

error_enum_boilerplate! {
    DbObjectStoreError,

    /// The requested object store is not in this transaction's scope.
    NotFoundError,
    /// The request was made on a source object that has been deleted or removed, or
    /// if the transaction has finished.
    InvalidStateError
}

/// The `DbTransaction` interface of the IndexedDB API provides a static, asynchronous transaction on a database using event handlers.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBTransaction")]
pub struct DbTransaction( Reference );

impl IEventTarget for DbTransaction {}

impl DbTransaction {

    /// The object_store_names read-only property of the DbTransaction interface returns
    /// a DOMStringList of names of DbObjectStore objects.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction/objectStoreNames)
    pub fn object_store_names( &self ) -> DOMStringList {
        js! ( return @{self}.objectStoreNames ).try_into().unwrap()
    }
    
    /// The mode read-only property of the `DbTransaction` interface returns the
    /// current mode for accessing the data in the object stores in the scope of the
    /// transaction (i.e. is the mode to be read-only, or do you want to write to
    /// the object stores?) The default value is readonly.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction/mode
    pub fn mode( &self ) -> DbTransactionMode {
        let mode: String = js!( return @{self}.mode; ).try_into().unwrap();
        string_to_transaction_mode(&mode)
    }
    
    /// Returns the database connection with which this transaction is associated.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction/db)
    pub fn db( &self ) -> DbDatabase {
        js! (
            return @{self}.db();
        ).try_into().unwrap()
    }

    /// The DbTransaction.error property of the DbTransaction interface returns
    /// one of several types of error when there is an unsuccessful transaction.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction/error)
    pub fn error( &self ) -> Option<DomException> {
        js!( return @{self}.error; ).try_into().unwrap()
    }
    
    /// The object_store() method of the DbTransaction interface returns an object
    /// store that has already been added to the scope of this transaction.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction/objectStore)
    pub fn object_store( &self, name: &str) -> Result<DbObjectStore, DbObjectStoreError> {
        js_try! (
            return @{self.as_ref()}.objectStore(@{name});
        ).unwrap()
    }
    
    /// The abort() method of the DbTransaction interface rolls back all the
    /// changes to objects in the database associated with this transaction. 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbTransaction/abort)
    pub fn abort( &self ) -> Result<(), InvalidStateError> {
        js_try! ( @{self}.abort(); ).unwrap()
    }

}

error_enum_boilerplate! {
    DbCreateObjectStoreError,
    
    /// Occurs if the method was not called from a versionchange transaction
    /// callback. For older WebKit browsers, you must call first.
    InvalidStateError,
    
    /// Occurs if a request is made on a source database that doesn't exist (e.g.
    /// has been deleted or removed.)
    TransactionInactiveError, 

    /// An object store with the given name (based on case-sensitive comparison)
    /// already exists in the connected database.
    ConstraintError, 

    /// If autoIncrement is set to true and keyPath is either an empty string or an
    /// array containing an empty string.
    InvalidAccessError
}

error_enum_boilerplate! {
    DbDeleteObjectStoreError,

    /// Occurs if the method was not called from a versionchange transaction callback.
    /// For older WebKit browsers, you must call first.
    InvalidStateError,

    /// Occurs if a request is made on a source database that doesn't exist (e.g. has
    /// been deleted or removed.)
    TransactionInactiveError,

    /// You are trying to delete an object store that does not exist. Names are case sensitive.
    NotFoundError
}

/// The `DbDatabase` interface of the IndexedDB API provides a connection to a database.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBDatabase")]
pub struct DbDatabase( Reference );

impl IEventTarget for DbDatabase {}

impl DbDatabase {
    
    /// Returns the the name of the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase/name)
    pub fn name( &self ) -> String {
        js! (
            return @{self.as_ref()}.name;
        ).try_into().unwrap()
    }
    
    /// Returns the version of the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase/version)
    pub fn version( &self ) -> u64 {
        js! (
            return @{self.as_ref()}.version;
            ).try_into().unwrap()
    }
    
    /// The objectStoreNames read-only property of the DbDatabase interface is a
    /// DOMStringList containing a list of the names of the object stores currently
    /// in the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase/objectStoreNames)
    pub fn object_store_names( &self ) -> DOMStringList {
        js! ( return @{self}.objectStoreNames ).try_into().unwrap()
    }

    /// Immediately returns a transaction object (`DbTransaction`) containing
    /// the `DbTransaction.object_store` method, which you can use to access
    /// your object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase/transaction)
    pub fn transaction( &self, store_names: Vec<&str>, mode: DbTransactionMode) -> DbTransaction {
        js! (
            return @{self.as_ref()}.transaction(@{store_names}, @{transaction_mode_to_string(mode)});
        ).try_into().unwrap()
    }
    
    /// Returns immediately and closes the connection in a separate thread.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase/close)
    pub fn close( &self ) {
        js! {
            @{self.as_ref()}.close();
        }
    }

    /// Creates and returns a new object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase/createObjectStore)
    pub fn create_object_store( &self, name: &str, auto_increment: bool, key_path: &str) -> Result<DbObjectStore, DbCreateObjectStoreError> {
        js_try! (
            return @{self.as_ref()}.createObjectStore(@{name}, { autoIncrement: @{auto_increment}, key_path: @{key_path} } );
        ).unwrap()
    }
    
    /// Destroys the object store with the given name.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DbDatabase/deleteObjectStore)
    pub fn delete_object_store( &self, name: &str ) -> Result<(), DbDeleteObjectStoreError> {
        js_try! (
            @{self.as_ref()}.deleteObjectStore(@{name});
        ).unwrap()
    }

}

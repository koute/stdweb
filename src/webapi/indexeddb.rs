use webcore::value::{Value, Reference};
use webcore::try_from::{TryFrom, TryInto};
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::dom_exception::{DomException, InvalidStateError, TransactionInactiveError, DataError, InvalidAccessError, ReadOnlyError, DataCloneError, ConstraintError, NotFoundError};
use webapi::error::TypeError;
use webapi::dom_string_list::DOMStringList;

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
    Store(IDBObjectStore),
    Index(IDBIndex),
    Cursor(IDBCursor)
}

/// IDBRequestSharedMethode represents the methode that are shared between
/// IDBOpenDBRequest and IDBRequest.
// https://w3c.github.io/IndexedDB/#idbrequest
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
    fn error(&self) -> Option<DomException> {
        return js!( @{self.as_ref()}.error;).try_into().unwrap()
    }
    
    /// Returns the source of the request.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source)
    fn source( &self ) -> Option<IDBRequestSource> {
        let t: i32 = js!{
            if (@{self.as_ref()}.source instanceof IDBObjectStore) {
                return 0;
            } else  if (@{self.as_ref()}.source instanceof IDBIndex) {
                return 1;
            } else if (@{self.as_ref()}.source instanceof IDBCursor) {
                return 2;
            } else {
                panic!()
            }
        }.try_into().unwrap();
        match t {
            0 => Some(IDBRequestSource::Store(js!(return @{self.as_ref()}.source;).try_into().unwrap())),
            1 => Some(IDBRequestSource::Index(js!(return @{self.as_ref()}.source;).try_into().unwrap())),
            2 => Some(IDBRequestSource::Cursor(js!(return @{self.as_ref()}.source;).try_into().unwrap())),
            _ => None
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

/// The `IDBRequest` interface of the IndexedDB API provides access to results
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

/// The `IDBFactory` interface of the IndexedDB API lets applications
/// asynchronously access the indexed databases. The object that
/// implements the interface is `window.indexedDB`. 
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory)
// https://w3c.github.io/IndexedDB/#idbfactory
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
    /// An enum of the exceptions that IDBCursorSharedMethods.advance()
    /// and IDBCursorSharedMethods.next may throw.
    IDBAdvanceError,
    /// This IDBCursor's transaction is inactive.
    TransactionInactiveError,
    /// The value passed into the parameter was zero or a negative number.
    TypeError,
    /// The cursor is currently being iterated or has iterated past its end.
    InvalidStateError
}

error_enum_boilerplate! {
    IDBContinuePrimaryKeyError,
    /// This IDBCursor's transaction is inactive.
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
    IDBUpdateError,
    /// This IDBCursor's transaction is inactive.
    TransactionInactiveError,
    /// The transaction mode is read only.
    ReadOnlyError,
    /// The cursor was created using IDBIndex.openKeyCursor, is currently being iterated, or has iterated past its end.
    InvalidStateError,
    /// The underlying object store uses in-line keys and the property in the value at the object store's key path does not match the key in this cursor's position.
    DataError,
    ///The data being stored could not be cloned by the internal structured cloning algorithm.
    DataCloneError
}

///
error_enum_boilerplate! {
    ///
    IDBAddError,
    /// This IDBCursor's transaction is inactive.
    TransactionInactiveError,
    /// The transaction mode is read only.
    ReadOnlyError,
    /// The cursor was created using IDBIndex.openKeyCursor, is currently being iterated, or has iterated past its end.
    InvalidStateError,
    /// The underlying object store uses in-line keys and the property in the value at the object store's key path does not match the key in this cursor's position.
    DataError,
    ///The data being stored could not be cloned by the internal structured cloning algorithm.
    DataCloneError,
    /// An operation failed because the primary key constraint was violated (due to an already existing record with the same primary key value).
    ConstraintError
}

error_enum_boilerplate! {
    IDBCursorDeleteError,
    /// This IDBCursor's transaction is inactive.
    TransactionInactiveError,
    /// The transaction mode is read-only.
    ReadOnlyError,
    /// The cursor was created using IDBindex.openKeyCursor, is currently being iterated, or has iterated past its end.
    InvalidStateError
}

error_enum_boilerplate! {
    IDBClearError,
    /// The transaction associated with this operation is in read-only mode.
    ReadOnlyError,
    /// This IDBObjectStore's transaction is inactive.
    TransactionInactiveError
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

    /// The advance() method of the IDBCursor interface sets the number of times
    /// a cursor should move its position forward. 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/advance) 
    fn advance( &self, count: u32) -> Result<(), IDBAdvanceError> {
        js_try!( @{self.as_ref()}.advance(@{count}); ).unwrap()
    }
        
    /// The next() method of the IDBCursor interface advances the cursor to the
    /// next position along its direction, to the item whose key matches the optional
    /// key parameter. If no key (None) is specified, the cursor advances to the immediate
    /// next position, based on its direction.
    ///
    /// This function stands in for continue in the javascript interface. Continue
    /// is a keyword in rust and so needed to be renamed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)
    fn next<K: Into<Option<Value>>>( &self, key: K) -> Result<(), IDBAdvanceError> {
        match key.into() {
            None => js_try!( @{self.as_ref()}.continue(); ).unwrap(),
            Some(key) => js_try! ( @{self.as_ref()}.continue(@{key.as_ref()}); ).unwrap()
        }
    }

    /// The continuePrimaryKey() method of the IDBCursor interface advances
    /// the cursor to the to the item whose key matches the key parameter as
    /// well as whose primary key matches the primary key parameter.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continuePrimaryKey)
    fn continue_primary_key( &self, key: Value, primary_key: Value) -> Result<(), IDBContinuePrimaryKeyError> {
        js_try!( @{self.as_ref()}.continuePrimaryKey(@{key}, @{primary_key}); ).unwrap()
    }

    /// The update() method of the IDBCursor interface returns an IDBRequest
    /// object, and, in a separate thread, updates the value at the current
    /// position of the cursor in the object store. If the cursor points to
    /// a record that has just been deleted, a new record is created.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/update)
    fn update( &self, value: Value) -> Result<IDBRequest, IDBUpdateError> {
        js_try!( return @{self.as_ref()}.update(@{value.as_ref()}); ).unwrap()
    }

    /// The delete() method of the IDBCursor interface returns an IDBRequest
    /// object, and, in a separate thread, deletes the record at the cursor's
    /// position, without changing the cursor's position. Once the record is
    /// deleted, the cursor's value is set to null.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/delete)
    fn delete( &self ) -> Result<IDBRequest, IDBCursorDeleteError> {
        js_try!( return @{self.as_ref()}.delete(); ).unwrap() 
    }
}

/// The IDBCursor interface of the IndexedDB API represents a cursor for
/// traversing or iterating over multiple records in a database.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBCursor")]
pub struct IDBCursor( Reference );

impl IDBCursorSharedMethods for IDBCursor {}

/// The IDBCursorWithValue interface of the IndexedDB API represents a cursor
/// for traversing or iterating over multiple records in a database. It is
/// the same as the IDBCursor, except that it includes the value property.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBCursorWithValue")]
pub struct IDBCursorWithValue( Reference );

impl IDBCursorSharedMethods for IDBCursorWithValue {}

impl IDBCursorWithValue {

    /// Returns the value of the current cursor.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue/value)
    pub fn value( &self ) -> Value {
        js! (
            return @{self}.value
            ).try_into().unwrap()
    }
}

/// The IDBKeyRange interface of the IndexedDB API represents a continuous interval
/// over some data type that is used for keys. Records can be retrieved from
/// IDBObjectStore and IDBIndex objects using keys or a range of keys. You can limit
/// the range using lower and upper bounds. For example, you can iterate over all
/// values of a key in the value range A–Z.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBKeyRange")]
pub struct IDBKeyRange( Reference );

impl IDBKeyRange {

    // Static construction methods:
    
    /// The only() method of the IDBKeyRange interface creates a new key range
    /// containing a single value.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/only)
    pub fn only( value: Value ) -> Result<IDBKeyRange, DataError> {
        js_try! ( return IDBKeyRange.only(@{value}); ).unwrap()
    }
    
    /// The lower_bound() method of the IDBKeyRange interface creates a new key range
    /// with only a lower bound. if open is false it includes the lower endpoint
    /// value and is closed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)
    pub fn lower_bound( lower: Value, open: bool ) -> Result<IDBKeyRange, DataError> {
        js_try! ( return IDBKeyRange.lowerBound(@{lower}, @{open}); ).unwrap()
    }
    
    /// The upper_bound() method of the IDBKeyRange interface creates a new key range
    /// with only an apper bound. if open is false it includes the upper endpoint
    /// value and is closed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)
    pub fn upper_bound( upper: Value, open: bool ) -> Result<IDBKeyRange, DataError> {
        js_try! ( return IDBKeyRange.upperBound(@{upper}, @{open}); ).unwrap()
    }
    
    /// The bound() method of the IDBKeyRange interface creates a new key range
    /// with the specified upper and lower bounds. The bounds can be open (that
    /// is, the bounds exclude the endpoint values) or closed (that is, the bounds
    /// include the endpoint values).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)
    pub fn bound (lower: Value, upper: Value, lower_open: bool, upper_open: bool) -> Result<IDBKeyRange, DataError> {
        js_try! (
            return IDBKeyRange.boundound(@{lower}, @{upper}, @{lower_open}, @{upper_open});
        ).unwrap()
    }
        
    /// Lower bound of the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lower)
    pub fn lower( &self ) -> Value {
        js!( return @{self}.lower; ).try_into().unwrap()
    }

    /// Upper bound of the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upper)
    pub fn upper( &self ) -> Value {
        js!( return @{self}.upper; ).try_into().unwrap()
    }

    /// Returns false if the lower-bound value is included in the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerOpen)
    pub fn lower_open( &self ) -> bool {
        js!( return @{self}.lowerOpen; ).try_into().unwrap()
    }

    /// Returns false if the upper-bound value is included in the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperOpen)
    pub fn upper_open( &self ) -> bool {
        js!( return @{self}.upperOpen; ).try_into().unwrap()
    }

    /// The includes() method of the IDBKeyRange interface returns a boolean
    /// indicating whether a specified key is inside the key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/includes)
    pub fn includes( &self, value: Value ) -> Result<bool, DataError> {
        js_try! ( return @{self}.includes(@{value}); ).unwrap()
    }
}

#[derive(Debug)]
pub enum IDBKeyOrKeyRange {
    None,
    Value(Value),
    Range(IDBKeyRange)
}

error_enum_boilerplate! {
    IDBSetNameError,

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
    IDBQueryError,
    
    ///  This IDBIndex's transaction is inactive.
    TransactionInactiveError,
    
    /// The key or key range provided contains an invalid key.
    DataError,
    
    ///  The IDBIndex has been deleted or removed.
    InvalidStateError
}

error_enum_boilerplate! {
    IDBIndexError,
    /// The source object store has been deleted, or the transaction for the object store has finished.
    InvalidStateError,
    /// There is no index with the given name (case-sensitive) in the database.
    NotFoundError
         
}

/// This trait contains mothods that are Identicle in both IDBIndex IDBObjectStore
pub trait IDBObjectStoreIndexSharedMethods: AsRef< Reference > {

    /// Returns the name of this index or object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    fn name( &self ) -> String {
        js! (
            return @{self.as_ref()}.name;
        ).try_into().unwrap()
    }

    /// Returns the name of this index or object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    fn set_name( &self, name: &str) -> Result<(), IDBSetNameError> {
        js_try! ( @{self.as_ref()}.name = @{name}; ).unwrap()
    }

    /// The key_path read-only property of the IDBObjectStore interface returns the
    /// key path of this object store. Or in the case of an IDBIndex, the current
    /// object store.
    fn key_path( &self ) -> Value {
        js!( return @{self.as_ref()}.keyPath; ).try_into().unwrap()
    }

    /// This is for retrieving specific records from an object store or index.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)
    fn get<Q: Into<IDBKeyOrKeyRange>>( &self, query: Q) -> Result<IDBRequest, IDBQueryError> {
        match query.into() {
            IDBKeyOrKeyRange::None => js_try! (
                return @{self.as_ref()}.get();
            ),
            IDBKeyOrKeyRange::Value(value) => js_try! (
                return @{self.as_ref()}.get(@{value.as_ref()});
            ),
            IDBKeyOrKeyRange::Range(range) => js_try! (
                return @{self.as_ref()}.get(@{range.as_ref()});
            )
        }.unwrap()
    }

    /// Returns an IDBRequest object, and, in a separate thread retrieves and
    /// returns the record key for the object matching the specified parameter.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)
    fn get_key<Q: Into<IDBKeyOrKeyRange>>( &self, query: Q) -> Result<IDBRequest, IDBQueryError> {
        match query.into() {
            IDBKeyOrKeyRange::None => js_try! (
                return @{self.as_ref()}.getKey();
            ),
            IDBKeyOrKeyRange::Value(value) => js_try! (
                return @{self.as_ref()}.getKey(@{value.as_ref()});
            ),
            IDBKeyOrKeyRange::Range(range) => js_try! (
                return @{self.as_ref()}.getKey(@{range.as_ref()});
            )
        }.unwrap()
    }
    
    /// The get_ll() method retrieves all objects that are inside the index or
    /// object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)
    fn get_all<Q: Into<IDBKeyOrKeyRange>, C: Into<Option<u32>>>( &self, query: Q, count: C) -> Result<IDBRequest, IDBQueryError> {
        match query.into() {
            IDBKeyOrKeyRange::None => js_try! ( return @{self.as_ref()}.getAll(); ),
            IDBKeyOrKeyRange::Value(value) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAll(@{value.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAll(@{value.as_ref()}, @{count}); )
                }
            },
            IDBKeyOrKeyRange::Range(range) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAll(@{range.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAll(@{range.as_ref()}, @{count}); )
                }
            }
        }.unwrap()
    }
    
    // Acording to the mozilla documentation the IDBIndex version does not
    // Throw DataError.
    /// The get_all_keys() method returns an IDBRequest object retrieves record keys
    /// for all objects matching the specified parameter or all objects if no
    /// parameters are given.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)
    fn get_all_keys<Q: Into<IDBKeyOrKeyRange>, C: Into<Option<u32>>>( &self, query: Q, count: C) -> Result<IDBRequest, IDBQueryError> {
        match query.into() {
            IDBKeyOrKeyRange::None => js_try! ( return @{self.as_ref()}.getAllKeys(); ),
            IDBKeyOrKeyRange::Value(value) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAllKeys(@{value.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAllKeys(@{value.as_ref()}, @{count}); )
                }
            },
            IDBKeyOrKeyRange::Range(range) => {
                match count.into() {
                    None => js_try! ( return @{self.as_ref()}.getAllKeys(@{range.as_ref()}); ),
                    Some(count) => js_try! ( return @{self.as_ref()}.getAllKeys(@{range.as_ref()}, @{count}); )
                }
            }
        }.unwrap()
    }
    
    /// Returns an IDBRequest object, and, in a separate thread, returns the total number of records that match the provided key or IDBKeyRange
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)
    fn count<Q: Into<IDBKeyOrKeyRange>>( &self, query: Q) -> Result<IDBRequest, IDBQueryError> {
        match query.into() {
            IDBKeyOrKeyRange::None => js_try! (
                return @{self.as_ref()}.count();
            ),
            IDBKeyOrKeyRange::Value(value) => js_try! (
                return @{self.as_ref()}.count(@{value.as_ref()});
            ),
            IDBKeyOrKeyRange::Range(range) => js_try! (
                return @{self.as_ref()}.count(@{range.as_ref()});
            )
        }.unwrap()
    }

    /// The open_cursor() method returns an IDBRequest object, and, in a separate
    /// thread, creates a cursor over the specified key range.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)
    fn open_cursor<Q: Into<Option<IDBKeyRange>>, D: Into<Option<IDBCursorDirection>>>( &self, range: Q, direction: D) -> Result<IDBRequest, IDBQueryError> {
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
    
    /// The open_key_cursor() method returns an IDBRequest object, and, in a
    /// separate thread, creates a cursor over the specified key range, as arranged
    /// by this index.
    /// 
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)
    fn open_key_cursor<Q: Into<Option<IDBKeyRange>>, D: Into<Option<IDBCursorDirection>>>( &self, range: Q, direction: D) -> Result<IDBRequest, IDBQueryError> {
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
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBIndex")]
pub struct IDBIndex( Reference );

impl IDBObjectStoreIndexSharedMethods for IDBIndex {}

impl IDBIndex {
    
    /// The object_store property of the IDBIndex interface returns the name of the object store referenced by the current index.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/objectStore)
    pub fn object_store( &self ) -> IDBObjectStore {
        js! ( return @{self.as_ref()}.objectStore ).try_into().unwrap()
    }
    
    /// Affects how the index behaves when the result of evaluating the index's key path yields an array. If `true`, there is one record in the index for each item in an array of keys. If `false`, then there is one record for each key that is an array.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/multiEntry)
    pub fn multi_entry( &self ) -> bool {
        js! (
            return @{self.as_ref()}.multiEntry;
        ).try_into().unwrap()
    }
    
    /// If `true`, this index does not allow duplicate values for a key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/unique)
    pub fn unique( &self ) -> bool {
        js! (
            return @{self.as_ref()}.unique;
        ).try_into().unwrap()
    }

}

error_enum_boilerplate! {
    IDBObjectStoreDeleteError,
    /// This object store's transaction is inactive.
    TransactionInactiveError,
    /// The object store's transaction mode is read-only.
    ReadOnlyError,
    /// The object store has been deleted.
    InvalidStateError,
    /// The key is not a valid key or a key range.
    DataError
}

/// The `IDBObjectStore` interface of the IndexedDB API represents an object store in a database
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBObjectStore")]
pub struct IDBObjectStore( Reference );

impl IDBObjectStoreIndexSharedMethods for IDBObjectStore {}

impl IDBObjectStore {
       
    /// The index_names read-only property of the `IDBObjectStore` interface returns a list of th
    /// names of indexes on objects in this object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames)
    pub fn index_names( &self ) -> DOMStringList {
        js! ( return @{self}.indexNames ).try_into().unwrap()
    }
    
    /// The `IDBTransaction` object to which this object store belongs.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)
    pub fn transaction( &self ) -> IDBTransaction {
        js! (
            return @{self.as_ref()}.transaction;
        ).try_into().unwrap()
    }
    
    /// Returns the value of the auto increment flag for this object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)
    pub fn auto_increment( &self ) -> bool {
        js! (
            return @{self.as_ref()}.autoIncrement;
        ).try_into().unwrap()
    }
    
    /// Updates a given record in a database, or inserts a new record if the given item does not already exist.
    /// The key is only needed if 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)
    pub fn put<T: Into<Option<Value>>>( &self, value: Value, key: T) -> Result<IDBRequest, IDBUpdateError> {
        match key.into() {
            None => js_try! (
                return @{self.as_ref()}.put(@{value.as_ref()});
            ),
            Some(key) => js_try! (
                return @{self.as_ref()}.put(@{value.as_ref()}, @{key.as_ref()});
            )
        }.unwrap()
    }
    
    /// Returns an `IDBRequest` object, and, in a separate thread, creates a structured clone of the value, and stores the cloned value in the object store. This is for adding new records to an object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)
    pub fn add<T: Into<Option<Value>>>( &self, value: Value, key: T) -> Result<IDBRequest, IDBAddError> {
        match key.into() {
            None => js_try! (
                return @{self.as_ref()}.add(@{value.as_ref()});
            ),
            Some(key) => js_try! (
                return @{self.as_ref()}.add(@{value.as_ref()}, @{key.as_ref()});
            )
        }.unwrap()
    }

    /// returns an `IDBRequest` object, and, in a separate thread, deletes the specified record or records.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)
    pub fn delete( &self, query: Value) -> Result<IDBRequest, IDBObjectStoreDeleteError> {
        js_try! (
            return @{self.as_ref()}.delete(@{query.as_ref()});
        ).unwrap()
    }
    
    /// Returns an IDBRequest object, and clears this object store in a separate thread
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)
    pub fn clear( &self ) -> Result<IDBRequest, IDBClearError> {
        js_try! (
            return @{self.as_ref()}.clear();
        ).unwrap()
    }
    
    /// opens a named index in the current object store
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)
    pub fn index( &self, name: &str) -> Result<IDBIndex, IDBIndexError> {
        js_try! (
            return @{self.as_ref()}.index(@{name});
        ).unwrap()
    }

    // [NewObject] IDBIndex createIndex(DOMString name, (DOMString or sequence<DOMString>) keyPath, optional IDBIndexParameters options);
    /// Creates and returns a new `IDBIndex` object in the connected database.
    ///
    /// Note that this method must be called only from a VersionChange
    /// transaction mode callback.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
    pub fn create_index( &self, name: &str, key_path: &str, options: Value) -> IDBIndex { // TODO, how am I doing the optinal options?
        js! (
            return @{self.as_ref()}.createIndex(@{name}, @{key_path}, @{options.as_ref()});
        ).try_into().unwrap()
    }
    
    /// Destroys the index with the specified name in the connected database, used during a version upgrade.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)
    pub fn delete_index( &self, name: &str) {
        js! {
            return @{self.as_ref()}.deleteIndex(@{name});
        }
    }
}

/* dictionary IDBIndexParameters {
  boolean unique = false;
  boolean multiEntry = false;
};*/

/// An IDBTransactionMode object defining the mode for isolating access to
/// data in the current object stores.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode)
#[derive(Debug)]
pub enum IDBTransactionMode {
    /// Allows data to be read but not changed.
    ReadOnly,
    /// Allows reading and writing of data in existing data stores to be changed.
    ReadWrite,
    /// Allows any operation to be performed, including ones that delete and
    /// create object stores and indexes. This mode is for updating the version
    /// number of transactions that were started using IDBDatabase.set_version().
    /// Transactions of this mode cannot run concurrently with other transactions.
    /// Transactions in this mode are known as "upgrade transactions."
    VersionChange
}

fn transaction_mode_to_string( mode: IDBTransactionMode ) -> String {
    match mode {
        IDBTransactionMode::ReadOnly => "readonly".to_string(),
        IDBTransactionMode::ReadWrite => "readwrite".to_string(),
        IDBTransactionMode::VersionChange => "versionchange".to_string()
    }
}

fn string_to_transaction_mode( mode: &str ) -> IDBTransactionMode {
    if mode.eq("readonly") {
        return IDBTransactionMode::ReadOnly;
    } else if mode.eq("readwrite") {
        return IDBTransactionMode::ReadWrite;
    } else if mode.eq("versionchange") {
        return IDBTransactionMode::VersionChange;
    } else {
        unreachable!("Unknown transaction mode \"{}\".", mode);
    }
}

error_enum_boilerplate! {
    IDBObjectStoreError,

    /// The requested object store is not in this transaction's scope.
    NotFoundError,
    /// The request was made on a source object that has been deleted or removed, or
    /// if the transaction has finished.
    InvalidStateError
}

/// The `IDBTransaction` interface of the IndexedDB API provides a static, asynchronous transaction on a database using event handlers.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBTransaction")]
pub struct IDBTransaction( Reference );

impl IEventTarget for IDBTransaction {}

impl IDBTransaction {

    /// The object_store_names read-only property of the IDBTransaction interface returns
    /// a DOMStringList of names of IDBObjectStore objects.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStoreNames)
    pub fn object_store_names( &self ) -> DOMStringList {
        js! ( return @{self}.objectStoreNames ).try_into().unwrap()
    }
    
    /// The mode read-only property of the `IDBTransaction` interface returns the
    /// current mode for accessing the data in the object stores in the scope of the
    /// transaction (i.e. is the mode to be read-only, or do you want to write to
    /// the object stores?) The default value is readonly.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode
    pub fn mode( &self ) -> IDBTransactionMode {
        let mode: String = js!( return @{self}.mode; ).try_into().unwrap();
        string_to_transaction_mode(&mode)
    }
    
    /// Returns the database connection with which this transaction is associated.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db)
    pub fn db( &self ) -> IDBDatabase {
        js! (
            return @{self}.db();
        ).try_into().unwrap()
    }

    /// The IDBTransaction.error property of the IDBTransaction interface returns
    /// one of several types of error when there is an unsuccessful transaction.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/error)
    pub fn error( &self ) -> Option<DomException> {
        js!( return @{self}.error; ).try_into().unwrap()
    }
    
    /// The object_store() method of the IDBTransaction interface returns an object
    /// store that has already been added to the scope of this transaction.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStore)
    pub fn object_store( &self, name: &str) -> Result<IDBObjectStore, IDBObjectStoreError> {
        js_try! (
            return @{self.as_ref()}.objectStore(@{name});
        ).unwrap()
    }
    
    /// The abort() method of the IDBTransaction interface rolls back all the
    /// changes to objects in the database associated with this transaction. 
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/abort)
    pub fn abort( &self ) -> Result<(), InvalidStateError> {
        js_try! ( @{self}.abort(); ).unwrap()
    }

}

error_enum_boilerplate! {
    IDBCreateObjectStoreError,
    
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
    IDBDeleteObjectStoreError,

    /// Occurs if the method was not called from a versionchange transaction callback.
    /// For older WebKit browsers, you must call first.
    InvalidStateError,

    /// Occurs if a request is made on a source database that doesn't exist (e.g. has
    /// been deleted or removed.)
    TransactionInactiveError,

    /// You are trying to delete an object store that does not exist. Names are case sensitive.
    NotFoundError
}

/// The `IDBDatabase` interface of the IndexedDB API provides a connection to a database.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "IDBDatabase")]
pub struct IDBDatabase( Reference );

impl IEventTarget for IDBDatabase {}

impl IDBDatabase {
    
    /// Returns the the name of the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)
    pub fn name( &self ) -> String {
        js! (
            return @{self.as_ref()}.name;
        ).try_into().unwrap()
    }
    
    /// Returns the version of the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)
    pub fn version( &self ) -> u64 {
        js! (
            return @{self.as_ref()}.version;
            ).try_into().unwrap()
    }
    
    /// The objectStoreNames read-only property of the IDBDatabase interface is a
    /// DOMStringList containing a list of the names of the object stores currently
    /// in the connected database.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames)
    pub fn object_store_names( &self ) -> DOMStringList {
        js! ( return @{self}.objectStoreNames ).try_into().unwrap()
    }

    /// Immediately returns a transaction object (`IDBTransaction`) containing
    /// the `IDBTransaction.object_store` method, which you can use to access
    /// your object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    pub fn transaction( &self, store_names: Vec<&str>, mode: IDBTransactionMode) -> IDBTransaction {
        js! (
            return @{self.as_ref()}.transaction(@{store_names}, @{transaction_mode_to_string(mode)});
        ).try_into().unwrap()
    }
    
    /// Returns immediately and closes the connection in a separate thread.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)
    pub fn close( &self ) {
        js! {
            @{self.as_ref()}.close();
        }
    }

    /// Creates and returns a new object store.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)
    pub fn create_object_store( &self, name: &str, auto_increment: bool, key_path: &str) -> Result<IDBObjectStore, IDBCreateObjectStoreError> {
        js_try! (
            return @{self.as_ref()}.createObjectStore(@{name}, { autoIncrement: @{auto_increment}, key_path: @{key_path} } );
        ).unwrap()
    }
    
    /// Destroys the object store with the given name.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)
    pub fn delete_object_store( &self, name: &str ) -> Result<(), IDBDeleteObjectStoreError> {
        js_try! (
            @{self.as_ref()}.deleteObjectStore(@{name});
        ).unwrap()
    }

}

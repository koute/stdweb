use webcore::value::Reference;
use webapi::event::{IEvent, Event, ConcreteEvent};
use webcore::try_from::TryInto;

/// The `DbSuccessEvent` handler is fired when a and Indexed DB request succeed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/success)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(subclass_of(Event))]
pub struct DbSuccessEvent( Reference );

impl IEvent for DbSuccessEvent {}

impl ConcreteEvent for DbSuccessEvent {
    const EVENT_TYPE: &'static str = "success";
}

/// This event is fired if a new verion of a database has been requested.
///
/// [(JavaScript docs)](https://www.w3.org/TR/IndexedDB/#events)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(subclass_of(Event))]
pub struct DbVersionChangeEvent( Reference );

impl IEvent for DbVersionChangeEvent {}

impl ConcreteEvent for DbVersionChangeEvent {
    const EVENT_TYPE: &'static str = "upgradeneeded";
}

impl DbVersionChangeEvent  {
    /// Returns the previous version of the database.
    pub fn old_version( &self ) -> u64 {
        js! (
            return @{self.as_ref()}.oldVersion;
        ).try_into().unwrap()
    }
    
    /// Returns the new version of the database, or null if the database is being deleted.
    pub fn new_version( &self ) -> Option<u64> {
        js! (
            return @{self.as_ref()}.newVersion;
        ).try_into().unwrap()
    }
    
}

/// This event is fired when a transaction completes successfully.
/// https://www.w3.org/TR/IndexedDB/#transaction
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(subclass_of(Event))]
pub struct DbCompleteEvent( Reference );

impl IEvent for DbCompleteEvent {}

impl ConcreteEvent for DbCompleteEvent {
    const EVENT_TYPE: &'static str = "complete";
}

/// This event is fired when a transaction errors.
/// https://www.w3.org/TR/IndexedDB/#transaction
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(subclass_of(Event))]
pub struct DbErrorEvent( Reference );

impl IEvent for DbErrorEvent {}

impl ConcreteEvent for DbErrorEvent {
    const EVENT_TYPE: &'static str = "error";
}

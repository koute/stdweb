use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event::{IEvent, Event, ConcreteEvent};

/// The `IProgressEvent` interface represents progress-related
/// events.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)
pub trait IProgressEvent: IEvent {
    /// Indicates whether the progress is measureable.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)
    #[inline]
    fn length_computable( &self ) -> bool {
        js!(
            return @{self.as_ref()}.lengthComputable;
        ).try_into().unwrap()
    }

    /// Returns the amount of work already performed by the underlying process.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)
    #[inline]
    fn loaded( &self ) -> u64 {
        js!(
            return @{self.as_ref()}.loaded;
        ).try_into().unwrap()
    }

    /// Returns the total amount of work that the underlying process will perform.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)
    #[inline]
    fn total( &self ) -> u64 {
        js!(
            return @{self.as_ref()}.total;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IProgressEvent](trait.IProgressEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(subclass_of(Event))]
pub struct ProgressRelatedEvent( Reference );

impl IEvent for ProgressRelatedEvent {}
impl IProgressEvent for ProgressRelatedEvent {}

/// The `ProgressEvent` is fired to indicate that an operation is in progress.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/progress)
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ProgressEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressEvent( Reference );

impl IEvent for ProgressEvent {}
impl IProgressEvent for ProgressEvent {}
impl ConcreteEvent for ProgressEvent {
    const EVENT_TYPE: &'static str = "progress";
}

/// The `ProgressLoadEvent` is fired when progress has successful finished.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/load_(ProgressEvent))
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ProgressEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressLoadEvent( Reference );

impl IEvent for ProgressLoadEvent {}
impl IProgressEvent for ProgressLoadEvent {}
impl ConcreteEvent for ProgressLoadEvent {
    const EVENT_TYPE: &'static str = "load";
}

/// The `LoadStartEvent` is fired when progress has begun.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/loadstart)
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ProgressEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct LoadStartEvent( Reference );

impl IEvent for LoadStartEvent {}
impl IProgressEvent for LoadStartEvent {}
impl ConcreteEvent for LoadStartEvent {
    const EVENT_TYPE: &'static str = "loadstart";
}

/// The `LoadEndEvent` is fired when progress has stopped,
/// e.g. after `ProgressErrorEvent`, `ProgressAbortEvent`
/// or `ProgressLoadEvent` have been dispatched.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/loadend)
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ProgressEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct LoadEndEvent( Reference );

impl IEvent for LoadEndEvent {}
impl IProgressEvent for LoadEndEvent {}
impl ConcreteEvent for LoadEndEvent {
    const EVENT_TYPE: &'static str = "loadend";
}

/// The `ProgressAbortEvent` is fired when the progress has been aborted.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/abort_(ProgressEvent))
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ProgressEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressAbortEvent( Reference );

impl IEvent for ProgressAbortEvent {}
impl IProgressEvent for ProgressAbortEvent {}
impl ConcreteEvent for ProgressAbortEvent {
    const EVENT_TYPE: &'static str = "abort";
}

/// The `ProgressErrorEvent` is fired when the progress has failed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/error_(ProgressEvent))
#[derive(Clone, Debug, ReferenceType)]
#[reference(instance_of = "ProgressEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressErrorEvent( Reference );

impl IEvent for ProgressErrorEvent {}
impl IProgressEvent for ProgressErrorEvent {}
impl ConcreteEvent for ProgressErrorEvent {
    const EVENT_TYPE: &'static str = "error";
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_progress_event() {
        let event: ProgressEvent = js!(
            return new ProgressEvent(
                @{ProgressEvent::EVENT_TYPE},
                {
                    lengthComputable: true,
                    loaded: 10,
                    total: 100,
                }
            );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ProgressEvent::EVENT_TYPE );
        assert!( event.length_computable() );
        assert_eq!( event.loaded(), 10 );
        assert_eq!( event.total(), 100 );
    }

    #[test]
    fn test_load_start_event() {
        let event: LoadStartEvent = js!(
            return new ProgressEvent( @{LoadStartEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), LoadStartEvent::EVENT_TYPE );
    }

    #[test]
    fn test_load_end_event() {
        let event: LoadEndEvent = js!(
            return new ProgressEvent( @{LoadEndEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), LoadEndEvent::EVENT_TYPE );
    }
}

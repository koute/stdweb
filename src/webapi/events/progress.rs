use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event::{IEvent, Event};

/// The `IProgressEvent` interface represents progress-related
/// events.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)
// https://xhr.spec.whatwg.org/#progressevent
pub trait IProgressEvent: IEvent {
    /// Indicates whether the progress is measureable.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)
    // https://xhr.spec.whatwg.org/#ref-for-dom-progressevent-lengthcomputable
    #[inline]
    fn length_computable( &self ) -> bool {
        js!(
            return @{self.as_ref()}.lengthComputable;
        ).try_into().unwrap()
    }

    /// Returns the amount of work already performed by the underlying process.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)
    // https://xhr.spec.whatwg.org/#ref-for-dom-progressevent-loaded
    #[inline]
    fn loaded( &self ) -> u64 {
        js!(
            return @{self.as_ref()}.loaded;
        ).try_into().unwrap()
    }

    /// Returns the total amount of work that the underlying process will perform.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)
    // https://xhr.spec.whatwg.org/#ref-for-dom-progressevent-total
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
// https://xhr.spec.whatwg.org/#progressevent
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(subclass_of(Event))]
pub struct ProgressRelatedEvent( Reference );

impl IEvent for ProgressRelatedEvent {}
impl IProgressEvent for ProgressRelatedEvent {}

/// The `ProgressEvent` is fired to indicate that an operation is in progress.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/progress)
// https://xhr.spec.whatwg.org/#event-xhr-progress
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(event = "progress")]
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressEvent( Reference );

impl IEvent for ProgressEvent {}
impl IProgressEvent for ProgressEvent {}

/// The `ProgressLoadEvent` is fired when progress has successful finished.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/load_(ProgressEvent))
// https://xhr.spec.whatwg.org/#event-xhr-load
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(event = "load")]
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressLoadEvent( Reference );

impl IEvent for ProgressLoadEvent {}
impl IProgressEvent for ProgressLoadEvent {}

/// The `LoadStartEvent` is fired when progress has begun.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/loadstart)
// https://xhr.spec.whatwg.org/#event-xhr-loadstart
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(event = "loadstart")]
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct LoadStartEvent( Reference );

impl IEvent for LoadStartEvent {}
impl IProgressEvent for LoadStartEvent {}

/// The `LoadEndEvent` is fired when progress has stopped,
/// e.g. after `ProgressErrorEvent`, `ProgressAbortEvent`
/// or `ProgressLoadEvent` have been dispatched.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/loadend)
// https://xhr.spec.whatwg.org/#event-xhr-loadend
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(event = "loadend")]
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct LoadEndEvent( Reference );

impl IEvent for LoadEndEvent {}
impl IProgressEvent for LoadEndEvent {}

/// The `ProgressAbortEvent` is fired when the progress has been aborted.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/abort_(ProgressEvent))
// https://xhr.spec.whatwg.org/#event-xhr-abort
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(event = "abort")]
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressAbortEvent( Reference );

impl IEvent for ProgressAbortEvent {}
impl IProgressEvent for ProgressAbortEvent {}

/// The `ProgressErrorEvent` is fired when the progress has failed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/error_(ProgressEvent))
// https://xhr.spec.whatwg.org/#event-xhr-error
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "ProgressEvent")]
#[reference(event = "error")]
#[reference(subclass_of(Event, ProgressRelatedEvent))]
pub struct ProgressErrorEvent( Reference );

impl IEvent for ProgressErrorEvent {}
impl IProgressEvent for ProgressErrorEvent {}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::event::ConcreteEvent;

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

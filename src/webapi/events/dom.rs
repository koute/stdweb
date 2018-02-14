use webcore::value::Reference;
use webapi::event::{IEvent, IUiEvent, UiEvent, Event, ConcreteEvent};

/// The `ChangeEvent` is fired for input, select, and textarea
/// elements when a change to the element's value is committed
/// by the user. Unlike the input event, the change event is not
/// necessarily fired for each change to an element's value.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/change)
// https://html.spec.whatwg.org/#event-change
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")] // TODO: Better type check.
#[reference(subclass_of(Event))]
pub struct ChangeEvent( Reference );

impl IEvent for ChangeEvent {}
impl ConcreteEvent for ChangeEvent {
    const EVENT_TYPE: &'static str = "change";
}

/// The `InputEvent` is fired synchronously when the value of an
/// input, select, or textarea element is changed. For input elements
/// with type=checkbox or type=radio, the input event should fire when
/// a user toggles the control (via touch, mouse or keyboard) per the
/// HTML5 specification, but historically, this has not been the case.
/// Check compatibility, or attach to the change event instead for
/// elements of these types.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/input)
// https://html.spec.whatwg.org/#event-input
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")] // TODO: Better type check.
#[reference(subclass_of(Event))]
pub struct InputEvent( Reference );

impl IEvent for InputEvent {}
impl ConcreteEvent for InputEvent {
    const EVENT_TYPE: &'static str = "input";
}

/// The `ResourceLoadEvent` is fired when a resource and its dependent resources have finished loading.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/load)
// https://w3c.github.io/uievents/#load
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "UIEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, UiEvent))]
pub struct ResourceLoadEvent( Reference );

impl IEvent for ResourceLoadEvent {}
impl IUiEvent for ResourceLoadEvent {}
impl ConcreteEvent for ResourceLoadEvent {
    const EVENT_TYPE: &'static str = "load";
}

/// The `ResourceAbortEvent` is fired when the loading of a resource has been aborted.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/abort)
// https://w3c.github.io/uievents/#event-type-abort
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "UIEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, UiEvent))]
pub struct ResourceAbortEvent( Reference );

impl IEvent for ResourceAbortEvent {}
impl IUiEvent for ResourceAbortEvent {}
impl ConcreteEvent for ResourceAbortEvent {
    const EVENT_TYPE: &'static str = "abort";
}

/// The `ResourceErrorEvent` is fired when an error occurred; the exact circumstances vary,
/// since this event is used from a variety of APIs.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/error)
// https://w3c.github.io/uievents/#event-type-error
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "UIEvent")] // TODO: Better type check.
#[reference(subclass_of(Event, UiEvent))]
pub struct ResourceErrorEvent( Reference );

impl IEvent for ResourceErrorEvent {}
impl IUiEvent for ResourceErrorEvent {}
impl ConcreteEvent for ResourceErrorEvent {
    const EVENT_TYPE: &'static str = "error";
}

/// The resize event is fired when the document view has been resized.
///
/// MDN incorrectly documents this as a UIEvent, but in browsers it is actually
/// just an Event.
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/resize)
// https://drafts.csswg.org/cssom-view/#eventdef-window-resize
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")] // TODO: Better type check.
#[reference(subclass_of(Event))]
pub struct ResizeEvent( Reference );

impl IEvent for ResizeEvent {}
impl ConcreteEvent for ResizeEvent {
    const EVENT_TYPE: &'static str = "resize";
}

/// The readystatechange event is fired when the readyState attribute of a document has changed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/readystatechange)
// https://html.spec.whatwg.org/#event-readystatechange
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")] // TODO: Better type check.
#[reference(subclass_of(Event))]
pub struct ReadyStateChangeEvent( Reference );

impl IEvent for ReadyStateChangeEvent {}

impl ConcreteEvent for ReadyStateChangeEvent {
    const EVENT_TYPE: &'static str = "readystatechange";
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webcore::try_from::TryInto;

    #[test]
    fn test_change_event() {
        let event: ChangeEvent = js!(
            return new Event( @{ChangeEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ChangeEvent::EVENT_TYPE );
    }

    #[test]
    fn test_input_event() {
        let event: InputEvent = js!(
            return new Event( @{InputEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), InputEvent::EVENT_TYPE );
    }

    #[test]
    fn test_resource_load_event() {
        let event: ResourceLoadEvent = js!(
            return new UIEvent( @{ResourceLoadEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ResourceLoadEvent::EVENT_TYPE );
    }

    #[test]
    fn test_resource_abort_event() {
        let event: ResourceAbortEvent = js!(
            return new UIEvent( @{ResourceAbortEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ResourceAbortEvent::EVENT_TYPE );
    }

    #[test]
    fn test_ready_state_change_event() {
        let event: ReadyStateChangeEvent = js!(
            return new Event( @{ReadyStateChangeEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ReadyStateChangeEvent::EVENT_TYPE);
    }
}

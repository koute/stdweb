use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::EventTarget;
use webapi::event::{IEvent, Event, ConcreteEvent};

/// The `IFocusEvent` interface represents focus-related
/// events.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent)
pub trait IFocusEvent: IEvent {
    /// Returns the secondary target of this event, if any.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/relatedTarget)
    #[inline]
    fn related_target( &self ) -> Option< EventTarget > {
        js!(
            return @{self.as_ref()}.relatedTarget;
        ).try_into().ok()
    }
}

/// A reference to a JavaScript object which implements the [IFocusEvent](trait.IFocusEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent)
pub struct FocusRelatedEvent( Reference );

impl IEvent for FocusRelatedEvent {}
impl IFocusEvent for FocusRelatedEvent {}

reference_boilerplate! {
    FocusRelatedEvent,
    instanceof FocusEvent
    convertible to Event
}

/// The `FocusEvent` is fired when an element has received focus. The main
/// difference between this event and focusin is that only the latter bubbles.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/focus)
pub struct FocusEvent( Reference );

impl IEvent for FocusEvent {}
impl IFocusEvent for FocusEvent {}
impl ConcreteEvent for FocusEvent {
    const EVENT_TYPE: &'static str = "focus";
}

reference_boilerplate! {
    FocusEvent,
    instanceof FocusEvent
    convertible to Event
    convertible to FocusRelatedEvent
}

/// The `BlurEvent` is fired when an element has lost focus. The main difference
/// between this event and focusout is that only the latter bubbles.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/blur)
pub struct BlurEvent( Reference );

impl IEvent for BlurEvent {}
impl IFocusEvent for BlurEvent {}
impl ConcreteEvent for BlurEvent {
    const EVENT_TYPE: &'static str = "blur";
}

reference_boilerplate! {
    BlurEvent,
    instanceof FocusEvent
    convertible to Event
    convertible to FocusRelatedEvent
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_focus_event() {
        let event: FocusEvent = js!(
            return new FocusEvent( "focus" );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), "focus" );
        assert!( event.related_target().is_none() );
    }

    #[test]
    fn test_blur_event() {
        let event: BlurEvent = js!(
            return new FocusEvent( @{BlurEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), BlurEvent::EVENT_TYPE );
    }
}

use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event::{IEvent, IUiEvent, UiEvent, Event};
use webapi::touch::Touch;

/// The `ITouchEvent` interface represents events that occur due to the user
/// interacting with a touch device (such as a phone).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)
// https://w3c.github.io/touch-events/#idl-def-touchevent
pub trait ITouchEvent: IUiEvent {
    /// Returns whether the Alt key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/altKey)
    // https://w3c.github.io/touch-events/#touchevent-interface
    #[inline]
    fn alt_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.altKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the Ctrl key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/ctrlKey)
    // https://w3c.github.io/touch-events/#touchevent-interface
    #[inline]
    fn ctrl_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.ctrlKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the Meta key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/metaKey)
    // https://w3c.github.io/touch-events/#touchevent-interface
    #[inline]
    fn meta_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.metaKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the Shift key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/shiftKey)
    // https://w3c.github.io/touch-events/#touchevent-interface
    #[inline]
    fn shift_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.shiftKey;
        ).try_into().unwrap()
    }

    /// A list of Touches for every point of contact currently touching the surface.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/touches)
    // https://w3c.github.io/touch-events/#touchevent-interface
    #[inline]
    fn touches( &self ) -> Vec<Touch> {
        js!(
            return Array.from( @{self.as_ref()}.touches );
        ).try_into().unwrap()
    }

    /// A list of Touches for every point of contact that is touching the surface and started
    /// on the element that is the target of the current event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/targetTouches)
    // https://w3c.github.io/touch-events/#touchevent-interface
    #[inline]
    fn target_touches( &self ) -> Vec<Touch> {
        js!(
            return Array.from( @{self.as_ref()}.targetTouches );
        ).try_into().unwrap()
    }

    /// A list of Touches, one for each touch touch point that just became active with the current event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/changedTouches)
    // https://w3c.github.io/touch-events/#touchevent-interface
    #[inline]
    fn changed_touches( &self ) -> Vec<Touch> {
        js!(
            return Array.from( @{self.as_ref()}.changedTouches );
        ).try_into().unwrap()
    }
}


/// A reference to a JavaScript object which implements the [ITouchEvent](trait.ITouchEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)
// https://w3c.github.io/touch-events/#idl-def-touchevent
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(subclass_of(Event, UiEvent))]
pub struct TouchEvent( Reference );

impl IEvent for TouchEvent {}
impl IUiEvent for TouchEvent {}
impl ITouchEvent for TouchEvent {}

/// The `TouchMove` is fired when one or more touch points are moved along the
/// touch surface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchmove)
// https://w3c.github.io/touch-events/#event-touchmove
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(event = "touchmove")]
#[reference(subclass_of(Event, UiEvent, TouchEvent))]
pub struct TouchMove( Reference );

impl IEvent for TouchMove {}
impl IUiEvent for TouchMove {}
impl ITouchEvent for TouchMove {}

/// The `TouchLeave` event is fired when a touch point is moved off the
/// interactive area of an element.
///
/// Warning: This event was a proposal in an early version of the specification
/// and has not been implemented. Do not rely on it.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchleave)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(event = "touchleave")]
#[reference(subclass_of(Event, UiEvent, TouchEvent))]
pub struct TouchLeave( Reference );

impl IEvent for TouchLeave {}
impl IUiEvent for TouchLeave {}
impl ITouchEvent for TouchLeave {}

/// The `TouchEnter` event is fired when a touch point is moved onto the
/// interactive area of an element.
///
/// Warning: This event was a proposal in an early version of the specification
/// and has not been implemented. Do not rely on it.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchenter)
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(event = "touchenter")]
#[reference(subclass_of(Event, UiEvent, TouchEvent))]
pub struct TouchEnter( Reference );

impl IEvent for TouchEnter {}
impl IUiEvent for TouchEnter {}
impl ITouchEvent for TouchEnter {}

/// The `TouchEnd` event is fired when one or more touch points are removed
/// from the touch surface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchend)
// https://w3c.github.io/touch-events/#event-touchend
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(event = "touchend")]
#[reference(subclass_of(Event, UiEvent, TouchEvent))]
pub struct TouchEnd( Reference );

impl IEvent for TouchEnd {}
impl IUiEvent for TouchEnd {}
impl ITouchEvent for TouchEnd {}

/// The `TouchCancel` event is fired when one or more touch points have been
/// disrupted in an implementation-specific manner (for example, too many touch
/// points are created).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchcancel)
// https://w3c.github.io/touch-events/#event-touchcancel
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(event = "touchcancel")]
#[reference(subclass_of(Event, UiEvent, TouchEvent))]
pub struct TouchCancel( Reference );

impl IEvent for TouchCancel {}
impl IUiEvent for TouchCancel {}
impl ITouchEvent for TouchCancel {}

/// The `TouchStart` event is fired when one or more touch points are placed
/// on the touch surface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchstart)
// https://w3c.github.io/touch-events/#event-touchstart
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(event = "touchstart")]
#[reference(subclass_of(Event, UiEvent, TouchEvent))]
pub struct TouchStart( Reference );

impl IEvent for TouchStart {}
impl IUiEvent for TouchStart {}
impl ITouchEvent for TouchStart {}


#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::event::ConcreteEvent;

    #[test]
    fn test_touch_event() {
        let event: TouchEvent = js!(
            return new TouchEvent(
                @{TouchMove::EVENT_TYPE},
                {
                    touches: [],
                    targetTouches: [],
                    changedTouches: [],
                    ctrlKey: true,
                    shiftKey: true,
                    altKey: true,
                    metaKey: true
                }
            );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), TouchMove::EVENT_TYPE );
        assert!( event.ctrl_key() );
        assert!( event.alt_key() );
        assert!( event.shift_key() );
        assert!( event.meta_key() );
        assert_eq!( event.touches(), vec![] );
        assert_eq!( event.target_touches(), vec![] );
        assert_eq!( event.changed_touches(), vec![] );
    }

    #[test]
    fn test_touch_move_event() {
        let event: TouchMove = js!(
            return new TouchEvent( @{TouchMove::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), TouchMove::EVENT_TYPE );
    }

    #[test]
    fn test_touch_leave_event() {
        let event: TouchLeave = js!(
            return new TouchEvent( @{TouchLeave::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), TouchLeave::EVENT_TYPE );
    }


    #[test]
    fn test_touch_enter_event() {
        let event: TouchEnter = js!(
            return new TouchEvent( @{TouchEnter::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), TouchEnter::EVENT_TYPE );
    }

    #[test]
    fn test_touch_end_event() {
        let event: TouchEnd = js!(
            return new TouchEvent( @{TouchEnd::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), TouchEnd::EVENT_TYPE );
    }

    #[test]
    fn test_touch_cancel_event() {
        let event: TouchCancel = js!(
            return new TouchEvent( @{TouchCancel::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), TouchCancel::EVENT_TYPE );
    }

    #[test]
    fn test_touch_start_event() {
        let event: TouchStart = js!(
            return new TouchEvent( @{TouchStart::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), TouchStart::EVENT_TYPE );
    }
}

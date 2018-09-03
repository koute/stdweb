use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event::{IEvent, IUiEvent, UiEvent, Event};
use webapi::events::mouse::{IMouseEvent, MouseEvent};

#[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
use webapi::events::mouse::MouseButton;

/// The `IPointerEvent` interface represents the state of a DOM event produced by a pointer
/// such as the geometry of the contact point, the device type that generated the event, the
/// amount of pressure that was applied on the contact surface, etc.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent)
// https://w3c.github.io/pointerevents/#pointerevent-interface
// https://w3c.github.io/pointerevents/extension.html#extensions-to-the-pointerevent-interface
pub trait IPointerEvent: IMouseEvent {
    /// Returns a unique identifier for the pointer causing the event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-pointerid
    #[inline]
    fn pointer_id( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.pointerId;
        ).try_into().unwrap()
    }

    /// Returns the width, in CSS pixels, of the contact geometry of the pointer.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-width
    #[inline]
    fn width( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.width;
        ).try_into().unwrap()
    }

    /// Returns the height, in CSS pixels, of the contact geometry of the pointer.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-height
    #[inline]
    fn height( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.height;
        ).try_into().unwrap()
    }

    /// Returns the normalized pressure of the pointer in the range [0, 1]
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-pressure
    //TODO: This should return a f32, but try_into() didn't support it at the time of writing
    #[inline]
    fn pressure( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.pressure;
        ).try_into().unwrap()
    }

    /// Returns the normalized tangential pressure of the pointer in the range [-1, 1], where 0 is
    /// the hardware's neutral position
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-tangentialpressure
    //TODO: This should return a f32, but try_into() didn't support it at the time of writing
    #[inline]
    fn tangential_pressure( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.tangentialPressure;
        ).try_into().unwrap()
    }

    /// Returns the angle, in the range of [-90, 90] degrees, between the Y-Z plane and the plane
    /// containing the transducer (e.g. pen stylus) and the Y axis.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-tiltx
    #[inline]
    fn tilt_x( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.tiltX;
        ).try_into().unwrap()
    }

    /// Returns the angle, in the range of [-90, 90] degrees, between the X-Z plane and the plane
    /// containing the transducer (e.g. pen stylus) and the X axis.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-tilty
    #[inline]
    fn tilt_y( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.tiltY;
        ).try_into().unwrap()
    }

    /// Returns the clockwise rotation, in the range of [0, 359] degrees, of
    /// the transducer (e.g. pen stylus) around it's own major axis
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-twist
    #[inline]
    fn twist( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.twist;
        ).try_into().unwrap()
    }

    /// Indicates the device type that caused the event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-pointertype
    #[inline]
    fn pointer_type( &self ) -> String {
        js!(
            return @{self.as_ref()}.pointerType;
        ).try_into().unwrap()
    }

    /// Indicates if the pointer represents the primary pointer of this pointer type
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)
    // https://w3c.github.io/pointerevents/#dom-pointerevent-isprimary
    #[inline]
    fn is_primary( &self ) -> bool {
        js!(
            return @{self.as_ref()}.isPrimary;
        ).try_into().unwrap()
    }

    /// Indicates the mouse button that fired this event. A None value indicates no change since the last PointerEvent.
    ///
    /// This function is feature-gated because it may be merged into `MouseEvent::button()`
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    // https://w3c.github.io/pointerevents/#the-button-property
    #[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
    fn button_pointer( &self ) -> Option<MouseButton> {
        match js!(
            return @{self.as_ref()}.button;
        ).try_into().unwrap() {
            -1 => None,
            0 => Some(MouseButton::Left),
            1 => Some(MouseButton::Wheel),
            2 => Some(MouseButton::Right),
            3 => Some(MouseButton::Button4),
            4 => Some(MouseButton::Button5),
            _ => unreachable!("Unexpected PointerEvent.button value"),
        }
    }
}

/// A reference to a JavaScript object which implements the [IPointerEvent](trait.IPointerEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent)
// https://w3c.github.io/pointerevents/#pointerevent-interface
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct PointerEvent( Reference );

impl IEvent for PointerEvent {}
impl IUiEvent for PointerEvent {}
impl IMouseEvent for PointerEvent {}
impl IPointerEvent for PointerEvent {}

/// The `PointerOverEvent` is fired when a pointing device is moved into
/// a element's hit test boundaries.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerover)
// https://w3c.github.io/pointerevents/#the-pointerover-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointerover")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerOverEvent( Reference );

impl IEvent for PointerOverEvent {}
impl IUiEvent for PointerOverEvent {}
impl IMouseEvent for PointerOverEvent {}
impl IPointerEvent for PointerOverEvent {}

/// The `PointerEnterEvent` is fired when a pointing device is moved into
/// the hit test boundaries of an element or its descendants. This event does not bubble.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerenter)
// https://w3c.github.io/pointerevents/#the-pointerenter-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointerenter")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerEnterEvent( Reference );

impl IEvent for PointerEnterEvent {}
impl IUiEvent for PointerEnterEvent {}
impl IMouseEvent for PointerEnterEvent {}
impl IPointerEvent for PointerEnterEvent {}

/// The `PointerDownEvent` is fired when a pointer becomes active
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerdown)
// https://w3c.github.io/pointerevents/#the-pointerdown-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointerdown")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerDownEvent( Reference );

impl IEvent for PointerDownEvent {}
impl IUiEvent for PointerDownEvent {}
impl IMouseEvent for PointerDownEvent {}
impl IPointerEvent for PointerDownEvent {}

/// The `PointerMoveEvent` is fired when a pointer changes coordinates
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointermove)
// https://w3c.github.io/pointerevents/#the-pointermove-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointermove")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerMoveEvent( Reference );

impl IEvent for PointerMoveEvent {}
impl IUiEvent for PointerMoveEvent {}
impl IMouseEvent for PointerMoveEvent {}
impl IPointerEvent for PointerMoveEvent {}

impl PointerMoveEvent
{
    /// Returns the sequence of all `PointerEvent` instances that were coalesced into the dispatched `PointerMoveEvent`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)
    // https://w3c.github.io/pointerevents/extension.html#dom-pointerevent-getcoalescedevents
    #[inline]
    pub fn get_coalesced_events( &self ) -> Vec<PointerEvent> {
        js!(
            return @{self.as_ref()}.getCoalescedEvents();
        ).try_into().unwrap()
    }
}

/// The `PointerUpEvent` is fired when a pointer is no longer active
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerup)
// https://w3c.github.io/pointerevents/#the-pointerup-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointerup")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerUpEvent( Reference );

impl IEvent for PointerUpEvent {}
impl IUiEvent for PointerUpEvent {}
impl IMouseEvent for PointerUpEvent {}
impl IPointerEvent for PointerUpEvent {}

/// The `PointerCancelEvent` is fired when a pointer will no longer produce events
/// (for example the device is deactivated), or if the pointer starts a gesture after a pointerdown event
/// (for example panning, zooming, or drag and drop)
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointercancel)
// https://w3c.github.io/pointerevents/#the-pointercancel-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointercancel")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerCancelEvent( Reference );

impl IEvent for PointerCancelEvent {}
impl IUiEvent for PointerCancelEvent {}
impl IMouseEvent for PointerCancelEvent {}
impl IPointerEvent for PointerCancelEvent {}

/// The `PointerOutEvent` is fired when the pointer moves out of the hit test boundaries of an element.
/// This can include when a finger leaves a touch screen or a pen leaves the detectable hover range.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerout)
// https://w3c.github.io/pointerevents/#the-pointerout-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointerout")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerOutEvent( Reference );

impl IEvent for PointerOutEvent {}
impl IUiEvent for PointerOutEvent {}
impl IMouseEvent for PointerOutEvent {}
impl IPointerEvent for PointerOutEvent {}

/// The `PointerLeaveEvent` is fired when the pointer moves out of the hit test boundaries
/// of an element and it's descendants. This can include when a finger leaves a touch screen
/// or a pen leaves the detectable hover range. This event does not bubble.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerleave)
// https://w3c.github.io/pointerevents/#the-pointerleave-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "pointerleave")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct PointerLeaveEvent( Reference );

impl IEvent for PointerLeaveEvent {}
impl IUiEvent for PointerLeaveEvent {}
impl IMouseEvent for PointerLeaveEvent {}
impl IPointerEvent for PointerLeaveEvent {}

/// The `GotPointerCaptureEvent` fires when an element receives pointer capture
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/gotpointercapture)
// https://w3c.github.io/pointerevents/#the-gotpointercapture-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "gotpointercapture")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct GotPointerCaptureEvent( Reference );

impl IEvent for GotPointerCaptureEvent {}
impl IUiEvent for GotPointerCaptureEvent {}
impl IMouseEvent for GotPointerCaptureEvent {}
impl IPointerEvent for GotPointerCaptureEvent {}

/// The `LostPointerCaptureEvent` fires when an element loses pointer capture
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/lostpointercapture)
// https://w3c.github.io/pointerevents/#the-lostpointercapture-event
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "PointerEvent")]
#[reference(event = "lostpointercapture")]
#[reference(subclass_of(Event, UiEvent, MouseEvent, PointerEvent))]
pub struct LostPointerCaptureEvent( Reference );

impl IEvent for LostPointerCaptureEvent {}
impl IUiEvent for LostPointerCaptureEvent {}
impl IMouseEvent for LostPointerCaptureEvent {}
impl IPointerEvent for LostPointerCaptureEvent {}

/// The `PointerLockChangeEvent` fires when the pointer is locked or unlocked
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerlockchange)
// http://www.w3.org/TR/pointerlock/#pointerlockchange-and-pointerlockerror-events
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(event = "pointerlockchange")]
#[reference(subclass_of(Event))]
pub struct PointerLockChangeEvent( Reference );

impl IEvent for PointerLockChangeEvent {}

/// The `PointerLockErrorEvent` fires when an error occurs locking a pointer
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/pointerlockerror)
// http://www.w3.org/TR/pointerlock/#pointerlockchange-and-pointerlockerror-events
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Event")]
#[reference(event = "pointerlockerror")]
#[reference(subclass_of(Event))]
pub struct PointerLockErrorEvent( Reference );

impl IEvent for PointerLockErrorEvent {}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::event::ConcreteEvent;

    #[test]
    fn test_pointer_event() {
        let event: PointerMoveEvent = js!(
            return new PointerEvent(
                @{PointerMoveEvent::EVENT_TYPE},
                {
                    altKey: false,
                    button: -1,
                    buttons: 6,
                    clientX: 3,
                    clientY: 4,
                    ctrlKey: true,
                    metaKey: false,
                    screenX: 1,
                    screenY: 2,
                    shiftKey: true,

                    pointerId: 5,
                    width: 8.2,
                    height: 6.1,
                    pressure: 0.49,
                    tangentialPressure: -0.2,
                    tiltX: 20,
                    tiltY: -42,
                    twist: 215,
                    pointerType: "stdweb-hand-wave",
                    isPrimary: false,
                }
            );
        ).try_into().unwrap();

        assert_eq!( event.event_type(), PointerMoveEvent::EVENT_TYPE );

        assert_eq!( event.pointer_id(), 5 );
        assert_eq!( event.width(), 8.2 );
        assert_eq!( event.height(), 6.1 );
        assert!( ( event.pressure() - 0.49 ).abs() < 0.00000001 );
        assert!( ( event.tangential_pressure() - -0.2 ).abs() < 0.00000001 );
        assert_eq!( event.tilt_x(), 20 );
        assert_eq!( event.tilt_y(), -42 );
        assert_eq!( event.twist(), 215 );
        assert_eq!( event.pointer_type(), "stdweb-hand-wave" );
        assert_eq!( event.is_primary(), false );

        assert_eq!( event.get_coalesced_events().len(), 0 );
    }

    #[test]
    fn test_pointer_over_event() {
        let event: PointerOverEvent = js!(
            return new PointerEvent( @{PointerOverEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerOverEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_enter_event() {
        let event: PointerEnterEvent = js!(
            return new PointerEvent( @{PointerEnterEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerEnterEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_down_event() {
        let event: PointerDownEvent = js!(
            return new PointerEvent( @{PointerDownEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerDownEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_move_event() {
        let event: PointerMoveEvent = js!(
            return new PointerEvent( @{PointerMoveEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerMoveEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_up_event() {
        let event: PointerUpEvent = js!(
            return new PointerEvent( @{PointerUpEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerUpEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_cancel_event() {
        let event: PointerCancelEvent = js!(
            return new PointerEvent( @{PointerCancelEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerCancelEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_out_event() {
        let event: PointerOutEvent = js!(
            return new PointerEvent( @{PointerOutEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerOutEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_leave_event() {
        let event: PointerLeaveEvent = js!(
            return new PointerEvent( @{PointerLeaveEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerLeaveEvent::EVENT_TYPE );
    }

    #[test]
    fn test_got_pointer_capture_event() {
        let event: GotPointerCaptureEvent = js!(
            return new PointerEvent( @{GotPointerCaptureEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), GotPointerCaptureEvent::EVENT_TYPE );
    }

    #[test]
    fn test_lost_pointer_capture_event() {
        let event: LostPointerCaptureEvent = js!(
            return new PointerEvent( @{LostPointerCaptureEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), LostPointerCaptureEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_lock_change_event() {
        let event: PointerLockChangeEvent = js!(
            return new Event( @{PointerLockChangeEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerLockChangeEvent::EVENT_TYPE );
    }

    #[test]
    fn test_pointer_lock_error_event() {
        let event: PointerLockErrorEvent = js!(
            return new Event( @{PointerLockErrorEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), PointerLockErrorEvent::EVENT_TYPE );
    }
}

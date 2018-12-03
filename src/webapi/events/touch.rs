use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event::{IEvent, IUiEvent, UiEvent, Event};
use webapi::touch::Touch;

/// The `ITouchEvent` interface represents events that occur due to the user
/// interacting with a touch device (such as a phone).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)
// https://w3c.github.io/uievents/#touchevent
pub trait ITouchEvent: IUiEvent {
    /// Returns whether the Alt key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/altKey)
    // https://w3c.github.io/uievents/#ref-for-dom-touchevent-altkey-1
    #[inline]
    fn alt_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.altKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the Ctrl key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/ctrlKey)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-ctrlkey-1
    #[inline]
    fn ctrl_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.ctrlKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the Meta key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/metaKey)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-metakey-1
    #[inline]
    fn meta_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.metaKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the Shift key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/shiftKey)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-shiftkey-1
    #[inline]
    fn shift_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.shiftKey;
        ).try_into().unwrap()
    }

    /// A list of Touches for every point of contact currently touching the surface.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/touches)
    #[inline]
    fn touches( &self ) -> Vec<Touch> {
        js!(
            return @{self.as_ref()}.touches;
        ).try_into().unwrap()
    }

    /// A list of Touches for every point of contact that is touching the surface and started
    /// on the element that is the target of the current event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/targetTouches)
    #[inline]
    fn target_touches( &self ) -> Vec<Touch> {
        js!(
            return @{self.as_ref()}.targetTouches;
        ).try_into().unwrap()
    }

    /// A list of Touches, one for each touch touch point that just became active with the current event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/changedTouches)
    #[inline]
    fn changed_touches( &self ) -> Vec<Touch> {
        js!(
            return @{self.as_ref()}.changedTouches;
        ).try_into().unwrap()
    }
}


/// A reference to a JavaScript object which implements the [ITouchEvent](trait.ITouchEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)
// https://w3c.github.io/uievents/#mouseevent
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
// https://w3c.github.io/uievents/#event-type-mousedown
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
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchleave)
// https://w3c.github.io/uievents/#event-type-mousemove
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
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/touchenter)
// https://w3c.github.io/uievents/#event-type-mouseover
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
// https://w3c.github.io/uievents/#event-type-mouseout
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
// https://w3c.github.io/uievents/#event-type-touchcancel
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
// https://w3c.github.io/uievents/#event-type-touchstart
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "TouchEvent")]
#[reference(event = "touchstart")]
#[reference(subclass_of(Event, UiEvent, TouchEvent))]
pub struct TouchStart( Reference );

impl IEvent for TouchStart {}
impl IUiEvent for TouchStart {}
impl ITouchEvent for TouchStart {}


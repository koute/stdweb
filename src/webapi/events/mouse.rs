use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::EventTarget;
use webapi::event::{IEvent, IUiEvent, UiEvent, Event, ConcreteEvent};
use webapi::events::keyboard::{ModifierKey, get_event_modifier_state};

/// The `IMouseEvent` interface represents events that occur due to the user
/// interacting with a pointing device (such as a mouse).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent)
pub trait IMouseEvent: IUiEvent {
    /// Returns whether the Alt key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/altKey)
    #[inline]
    fn alt_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.altKey;
        ).try_into().unwrap()
    }

    /// Indicates the mouse button that fired this event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    fn button( &self ) -> MouseButton {
        match js!(
            return @{self.as_ref()}.button;
        ).try_into().unwrap() {
            0 => MouseButton::Left,
            1 => MouseButton::Wheel,
            2 => MouseButton::Right,
            3 => MouseButton::Button4,
            4 => MouseButton::Button5,
            _ => unreachable!("Unexpected MouseEvent.button value"),
        }
    }

    /// Indicates which mouse buttons were down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons)
    fn buttons( &self ) -> MouseButtonsState {
        MouseButtonsState(
            js!(
                return @{self.as_ref()}.buttons;
            ).try_into().unwrap()
        )
    }

    /// Returns the X position in the application's client area where this event occured.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)
    #[inline]
    fn client_x( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.clientX;
        ).try_into().unwrap()
    }

    /// Returns the Y position in the application's client area where this event occured.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)
    #[inline]
    fn client_y( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.clientY;
        ).try_into().unwrap()
    }

    /// Indicates whether the Ctrl key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/ctrlKey)
    #[inline]
    fn ctrl_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.ctrlKey;
        ).try_into().unwrap()
    }

    /// Returns the current state of the specified modifier key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/getModifierState)
    #[inline]
    fn get_modifier_state( &self, key: ModifierKey ) -> bool {
        get_event_modifier_state( self, key )
    }

    /// Indicates whether the Meta key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/metaKey)
    #[inline]
    fn meta_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.metaKey;
        ).try_into().unwrap()
    }

    /// Returns the change in X coordinate of the pointer between this event and the previous
    /// MouseMove event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)
    #[inline]
    fn movement_x( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.movementX;
        ).try_into().unwrap()
    }

    /// Returns the change in Y coordinate of the pointer between this event and the previous
    /// MouseMove event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)
    #[inline]
    fn movement_y( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.movementY;
        ).try_into().unwrap()
    }

    /// Returns the ID of the hit region affected by the event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/region)
    #[inline]
    fn region( &self ) -> Option< String > {
        js!(
            return @{self.as_ref()}.region;
        ).try_into().ok()
    }

    /// Returns the secondary target of this event, if any.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/relatedTarget)
    #[inline]
    fn related_target( &self ) -> Option< EventTarget > {
        js!(
            return @{self.as_ref()}.relatedTarget;
        ).try_into().ok()
    }

     /// Returns the X position of the pointer in screen coordinates.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)
    #[inline]
    fn screen_x( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.screenX;
        ).try_into().unwrap()
    }

    /// Returns the Y position of the pointer in screen coordinates.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)
    #[inline]
    fn screen_y( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.screenY;
        ).try_into().unwrap()
    }

    /// Indicates whether the Shift key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/shiftKey)
    #[inline]
    fn shift_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.shiftKey;
        ).try_into().unwrap()
    }
}

/// Represents buttons on a mouse during mouse events.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The mouse wheel/middle mouse button.
    Wheel,
    /// The right mouse button.
    Right,
    /// The fourth mouse button (browser back).
    Button4,
    /// The fifth mouse button (browser forward).
    Button5,
}

/// Represents the state of mouse buttons in a `MouseEvent`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MouseButtonsState(u8);

impl MouseButtonsState {
    pub fn is_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.0 & 0b1 != 0,
            MouseButton::Right => self.0 & 0b10 != 0,
            MouseButton::Wheel => self.0 & 0b100 != 0,
            MouseButton::Button4 => self.0 & 0b1000 != 0,
            MouseButton::Button5 => self.0 & 0b1_0000 != 0,
        }
    }
}

/// A reference to a JavaScript object which implements the [IMouseEvent](trait.IMouseEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent)
pub struct MouseEvent( Reference );

impl IEvent for MouseEvent {}
impl IUiEvent for MouseEvent {}
impl IMouseEvent for MouseEvent {}

reference_boilerplate! {
    MouseEvent,
    instanceof MouseEvent
    convertible to Event
    convertible to UiEvent
}

/// The `ClickEvent` is fired when a pointing device button (usually a
/// mouse's primary button) is pressed and released on a single element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/click)
pub struct ClickEvent( Reference );

impl IEvent for ClickEvent {}
impl IUiEvent for ClickEvent {}
impl IMouseEvent for ClickEvent {}
impl ConcreteEvent for ClickEvent {
    const EVENT_TYPE: &'static str = "click";
}

reference_boilerplate! {
    ClickEvent,
    instanceof MouseEvent
    convertible to Event
    convertible to UiEvent
    convertible to MouseEvent
}

/// The `DoubleClickEvent` is fired when a pointing device button
/// (usually a mouse's primary button) is clicked twice on a single
/// element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dblclick)
pub struct DoubleClickEvent( Reference );

impl IEvent for DoubleClickEvent {}
impl IUiEvent for DoubleClickEvent {}
impl IMouseEvent for DoubleClickEvent {}
impl ConcreteEvent for DoubleClickEvent {
    const EVENT_TYPE: &'static str = "dblclick";
}

reference_boilerplate! {
    DoubleClickEvent,
    instanceof MouseEvent
    convertible to Event
    convertible to UiEvent
    convertible to MouseEvent
}

/// The `MouseDownEvent` is fired when a pointing device button is pressed on
/// an element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mousedown)
pub struct MouseDownEvent( Reference );

impl IEvent for MouseDownEvent {}
impl IUiEvent for MouseDownEvent {}
impl IMouseEvent for MouseDownEvent {}
impl ConcreteEvent for MouseDownEvent {
    const EVENT_TYPE: &'static str = "mousedown";
}

reference_boilerplate! {
    MouseDownEvent,
    instanceof MouseEvent
    convertible to Event
    convertible to UiEvent
    convertible to MouseEvent
}

/// The `MouseUpEvent` is fired when a pointing device button is released
/// over an element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mouseup)
pub struct MouseUpEvent( Reference );

impl IEvent for MouseUpEvent {}
impl IUiEvent for MouseUpEvent {}
impl IMouseEvent for MouseUpEvent {}
impl ConcreteEvent for MouseUpEvent {
    const EVENT_TYPE: &'static str = "mouseup";
}

reference_boilerplate! {
    MouseUpEvent,
    instanceof MouseEvent
    convertible to Event
    convertible to UiEvent
    convertible to MouseEvent
}

/// The `MouseMoveEvent` is fired when a pointing device (usually a mouse)
/// is moved while over an element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mousemove)
pub struct MouseMoveEvent( Reference );

impl IEvent for MouseMoveEvent {}
impl IUiEvent for MouseMoveEvent {}
impl IMouseEvent for MouseMoveEvent {}
impl ConcreteEvent for MouseMoveEvent {
    const EVENT_TYPE: &'static str = "mousemove";
}

reference_boilerplate! {
    MouseMoveEvent,
    instanceof MouseEvent
    convertible to Event
    convertible to UiEvent
    convertible to MouseEvent
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_event() {
        let event: MouseEvent = js!(
            return new MouseEvent(
                @{ClickEvent::EVENT_TYPE},
                {
                    altKey: false,
                    button: 2,
                    buttons: 6,
                    clientX: 3.0,
                    clientY: 4.0,
                    ctrlKey: true,
                    metaKey: false,
                    screenX: 1.0,
                    screenY: 2.0,
                    shiftKey: true
                }
            );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ClickEvent::EVENT_TYPE );
        assert_eq!( event.alt_key(), false );
        assert_eq!( event.button(), MouseButton::Right );
        assert!( !event.buttons().is_down( MouseButton::Left ) );
        assert!( event.buttons().is_down( MouseButton::Right ) );
        assert!( event.buttons().is_down( MouseButton::Wheel ) );
        assert_eq!( event.client_x(), 3.0 );
        assert_eq!( event.client_y(), 4.0 );
        assert!( event.ctrl_key() );
        assert!( !event.get_modifier_state( ModifierKey::Alt ) );
        assert!( event.get_modifier_state( ModifierKey::Ctrl ) );
        assert!( event.get_modifier_state( ModifierKey::Shift ) );
        assert!( !event.meta_key() );
        assert_eq!( event.movement_x(), 0.0 );
        assert_eq!( event.movement_y(), 0.0 );
        assert!( event.region().is_none() );
        assert!( event.related_target().is_none() );
        assert_eq!( event.screen_x(), 1.0 );
        assert_eq!( event.screen_y(), 2.0 );
        assert!( event.shift_key() );
    }

    #[test]
    fn test_click_event() {
        let event: ClickEvent = js!(
            return new MouseEvent( @{ClickEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ClickEvent::EVENT_TYPE );
    }

    #[test]
    fn test_double_click_event() {
        let event: DoubleClickEvent = js!(
            return new MouseEvent( @{DoubleClickEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), DoubleClickEvent::EVENT_TYPE );
    }

    #[test]
    fn test_mouse_down_event() {
        let event: MouseDownEvent = js!(
            return new MouseEvent( @{MouseDownEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), MouseDownEvent::EVENT_TYPE );
    }

    #[test]
    fn test_mouse_up_event() {
        let event: MouseUpEvent = js!(
            return new MouseEvent( @{MouseUpEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), MouseUpEvent::EVENT_TYPE );
    }

    #[test]
    fn test_mouse_move_event() {
        let event: MouseMoveEvent = js!(
            return new MouseEvent( @{MouseMoveEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), MouseMoveEvent::EVENT_TYPE );
    }
}

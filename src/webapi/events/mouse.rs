use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::EventTarget;
use webapi::event::{IEvent, IUiEvent, UiEvent, Event};
use webapi::events::keyboard::{ModifierKey, get_event_modifier_state};

/// The `IMouseEvent` interface represents events that occur due to the user
/// interacting with a pointing device (such as a mouse).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent)
// https://w3c.github.io/uievents/#mouseevent
pub trait IMouseEvent: IUiEvent {
    /// Returns whether the Alt key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/altKey)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-altkey-1
    #[inline]
    fn alt_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.altKey;
        ).try_into().unwrap()
    }

    /// Indicates the mouse button that fired this event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-button-1
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
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-buttons-1
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
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-clientx-2
    #[inline]
    fn client_x( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.clientX;
        ).try_into().unwrap()
    }

    /// Returns the Y position in the application's client area where this event occured.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-clienty-2
    #[inline]
    fn client_y( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.clientY;
        ).try_into().unwrap()
    }

    /// Returns the X position on the target element where this event occured.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-mouseevent-offsetx
    #[inline]
    fn offset_x( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.offsetX;
        ).try_into().unwrap()
    }

    /// Returns the Y position on the target element where this event occured.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-mouseevent-offsety
    #[inline]
    fn offset_y( &self ) -> f64 {
        js!(
            return @{self.as_ref()}.offsetY;
        ).try_into().unwrap()
    }

    /// Indicates whether the Ctrl key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/ctrlKey)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-ctrlkey-1
    #[inline]
    fn ctrl_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.ctrlKey;
        ).try_into().unwrap()
    }

    /// Returns the current state of the specified modifier key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/getModifierState)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-getmodifierstate-2
    #[inline]
    fn get_modifier_state( &self, key: ModifierKey ) -> bool {
        get_event_modifier_state( self, key )
    }

    /// Indicates whether the Meta key was down when this event fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/metaKey)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-metakey-1
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
    // https://w3c.github.io/pointerlock/#extensions-to-the-mouseevent-interface
    #[inline]
    fn movement_x( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.movementX;
        ).try_into().unwrap()
    }

    /// Returns the change in Y coordinate of the pointer between this event and the previous
    /// MouseMove event.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)
    // https://w3c.github.io/pointerlock/#extensions-to-the-mouseevent-interface
    #[inline]
    fn movement_y( &self ) -> i32 {
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
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-relatedtarget-1
    #[inline]
    fn related_target( &self ) -> Option< EventTarget > {
        js!(
            return @{self.as_ref()}.relatedTarget;
        ).try_into().ok()
    }

    /// Returns the X position of the pointer in screen coordinates.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-screenx-1
    #[inline]
    fn screen_x( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.screenX;
        ).try_into().unwrap()
    }

    /// Returns the Y position of the pointer in screen coordinates.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-screeny-1
    #[inline]
    fn screen_y( &self ) -> i32 {
        js!(
            return @{self.as_ref()}.screenY;
        ).try_into().unwrap()
    }

    /// Indicates whether the Shift key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/shiftKey)
    // https://w3c.github.io/uievents/#ref-for-dom-mouseevent-shiftkey-1
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
    // /// The sixth mouse button, or the Pen Eraser button
    //TODO: Eraser,
}

/// Represents the state of mouse buttons in a `MouseEvent`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MouseButtonsState(u8);

impl MouseButtonsState {
    /// Check if a [MouseButton](enum.MouseButton.html) is currently pressed
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
// https://w3c.github.io/uievents/#mouseevent
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(subclass_of(Event, UiEvent))]
pub struct MouseEvent( Reference );

impl IEvent for MouseEvent {}
impl IUiEvent for MouseEvent {}
impl IMouseEvent for MouseEvent {}

/// The `ClickEvent` is fired when a pointing device button (usually a
/// mouse's primary button) is pressed and released on a single element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/click)
// https://w3c.github.io/uievents/#event-type-click
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "click")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct ClickEvent( Reference );

impl IEvent for ClickEvent {}
impl IUiEvent for ClickEvent {}
impl IMouseEvent for ClickEvent {}

/// The `AuxClickEvent` event is fired when a non-primary pointing device button
/// (e.g. any non-left mouse button) has been pressed and released on an element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/auxclick)
// https://w3c.github.io/uievents/#event-type-auxclick
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "auxclick")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct AuxClickEvent( Reference );

impl IEvent for AuxClickEvent {}
impl IUiEvent for AuxClickEvent {}
impl IMouseEvent for AuxClickEvent {}

/// The `ContextMenuEvent` event is fired when the right button of the mouse is clicked
/// (before the context menu is displayed), or when the context menu key is pressed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/contextmenu)
// https://html.spec.whatwg.org/#event-contextmenu
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "contextmenu")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct ContextMenuEvent( Reference );

impl IEvent for ContextMenuEvent {}
impl IUiEvent for ContextMenuEvent {}
impl IMouseEvent for ContextMenuEvent {}

/// The `DoubleClickEvent` is fired when a pointing device button
/// (usually a mouse's primary button) is clicked twice on a single
/// element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dblclick)
// https://w3c.github.io/uievents/#event-type-dblclick
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "dblclick")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct DoubleClickEvent( Reference );

impl IEvent for DoubleClickEvent {}
impl IUiEvent for DoubleClickEvent {}
impl IMouseEvent for DoubleClickEvent {}

/// The `MouseDownEvent` is fired when a pointing device button is pressed on
/// an element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mousedown)
// https://w3c.github.io/uievents/#event-type-mousedown
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "mousedown")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseDownEvent( Reference );

impl IEvent for MouseDownEvent {}
impl IUiEvent for MouseDownEvent {}
impl IMouseEvent for MouseDownEvent {}

/// The `MouseUpEvent` is fired when a pointing device button is released
/// over an element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mouseup)
// https://w3c.github.io/uievents/#event-type-mouseup
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "mouseup")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseUpEvent( Reference );

impl IEvent for MouseUpEvent {}
impl IUiEvent for MouseUpEvent {}
impl IMouseEvent for MouseUpEvent {}

/// The `MouseMoveEvent` is fired when a pointing device (usually a mouse)
/// is moved while over an element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mousemove)
// https://w3c.github.io/uievents/#event-type-mousemove
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "mousemove")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseMoveEvent( Reference );

impl IEvent for MouseMoveEvent {}
impl IUiEvent for MouseMoveEvent {}
impl IMouseEvent for MouseMoveEvent {}

/// The `MouseOverEvent` is fired when a pointing device (usually a mouse)
/// is moved onto the element that has the listener attached or onto one of its children.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mouseover)
// https://w3c.github.io/uievents/#event-type-mouseover
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "mouseover")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseOverEvent( Reference );

impl IEvent for MouseOverEvent {}
impl IUiEvent for MouseOverEvent {}
impl IMouseEvent for MouseOverEvent {}

/// The `MouseOutEvent` is fired when a pointing device (usually a mouse)
/// is moved off the element that has the listener attached or off one of its children.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mouseout)
// https://w3c.github.io/uievents/#event-type-mouseout
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "mouseout")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseOutEvent( Reference );

impl IEvent for MouseOutEvent {}
impl IUiEvent for MouseOutEvent {}
impl IMouseEvent for MouseOutEvent {}

/// The `MouseEnterEvent` is fired when a pointing device (usually a mouse)
/// is moved over the element that has the listener attached.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mouseenter)
// https://w3c.github.io/uievents/#event-type-mouseenter
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "mouseenter")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseEnterEvent( Reference );

impl IEvent for MouseEnterEvent {}
impl IUiEvent for MouseEnterEvent {}
impl IMouseEvent for MouseEnterEvent {}

/// The `MouseLeaveEvent` is fired when a pointing device (usually a mouse)
/// is moved out of an element that has the listener attached to it.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/mouseleave)
// https://w3c.github.io/uievents/#event-type-mouseleave
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "mouseleave")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseLeaveEvent( Reference );

impl IEvent for MouseLeaveEvent {}
impl IUiEvent for MouseLeaveEvent {}
impl IMouseEvent for MouseLeaveEvent {}

/// The `MouseWheelEvent` is fired when a pointing device's wheel button (usually a mousewheel)
/// is rotated over the element that has the listener attached.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/wheel)
// https://w3c.github.io/uievents/#event-type-wheel
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MouseEvent")]
#[reference(event = "wheel")]
#[reference(subclass_of(Event, UiEvent, MouseEvent))]
pub struct MouseWheelEvent( Reference );

impl IEvent for MouseWheelEvent {}
impl IUiEvent for MouseWheelEvent {}
impl IMouseEvent for MouseWheelEvent {}

impl MouseWheelEvent {
    /// The change in X of the wheel
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaX)
    // https://w3c.github.io/uievents/#dom-wheelevent-deltax
    pub fn delta_x(&self) -> f64 {
        js! (
            return @{self}.deltaX;
        ).try_into().unwrap()
    }

    /// The change in Y of the wheel
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaY)
    // https://w3c.github.io/uievents/#dom-wheelevent-deltay
    pub fn delta_y(&self) -> f64 {
        js! (
            return @{self}.deltaY;
        ).try_into().unwrap()
    }

    /// The change in Z of the wheel
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaZ)
    // https://w3c.github.io/uievents/#dom-wheelevent-deltaz
    pub fn delta_z(&self) -> f64 {
        js! (
            return @{self}.deltaZ;
        ).try_into().unwrap()
    }

    /// The unit of measure of change
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)
    // https://w3c.github.io/uievents/#dom-wheelevent-deltamode
    pub fn delta_mode(&self) -> MouseWheelDeltaMode {
        let mode: u32 = js! (
            return @{self}.deltaMode;
        ).try_into().unwrap();
        match mode {
            0 => MouseWheelDeltaMode::Pixel,
            1 => MouseWheelDeltaMode::Line,
            2 => MouseWheelDeltaMode::Page,
            _ => unreachable!()
        }
    }
}

/// What unit of measure the mouse wheel delta is in
///
/// [(JavaScipt docs)](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseWheelDeltaMode {
    /// The unit of measurement for the delta is pixels
    // https://w3c.github.io/uievents/#dom-wheelevent-dom_delta_pixel
    Pixel,
    /// The unit of measurement for the delta is lines
    // https://w3c.github.io/uievents/#dom-wheelevent-dom_delta_line
    Line,
     /// The unit of measurement for the delta is pages
    // https://w3c.github.io/uievents/#dom-wheelevent-dom_delta_page
    Page
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::event::ConcreteEvent;

    #[test]
    fn test_mouse_event() {
        let event: MouseEvent = js!(
            return new MouseEvent(
                @{ClickEvent::EVENT_TYPE},
                {
                    altKey: false,
                    button: 2,
                    buttons: 6,
                    clientX: 3,
                    clientY: 4,
                    ctrlKey: true,
                    metaKey: false,
                    screenX: 1,
                    screenY: 2,
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
        assert_eq!( event.client_x(), 3 );
        assert_eq!( event.client_y(), 4 );
        assert!( event.ctrl_key() );
        assert!( !event.get_modifier_state( ModifierKey::Alt ) );
        assert!( event.get_modifier_state( ModifierKey::Ctrl ) );
        assert!( event.get_modifier_state( ModifierKey::Shift ) );
        assert!( !event.meta_key() );
        assert_eq!( event.movement_x(), 0 );
        assert_eq!( event.movement_y(), 0 );
        assert!( event.region().is_none() );
        assert!( event.related_target().is_none() );
        assert_eq!( event.screen_x(), 1 );
        assert_eq!( event.screen_y(), 2 );
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
    fn test_aux_click_event() {
        let event: AuxClickEvent = js!(
            return new MouseEvent( @{AuxClickEvent::EVENT_TYPE} );
        ).try_into()
            .unwrap();
        assert_eq!( event.event_type(), AuxClickEvent::EVENT_TYPE );
    }

    #[test]
    fn test_context_menu_event() {
        let event: ContextMenuEvent = js!(
            return new MouseEvent( @{ContextMenuEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), ContextMenuEvent::EVENT_TYPE );
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

    #[test]
    fn test_mouse_over_event() {
        let event: MouseOverEvent = js!(
            return new MouseEvent( @{MouseOverEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), MouseOverEvent::EVENT_TYPE );
    }

    #[test]
    fn test_mouse_out_event() {
        let event: MouseOutEvent = js!(
            return new MouseEvent( @{MouseOutEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), MouseOutEvent::EVENT_TYPE );
    }

    #[test]
    fn test_mouse_enter_event() {
        let event: MouseEnterEvent = js!(
            return new MouseEvent( @{MouseEnterEvent::EVENT_TYPE} );
        ).try_into()
            .unwrap();
        assert_eq!( event.event_type(), MouseEnterEvent::EVENT_TYPE );
    }

    #[test]
    fn test_mouse_leave_event() {
        let event: MouseLeaveEvent = js!(
            return new MouseEvent( @{MouseLeaveEvent::EVENT_TYPE} );
        ).try_into()
            .unwrap();
        assert_eq!( event.event_type(), MouseLeaveEvent::EVENT_TYPE );
    }

    #[test]
    fn test_mouse_wheel_event() {
        let event: MouseWheelEvent = js!(
            return new MouseEvent( @{MouseWheelEvent::EVENT_TYPE} );
        ).try_into()
            .unwrap();
        assert_eq!( event.event_type(), MouseWheelEvent::EVENT_TYPE );
    }
}

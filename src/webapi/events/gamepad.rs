//use std::fmt::Debug;

use webcore::value::Reference;
use webcore::try_from::TryInto;

use webapi::event::{IEvent, Event, ConcreteEvent};
use webapi::gamepad::Gamepad;

/// A GamepadEvent is fired on the window object, when a gamepad is connected or disconnected to the system.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)
// https://www.w3.org/TR/gamepad/#gamepadevent-interface
pub trait IGamepadEvent: IEvent {

    /// Returns the gamepad associated with this event.
    #[inline]
    fn gamepad( &self ) -> Gamepad {
        js!(
            return @{self.as_ref()}.gamepad;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IGamepadEvent](trait.IGamepadEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)
// https://www.w3.org/TR/gamepad/#gamepadevent-interface
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadEvent")]
#[reference(subclass_of(Event))]
pub struct GamepadEvent( Reference );

impl IEvent for GamepadEvent {}
impl IGamepadEvent for GamepadEvent {}

/// The `GamepadConnected` event is fired on the window object, when the first input is received for a gamepad.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/gamepadconnected)
// https://www.w3.org/TR/gamepad/#event-gamepadconnected
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadEvent")]
#[reference(subclass_of(Event, GamepadEvent))]
pub struct GamepadConnectedEvent( Reference );

impl IEvent for GamepadConnectedEvent {}
impl IGamepadEvent for GamepadConnectedEvent {}
impl ConcreteEvent for GamepadConnectedEvent {
    const EVENT_TYPE: &'static str = "gamepadconnected";
}

/// The `GamepadDisconnected` event is fired on the window object, when a gamepad is disconnected.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/gamepaddisconnected)
// https://www.w3.org/TR/gamepad/#event-gamepaddisconnected
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadEvent")]
#[reference(subclass_of(Event, GamepadEvent))]
pub struct GamepadDisconnectedEvent( Reference );

impl IEvent for GamepadDisconnectedEvent {}
impl IGamepadEvent for GamepadDisconnectedEvent {}
impl ConcreteEvent for GamepadDisconnectedEvent {
    const EVENT_TYPE: &'static str = "gamepaddisconnected";
}

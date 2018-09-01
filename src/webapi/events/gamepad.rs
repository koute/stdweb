use webcore::value::Reference;
use webcore::try_from::TryInto;

use webapi::event::{IEvent, Event};
use webapi::gamepad::Gamepad;

/// A GamepadEvent is fired on the window object, when a gamepad is connected or disconnected to the system.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)
// https://w3c.github.io/gamepad/#gamepadevent-interface
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
// https://w3c.github.io/gamepad/#gamepadevent-interface
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadEvent")]
#[reference(subclass_of(Event))]
pub struct GamepadEvent( Reference );

impl IEvent for GamepadEvent {}
impl IGamepadEvent for GamepadEvent {}

/// The `GamepadConnected` event is fired on the window object, when the first input is received for a gamepad.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/gamepadconnected)
// https://w3c.github.io/gamepad/#event-gamepadconnected
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadEvent")]
#[reference(event = "gamepadconnected")]
#[reference(subclass_of(Event, GamepadEvent))]
pub struct GamepadConnectedEvent( Reference );

impl IEvent for GamepadConnectedEvent {}
impl IGamepadEvent for GamepadConnectedEvent {}

/// The `GamepadDisconnected` event is fired on the window object, when a gamepad is disconnected.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/gamepaddisconnected)
// https://w3c.github.io/gamepad/#event-gamepaddisconnected
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadEvent")]
#[reference(event = "gamepaddisconnected")]
#[reference(subclass_of(Event, GamepadEvent))]
pub struct GamepadDisconnectedEvent( Reference );

impl IEvent for GamepadDisconnectedEvent {}
impl IGamepadEvent for GamepadDisconnectedEvent {}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_gamepad_connected_event() {

        let event: GamepadConnectedEvent = js!(
            return new GamepadEvent("gamepadconnected");
        ).try_into().unwrap();

        assert_eq!(event.event_type(), "gamepadconnected");
    }

    #[test]
    fn test_gamepad_disconnected_event() {

        let event: GamepadDisconnectedEvent = js!(
            return new GamepadEvent("gamepaddisconnected");
        ).try_into().unwrap();

        assert_eq!(event.event_type(), "gamepaddisconnected");
    }
}

//use std::fmt::Debug;

use webcore::reference_type::ReferenceType;
use webcore::try_from::{
    TryFrom,
    TryInto,
};
use webcore::value::{
    ConversionError,
    Reference,
    Value,
};

/// The set of known gamepad layout mappings.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)
// https://w3c.github.io/gamepad/#dom-gamepadmappingtype
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GamepadMappingType {
    /// No mapping is in use for this gamepad
    NoMapping,
    /// This gamepad is mapped to the [Standard Gamepad layout](https://w3c.github.io/gamepad/#remapping)
    Standard,
}

impl TryFrom<Value> for GamepadMappingType {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::String(s) => match s.as_ref() {
                "" => Ok(GamepadMappingType::NoMapping),
                "standard" => Ok(GamepadMappingType::Standard),
                s => Err(ConversionError::Custom(format!("invalid gamepad mapping type \"{}\"", s))),
            },
            _ => Err(ConversionError::type_mismatch(&v)),
        }
    }
}

/// The state of an individual button on a gamepad device.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)
// https://w3c.github.io/gamepad/#gamepadbutton-interface
pub trait IGamepadButton: ReferenceType {

    /// Is the button currently pressed?
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/pressed)
    // https://w3c.github.io/gamepad/#dom-gamepadbutton-pressed
    #[inline]
    fn pressed(&self) -> bool {
        js!(
            return @{self.as_ref()}.pressed;
        ).try_into().unwrap()
    }

    /// Is the button currently touched?
    ///
    /// MDN does not document this, it may be unsupported by browsers.
    // https://w3c.github.io/gamepad/#dom-gamepadbutton-touched
    #[inline]
    fn touched(&self) -> bool {
        js!(
            return @{self.as_ref()}.touched;
        ).try_into().unwrap()
    }

    /// The amount which the button has been pressed, between 0.0 (not pressed), and 1.0 (fully pressed).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/value)
    // https://w3c.github.io/gamepad/#dom-gamepadbutton-touched
    #[inline]
    fn value(&self) -> f64 {
        js!(
            return @{self.as_ref()}.value;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IGamepadButton](trait.IGamepadButton.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)
// https://w3c.github.io/gamepad/#gamepadbutton-interface
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadButton")]
pub struct GamepadButton( Reference );

impl IGamepadButton for GamepadButton {}

/// An individual gamepad/controller.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)
// https://w3c.github.io/gamepad/#gamepad-interface
pub trait IGamepad: ReferenceType {

    /// A string containing some information about this gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)
    // https://www.w3.org/TR/gamepad/#dom-gamepad-id
    #[inline]
    fn id(&self) -> String {
        js!(
            return @{self.as_ref()}.id;
        ).try_into().unwrap()
    }

    /// An auto-incrementing integer to uniquely identify a connected Gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)
    // https://www.w3.org/TR/gamepad/#dom-gamepad-index
    #[inline]
    fn index(&self) -> i32 {
        js!(
            return @{self.as_ref()}.index;
        ).try_into().unwrap()
    }

    /// Is this gamepad connected to the system, powered on, and usable?
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)
    // https://www.w3.org/TR/gamepad/#dom-gamepad-connected
    #[inline]
    fn connected(&self) -> bool {
        js!(
            return @{self.as_ref()}.connected;
        ).try_into().unwrap()
    }

    /// Monotonically increasing time this gamepad was updated.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)
    // https://www.w3.org/TR/gamepad/#dom-gamepad-timestamp
    #[inline]
    fn timestamp(&self) -> f64 {
        js!(
            return @{self.as_ref()}.timestamp;
        ).try_into().unwrap()
    }

    /// The mapping in use for this device.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)
    // https://www.w3.org/TR/gamepad/#dom-gamepad-mapping
    #[inline]
    fn mapping(&self) -> GamepadMappingType {
        js!(
            return @{self.as_ref()}.mapping;
        ).try_into().unwrap()
    }

    /// Array of values for all axes of the gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)
    // https://www.w3.org/TR/gamepad/#dom-gamepad-axes
    #[inline]
    fn axes(&self) -> Vec<f64> {
        js!(
            return @{self.as_ref()}.axes;
        ).try_into().unwrap()
    }

    /// Array of button states for all buttons of the gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)
    // https://www.w3.org/TR/gamepad/#dom-gamepad-buttons
    #[inline]
    fn buttons(&self) -> Vec<GamepadButton> {
        js!(
            return @{self.as_ref()}.buttons;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IGamepad](trait.IGamepad.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)
// https://w3c.github.io/gamepad/#gamepad-interface
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "Gamepad")]
pub struct Gamepad( Reference );

impl IGamepad for Gamepad {}

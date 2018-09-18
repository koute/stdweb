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
        let value: String = v.try_into()?;
        match value.as_ref() {
            "" => Ok(GamepadMappingType::NoMapping),
            "standard" => Ok(GamepadMappingType::Standard),
            s => Err(ConversionError::Custom(format!("invalid gamepad mapping type \"{}\"", s))),
        }
    }
}

/// The state of an individual button on a gamepad device.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)
// https://w3c.github.io/gamepad/#gamepadbutton-interface
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "GamepadButton")]
pub struct GamepadButton( Reference );

impl GamepadButton {

    /// Is the button currently pressed?
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/pressed)
    // https://w3c.github.io/gamepad/#dom-gamepadbutton-pressed
    #[inline]
    pub fn pressed(&self) -> bool {
        js!(
            return @{self.as_ref()}.pressed;
        ).try_into().unwrap()
    }

    /// Is the button currently touched?
    ///
    /// MDN does not document this. Firefox supports it, but Chrome (as of v65) does not.
    // https://w3c.github.io/gamepad/#dom-gamepadbutton-touched
    #[inline]
    pub fn touched(&self) -> bool {
        js!(
            return @{self.as_ref()}.touched;
        ).try_into().unwrap()
    }

    /// The amount which the button has been pressed, between 0.0 (not pressed), and 1.0 (fully pressed).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/value)
    // https://w3c.github.io/gamepad/#dom-gamepadbutton-value
    #[inline]
    pub fn value(&self) -> f64 {
        js!(
            return @{self.as_ref()}.value;
        ).try_into().unwrap()
    }
}

/// An individual gamepad/controller.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)
// https://w3c.github.io/gamepad/#gamepad-interface
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "Gamepad")]
pub struct Gamepad( Reference );

impl Gamepad {

    /// A string containing some information about this gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)
    // https://w3c.github.io/gamepad/#dom-gamepad-id
    #[inline]
    pub fn id(&self) -> String {
        js!(
            return @{self.as_ref()}.id;
        ).try_into().unwrap()
    }

    /// An auto-incrementing integer to uniquely identify a connected Gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)
    // https://w3c.github.io/gamepad/#dom-gamepad-index
    #[inline]
    pub fn index(&self) -> i32 {
        js!(
            return @{self.as_ref()}.index;
        ).try_into().unwrap()
    }

    /// Is this gamepad connected to the system, powered on, and usable?
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)
    // https://w3c.github.io/gamepad/#dom-gamepad-connected
    #[inline]
    pub fn connected(&self) -> bool {
        js!(
            return @{self.as_ref()}.connected;
        ).try_into().unwrap()
    }

    /// Monotonically increasing time this gamepad was updated.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)
    // https://w3c.github.io/gamepad/#dom-gamepad-timestamp
    #[inline]
    pub fn timestamp(&self) -> f64 {
        js!(
            return @{self.as_ref()}.timestamp;
        ).try_into().unwrap()
    }

    /// The mapping in use for this device.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)
    // https://w3c.github.io/gamepad/#dom-gamepad-mapping
    #[inline]
    pub fn mapping(&self) -> GamepadMappingType {
        js!(
            return @{self.as_ref()}.mapping;
        ).try_into().unwrap()
    }

    /// Array of values for all axes of the gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)
    // https://w3c.github.io/gamepad/#dom-gamepad-axes
    #[inline]
    pub fn axes(&self) -> Vec<f64> {
        js!(
            return @{self.as_ref()}.axes;
        ).try_into().unwrap()
    }

    /// Array of button states for all buttons of the gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)
    // https://w3c.github.io/gamepad/#dom-gamepad-buttons
    #[inline]
    pub fn buttons(&self) -> Vec<GamepadButton> {
        js!(
            return @{self.as_ref()}.buttons;
        ).try_into().unwrap()
    }

    /// Retrieve all connected gamepads, in an array indexed by each gamepad's `index` member.
    ///
    /// Chrome doesn't update Gamepad state automatically, you must call this function each frame.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)
    // https://w3c.github.io/gamepad/#dom-navigator-getgamepads
    pub fn get_all() -> Vec<Option<Gamepad>> {
        js!(
            return Array.from(navigator.getGamepads());
        ).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::GamepadMappingType;

    use webcore::try_from::TryInto;
    use webcore::value::{ConversionError, Value};

    #[test]
    fn test_value_into_gamepad_mapping() {

        let to_mapping = |v: Value| -> Result<GamepadMappingType, ConversionError> {
            v.try_into()
        };

        assert_eq!(to_mapping("standard".into()), Ok(GamepadMappingType::Standard));
        assert_eq!(to_mapping("".into()), Ok(GamepadMappingType::NoMapping));
        assert!(to_mapping("fakemapping".into()).is_err());
        assert!(to_mapping(Value::Null).is_err());
    }

    // most of the Gamepad API is not testable,
    // because Gamepad and GamepadButton are not constructible
}

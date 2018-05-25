//use std::fmt::Debug;

use webcore::reference_type::ReferenceType;
use webcore::try_from::TryInto;
use webcore::value::Reference;

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

/// An individual gamepad/controller.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)
// https://w3c.github.io/gamepad/#gamepad-interface
pub trait IGamepad: ReferenceType {

    /// A string containing some information about this gamepad.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)
    // https://w3c.github.io/gamepad/#gamepad-interface
    #[inline]
    fn id(&self) -> String {
        js!(
            return @{self.as_ref()}.id;
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

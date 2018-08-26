use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event::{IEvent, Event};

// Used by KeyboardEvent and MouseEvent to get the state of a modifier key.
pub(crate) fn get_event_modifier_state< T: IEvent >( event: &T, key: ModifierKey ) -> bool {
    js!(
        return @{event.as_ref()}.getModifierState( @{
            match key {
                ModifierKey::Alt => "Alt",
                ModifierKey::AltGr => "AltGraph",
                ModifierKey::CapsLock => "CapsLock",
                ModifierKey::Ctrl => "Control",
                ModifierKey::Function => "Fn",
                ModifierKey::FunctionLock => "FnLock",
                ModifierKey::Hyper => "Hyper",
                ModifierKey::Meta => "Meta",
                ModifierKey::NumLock => "NumLock",
                ModifierKey::OS => "OS",
                ModifierKey::ScrollLock => "ScrollLock",
                ModifierKey::Shift => "Shift",
                ModifierKey::Super => "Super",
                ModifierKey::Symbol => "Symbol",
                ModifierKey::SymbolLock => "SymbolLock",
            }
        } );
    ).try_into().unwrap()
}

/// `IKeyboardEvent` objects describe a user interaction with the
/// keyboard. Each event describes a key; the event type identifies
/// what kind of activity was performed.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)
// https://w3c.github.io/uievents/#keyboardevent
pub trait IKeyboardEvent: IEvent {
    /// Indicates whether the Alt key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-altkey-3
    #[inline]
    fn alt_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.altKey;
        ).try_into().unwrap()
    }

    /// Returns a code value that indicates the physical key pressed on the keyboard.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-code-3
    #[inline]
    fn code( &self ) -> String {
        js!(
            return @{self.as_ref()}.code;
        ).try_into().unwrap()
    }

    /// Returns whether the Ctrl key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-ctrlkey-3
    #[inline]
    fn ctrl_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.ctrlKey;
        ).try_into().unwrap()
    }


    /// Returns whether a modifier key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-getmodifierstate-19
    #[inline]
    fn get_modifier_state( &self, key: ModifierKey ) -> bool {
        get_event_modifier_state( self, key )
    }

    /// Returns whether this event was fired during composition.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-iscomposing-1
    #[inline]
    fn is_composing( &self ) -> bool {
        js!(
            return @{self.as_ref()}.isComposing;
        ).try_into().unwrap()
    }

    /// Returns the location of the key on the keyboard.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-location-1
    fn location( &self ) -> KeyboardLocation {
        match js!(
            return @{self.as_ref()}.location;
        ).try_into().unwrap() {
            0 => KeyboardLocation::Standard,
            1 => KeyboardLocation::Left,
            2 => KeyboardLocation::Right,
            3 => KeyboardLocation::Numpad,
            4 => KeyboardLocation::Mobile,
            5 => KeyboardLocation::Joystick,
            _ => unreachable!("Unexpected KeyboardEvent.location value"),
        }
    }

    /// Returns the value of a key or keys pressed by the user.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-key-4
    #[inline]
    fn key( &self ) -> String {
        js!(
            return @{self.as_ref()}.key;
        ).into_string().unwrap()
    }

    /// Indicates whether the Meta key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/metaKey)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-metakey-3
    #[inline]
    fn meta_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.metaKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the key is held down such that it is repeating.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-repeat-1
    #[inline]
    fn repeat( &self ) -> bool {
        js!(
            return @{self.as_ref()}.repeat;
        ).try_into().unwrap()
    }

    /// Indicates whether the Shift key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)
    // https://w3c.github.io/uievents/#ref-for-dom-keyboardevent-shiftkey-3
    #[inline]
    fn shift_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.shiftKey;
        ).try_into().unwrap()
    }
}

/// A modifier key on the keyboard.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModifierKey {
    Alt,
    AltGr,
    CapsLock,
    Ctrl,
    Function,
    FunctionLock,
    Hyper,
    Meta,
    NumLock,
    OS,
    ScrollLock,
    Shift,
    Super,
    Symbol,
    SymbolLock,
}

/// The location on the keyboard of a key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyboardLocation {
    /// The key has only one version, or the location can't be distinguished.
    Standard,
    /// The left-hand version of a key.
    Left,
    /// The right-hand version of a key.
    Right,
    /// The key was on a numeric pad.
    Numpad,
    /// The key was on a mobile device.
    Mobile,
    /// The key was on a joystick.
    Joystick,
}

/// A reference to a JavaScript object which implements the [IKeyboardEvent](trait.IKeyboardEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)
// https://w3c.github.io/uievents/#keyboardevent
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "KeyboardEvent")]
#[reference(subclass_of(Event))]
pub struct KeyboardEvent( Reference );

impl IEvent for KeyboardEvent {}
impl IKeyboardEvent for KeyboardEvent {}

/// The `KeyPressEvent` is fired when a key is pressed down. It's only
/// fired for keys which produce a character value.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/keypress)
// https://w3c.github.io/uievents/#keypress
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "KeyboardEvent")]
#[reference(event = "keypress")]
#[reference(subclass_of(Event, KeyboardEvent))]
pub struct KeyPressEvent( Reference );

impl IEvent for KeyPressEvent {}
impl IKeyboardEvent for KeyPressEvent {}

/// The `KeyDownEvent` is fired when a key is pressed down.
/// Unlike the `KeyPressEvent` event it's also fired for keys which
/// do not produce a character value.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/keydown)
// https://w3c.github.io/uievents/#event-type-keydown
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "KeyboardEvent")]
#[reference(event = "keydown")]
#[reference(subclass_of(Event, KeyboardEvent))]
pub struct KeyDownEvent( Reference );

impl IEvent for KeyDownEvent {}
impl IKeyboardEvent for KeyDownEvent {}

/// The `KeyUpEvent` is fired when a key is released.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/keyup)
// https://w3c.github.io/uievents/#event-type-keyup
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "KeyboardEvent")]
#[reference(event = "keyup")]
#[reference(subclass_of(Event, KeyboardEvent))]
pub struct KeyUpEvent( Reference );

impl IEvent for KeyUpEvent {}
impl IKeyboardEvent for KeyUpEvent {}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::event::ConcreteEvent;

    #[test]
    fn test_keyboard_event() {
        let event: KeyboardEvent = js!(
            return new KeyboardEvent(
                @{KeyPressEvent::EVENT_TYPE},
                {
                    key: "A",
                    code: "KeyA",
                    location: 0,
                    ctrlKey: true,
                    shiftKey: false,
                    altKey: true,
                    metaKey: false,
                    repeat: true,
                    isComposing: false
                }
            );
        ).try_into().unwrap();
        assert!( event.alt_key() );
        assert_eq!( event.code(), "KeyA" );
        assert!( event.ctrl_key() );
        assert!( event.get_modifier_state( ModifierKey::Alt ) );
        assert!( event.get_modifier_state( ModifierKey::Ctrl ) );
        assert!( !event.get_modifier_state( ModifierKey::Shift ) );
        assert!( !event.is_composing() );
        assert_eq!( event.location(), KeyboardLocation::Standard );
        assert_eq!( event.key(), "A" );
        assert!( !event.meta_key() );
        assert!( event.repeat() );
        assert!( !event.shift_key() );
    }

    #[test]
    fn test_keypress_event() {
        let event: KeyPressEvent = js!(
            return new KeyboardEvent( @{KeyPressEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), KeyPressEvent::EVENT_TYPE );
    }

    #[test]
    fn test_keydown_event_is_only_constructible_from_a_keydown_event() {
        let event: Option< KeyDownEvent > = js!(
            return new KeyboardEvent( "keydown" );
        ).try_into().ok();
        assert!( event.is_some() );

        let event: Option< KeyDownEvent > = js!(
            return new KeyboardEvent( "keyup" );
        ).try_into().ok();
        assert!( event.is_none() );
    }
}

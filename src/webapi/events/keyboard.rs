use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event::{IEvent, Event, ConcreteEvent};

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
pub trait IKeyboardEvent: IEvent {
    /// Indicates whether the Alt key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)
    #[inline]
    fn alt_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.altKey;
        ).try_into().unwrap()
    }

    /// Returns a code value that indicates the physical key pressed on the keyboard.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)
    #[inline]
    fn code( &self ) -> String {
        js!(
            return @{self.as_ref()}.code;
        ).try_into().unwrap()
    }

    /// Returns whether the Ctrl key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)
    #[inline]
    fn ctrl_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.ctrlKey;
        ).try_into().unwrap()
    }


    /// Returns whether a modifier key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)
    #[inline]
    fn get_modifier_state( &self, key: ModifierKey ) -> bool {
        get_event_modifier_state( self, key )
    }

    /// Returns whether this event was fired during composition.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)
    #[inline]
    fn is_composing( &self ) -> bool {
        js!(
            return @{self.as_ref()}.isComposing;
        ).try_into().unwrap()
    }

    /// Returns the location of the key on the keyboard.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location)
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
    #[inline]
    fn key( &self ) -> String {
        js!(
            return @{self.as_ref()}.key;
        ).into_string().unwrap()
    }

    /// Indicates whether the Meta key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/metaKey)
    #[inline]
    fn meta_key( &self ) -> bool {
        js!(
            return @{self.as_ref()}.metaKey;
        ).try_into().unwrap()
    }

    /// Indicates whether the key is held down such that it is repeating.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)
    #[inline]
    fn repeat( &self ) -> bool {
        js!(
            return @{self.as_ref()}.repeat;
        ).try_into().unwrap()
    }

    /// Indicates whether the Shift key was down when this event was fired.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)
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
pub struct KeyboardEvent( Reference );

impl IEvent for KeyboardEvent {}
impl IKeyboardEvent for KeyboardEvent {}

reference_boilerplate! {
    KeyboardEvent,
    instanceof KeyboardEvent
    convertible to Event
}

/// The `KeypressEvent` is fired when a key is pressed down. It's only
/// fired for keys which produce a character value.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/keypress)
pub struct KeypressEvent( Reference );

impl IEvent for KeypressEvent {}
impl IKeyboardEvent for KeypressEvent {}
impl ConcreteEvent for KeypressEvent {
    const EVENT_TYPE: &'static str = "keypress";
}

reference_boilerplate! {
    KeypressEvent,
    instanceof KeyboardEvent
    convertible to Event
    convertible to KeyboardEvent
}

/// The `KeyDownEvent` is fired when a key is pressed down.
/// Unlike the `KeypressEvent` event it's also fired for keys which
/// do not produce a character value.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/keydown)
pub struct KeyDownEvent( Reference );

impl IEvent for KeyDownEvent {}
impl IKeyboardEvent for KeyDownEvent {}
impl ConcreteEvent for KeyDownEvent {
    const EVENT_TYPE: &'static str = "keydown";
}

reference_boilerplate! {
    KeyDownEvent,
    instanceof KeyboardEvent
    convertible to Event
    convertible to KeyboardEvent
}

/// The `KeyUpEvent` is fired when a key is released.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/keyup)
pub struct KeyUpEvent( Reference );

impl IEvent for KeyUpEvent {}
impl IKeyboardEvent for KeyUpEvent {}
impl ConcreteEvent for KeyUpEvent {
    const EVENT_TYPE: &'static str = "keyup";
}

reference_boilerplate! {
    KeyUpEvent,
    instanceof KeyboardEvent
    convertible to Event
    convertible to KeyboardEvent
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_event() {
        let event: KeyboardEvent = js!(
            return new KeyboardEvent(
                @{KeypressEvent::EVENT_TYPE},
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
        let event: KeypressEvent = js!(
            return new KeyboardEvent( @{KeypressEvent::EVENT_TYPE} );
        ).try_into().unwrap();
        assert_eq!( event.event_type(), KeypressEvent::EVENT_TYPE );
    }
}

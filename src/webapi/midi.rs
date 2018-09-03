use std::fmt;
use webcore::try_from::{TryInto, TryFrom};
use webcore::value::{Value, Reference};
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::dom_exception::{
    SecurityError,
    InvalidStateError,
    NotSupportedError,
    AbortError
};

#[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
use webcore::promise::{Promise, TypedPromise};

/// This structure contains optional settings that may be provided
/// when requesting MIDI access.
// https://webaudio.github.io/web-midi-api/#dom-midioptions
#[derive(Clone, PartialEq, Debug, Default)]
pub struct MidiOptions {
    /// This member informs the system whether the ability to send and receive
    /// system exclusive messages is requested or allowed. If this member is
    /// set to true, but system exclusive support is denied (either by policy
    /// or by user action), the access request will fail with a `SecurityError`
    /// error. If this support is not requested (and allowed), the system will
    /// throw exceptions if the user tries to send system exclusive messages,
    /// and will silently mask out any system exclusive messages received on
    /// the port.
    request_sysex: bool,

    /// This member informs the system whether the ability to utilize any software
    /// synthesizers installed in the host system is requested or allowed. If this
    /// member is set to true, but software synthesizer support is denied (either
    /// by policy or by user action), the access request will fail with a `SecurityError`
    /// error.
    ///
    /// Note that may result in a two-step request procedure if software synthesizer
    /// support is desired but not required - software synthesizers may be disabled
    /// when MIDI hardware device access is allowed.
    request_software_synth: bool,

    #[doc(hidden)]
    __non_exhaustive: ()
}

error_enum_boilerplate! {
    MidiAccessError,
    SecurityError, AbortError, InvalidStateError, NotSupportedError
}

/// This object provides the methods to list MIDI input and output devices,
/// and obtain access to an individual device.
// https://webaudio.github.io/web-midi-api/#dom-midiaccess
#[derive(Clone, Debug, Eq, PartialEq, ReferenceType)]
#[reference(instance_of = "MIDIAccess")]
#[reference(subclass_of(EventTarget))]
pub struct MidiAccess( Reference );

impl MidiAccess {
    /// Requests access to MIDI devices.
    // https://webaudio.github.io/web-midi-api/#dom-navigator-requestmidiaccess
    #[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
    pub fn new_with_options( options: &MidiOptions ) -> TypedPromise< MidiAccess, MidiAccessError > {
        let promise: Promise = js!(
            if( !navigator.requestMIDIAccess ) {
                return new Promise( function( resolve, reject ) {
                    reject( new DOMException( "WebMIDI is not supported by your browser!", "NotSupportedError" ) );
                });
            }

            return navigator.requestMIDIAccess({
                sysex: @{options.request_sysex},
                software: @{options.request_software_synth}
            });
        ).try_into().unwrap();

        TypedPromise::new( promise )
    }

    /// Requests access to MIDI devices with default options.
    // https://webaudio.github.io/web-midi-api/#dom-navigator-requestmidiaccess
    #[cfg(feature = "experimental_features_which_may_break_on_minor_version_bumps")]
    pub fn new() -> TypedPromise< MidiAccess, MidiAccessError > {
        Self::new_with_options( &MidiOptions::default() )
    }

    /// The MIDI input ports available to the system.
    // https://webaudio.github.io/web-midi-api/#dom-midiaccess-inputs
    pub fn inputs( &self ) -> MidiInputMap {
        return js!(
            return @{self}.inputs;
        ).try_into().unwrap()
    }

    /// The MIDI output ports available to the system.
    // https://webaudio.github.io/web-midi-api/#dom-midiaccess-outputs
    pub fn outputs( &self ) -> MidiOutputMap {
        return js!(
            return @{self}.outputs;
        ).try_into().unwrap()
    }

    /// This attribute informs the user whether system exclusive support is enabled.
    // https://webaudio.github.io/web-midi-api/#dom-midiaccess-sysexenabled
    pub fn sysex_enabled( &self ) -> bool {
        return js!(
            return @{self}.sysexEnabled;
        ).try_into().unwrap()
    }
}

fn map_iter_next< K, V >( iter: &Value ) -> Option< (K, V) >
    where K: TryFrom< Value >,
          V: TryFrom< Value >,
          K::Error: fmt::Debug,
          V::Error: fmt::Debug
{
    let entry = js!( return @{&iter}.next(); );
    let is_done: bool = js!( return @{&entry}.done; ).try_into().unwrap();
    if is_done {
        return None;
    }

    let key = js!( return @{&entry}.value[0] ).try_into().unwrap();
    let value = js!( return @{&entry}.value[1] ).try_into().unwrap();
    Some( (key, value) )
}

/// This type is used to represent all the currently available MIDI input ports.
// https://webaudio.github.io/web-midi-api/#dom-midiinputmap
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MIDIInputMap")]
pub struct MidiInputMap( Reference );

macro_rules! define_map {
    ($map_name:ident, $item_name:ident, $iter_name:ident) => {
        impl $map_name {
            /// Returns the number of elements in this map.
            pub fn len( &self ) -> u32 {
                js!( return @{self}.size; ).try_into().unwrap()
            }

            /// Returns an iterator over the map.
            pub fn iter( &self ) -> $iter_name {
                self.into_iter()
            }
        }

        #[derive(Debug)]
        pub struct $iter_name {
            iter: Value
        }

        impl Iterator for $iter_name {
            type Item = (String, $item_name);
            fn next( &mut self ) -> Option< Self::Item > {
                map_iter_next( &self.iter )
            }
        }

        impl IntoIterator for $map_name {
            type Item = (String, $item_name);
            type IntoIter = $iter_name;

            #[inline]
            fn into_iter( self ) -> Self::IntoIter {
                $iter_name {
                    iter: js!( return @{self}.entries(); )
                }
            }
        }

        impl< 'a > IntoIterator for &'a $map_name {
            type Item = (String, $item_name);
            type IntoIter = $iter_name;

            #[inline]
            fn into_iter( self ) -> Self::IntoIter {
                self.clone().into_iter()
            }
        }
    }
}

/// This interface represents a MIDI input or output port.
pub trait IMidiPort: IEventTarget {
    /// A unique ID of the port.
    ///
    /// This can be used by developers to remember ports the user
    /// has chosen for their application.
    // https://webaudio.github.io/web-midi-api/#dom-midiport-id
    fn id( &self ) -> String {
        return js!( return @{self.as_ref()}.id; ).try_into().unwrap();
    }

    /// The system name of the port.
    // https://webaudio.github.io/web-midi-api/#dom-midiport-name
    fn name( &self ) -> Option< String > {
        return js!( return @{self.as_ref()}.name; ).try_into().unwrap();
    }
}

/// This object represents a MIDI input or output port.
// https://webaudio.github.io/web-midi-api/#dom-midiport
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MIDIPort")]
#[reference(subclass_of(EventTarget))]
pub struct MidiPort( Reference );

impl IEventTarget for MidiPort {}
impl IMidiPort for MidiPort {}

/// This type is used to represent all the currently available MIDI output ports.
// https://webaudio.github.io/web-midi-api/#dom-midioutputmap
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MIDIOutputMap")]
pub struct MidiOutputMap( Reference );

define_map!( MidiInputMap, MidiInput, MidiInputMapIter );
define_map!( MidiOutputMap, MidiOutput, MidiOutputMapIter );

/// A MIDI input port.
// https://webaudio.github.io/web-midi-api/#dom-midiinput
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MIDIInput")]
#[reference(subclass_of(EventTarget, MidiPort))]
pub struct MidiInput( Reference );

impl IEventTarget for MidiInput {}
impl IMidiPort for MidiInput {}

/// A MIDI output port.
// https://webaudio.github.io/web-midi-api/#dom-midioutput
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MIDIOutput")]
#[reference(subclass_of(EventTarget, MidiPort))]
pub struct MidiOutput( Reference );

impl IEventTarget for MidiOutput {}
impl IMidiPort for MidiOutput {}

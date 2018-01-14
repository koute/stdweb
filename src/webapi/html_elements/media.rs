use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::node::{INode, Node};
use webapi::element::{IElement, Element};
use webapi::html_element::{IHtmlElement, HtmlElement};

/// The readiness state of a media element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReadyState {
    /// No information is available about this media resource yet.
    HaveNothing,
    /// Metadata for the media has been loaded, and seeking is possible.
    HaveMetadata,
    /// Data is available for the current playback position, but not for more
    /// than one frame.
    HaveCurrentData,
    /// Data for several frames after the current position is available, but
    /// not enough to allow consistent playback.
    HaveFutureData,
    /// Enough data is available to permit uninterrupted playback at the current
    /// download rate.
    HaveEnoughData,
}

/// The `IHtmlElement` interface represents an `<audio>` or `<video>` element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)
pub trait IHtmlMediaElement: IHtmlElement {

    /// Should the media start playing as soon as enough data has been loaded
    /// that is should be able play without interruption?
    #[inline]
    fn autoplay( &self ) -> bool {
        js! (
            return @{self.as_ref()}.autoplay;
        ).try_into().unwrap()
    }

    /// Sets whether autoplay is enabled.
    #[inline]
    fn set_autoplay( &self, value: bool ) {
        js! { @(no_return)
            @{self.as_ref()}.autoplay = @{value};
        }
    }

    /// Should media-playing controls be displayed for this element?
    #[inline]
    fn controls( &self ) -> bool {
        js! (
            return @{self.as_ref()}.controls;
        ).try_into().unwrap()
    }

    /// Sets whether media-playing controls should be displayed.
    #[inline]
    fn set_controls( &self, value: bool ) {
        js! { @(no_return)
            @{self.as_ref()}.controls = @{value};
        }
    }

    /// The current playback time, in seconds.
    #[inline]
    fn current_time( &self ) -> f64 {
        js! (
            return @{self.as_ref()}.currentTime;
        ).try_into().unwrap()
    }

    /// Set current playback time to the specified value, in seconds. This will
    /// seek playback.
    #[inline]
    fn set_current_time( &self, value: f64 ) {
        js! { @(no_return)
            @{self.as_ref()}.currentTime = @{value};
        }
    }

    /// Is this media element currently paused?
    #[inline]
    fn paused( &self ) -> bool {
        js! (
            return @{self.as_ref()}.paused;
        ).try_into().unwrap()
    }

    /// Is this media element currently paused?
    fn ready_state( &self ) -> ReadyState {
        let ready_state = js! (
            return @{self.as_ref()}.ready_state;
        ).try_into().unwrap();
        match ready_state {
            0 => ReadyState::HaveNothing,
            1 => ReadyState::HaveMetadata,
            2 => ReadyState::HaveCurrentData,
            3 => ReadyState::HaveFutureData,
            4 => ReadyState::HaveEnoughData,
            _ => unreachable!( "Unexpected value of MediaElement::ready_state: {}", ready_state )
        }
    }

    /// Pauses the media playback.
    fn pause( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.pause();
        }
    }

    /// Starts the media playback.
    fn play( &self ) {
        js! { @(no_return)
            @{self.as_ref()}.play();
        }
    }
}

/// A reference to a JavaScript object which implements the
/// [IMediaHtmlElement](trait.IMediaHtmlElement.html) interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)
pub struct HtmlMediaElement(Reference);

impl IEventTarget for HtmlMediaElement {}
impl INode for HtmlMediaElement {}
impl IElement for HtmlMediaElement {}
impl IHtmlElement for HtmlMediaElement {}

reference_boilerplate! {
    HtmlMediaElement,
    instanceof HTMLMediaElement
    convertible to EventTarget
    convertible to Node
    convertible to Element
    convertible to HtmlElement
}

/// The HTML `<audio>` element represents an audio stream with optional player
/// controls.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement)
pub struct AudioElement(Reference);

impl IEventTarget for AudioElement {}
impl INode for AudioElement {}
impl IElement for AudioElement {}
impl IHtmlElement for AudioElement {}
impl IHtmlMediaElement for AudioElement {}

reference_boilerplate! {
    AudioElement,
    instanceof HTMLAudioElement
    convertible to EventTarget
    convertible to Node
    convertible to Element
    convertible to HtmlElement
    convertible to HtmlMediaElement
}

/// The HTML `<video>` element represents a video stream with optional player
/// controls.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/HTMLVideoElement)
pub struct VideoElement(Reference);

impl IEventTarget for VideoElement {}
impl INode for VideoElement {}
impl IElement for VideoElement {}
impl IHtmlElement for VideoElement {}
impl IHtmlMediaElement for VideoElement {}

reference_boilerplate! {
    VideoElement,
    instanceof HTMLVideoElement
    convertible to EventTarget
    convertible to Node
    convertible to Element
    convertible to HtmlElement
    convertible to HtmlMediaElement
}

use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::window_or_worker::IWindowOrWorker;
use webapi::storage::Storage;
use webapi::location::Location;

/// The `Window` object represents a window containing a DOM document.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window)
pub struct Window( Reference );

impl IEventTarget for Window {}
impl IWindowOrWorker for Window {}

reference_boilerplate! {
    Window,
    instanceof Window
    convertible to EventTarget
}

/// A global instance of [Window](struct.Window.html).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window)
pub fn window() -> Window {
    unsafe { js!( return window; ).into_reference_unchecked() }.unwrap()
}

impl Window {
    /// The Window.alert() method displays an alert dialog
    /// with the optional specified content and an OK button.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    pub fn alert( &self, message: &str ) {
        js!( @(no_return)
            @{self}.alert( @{message} );
        );
    }

    /// The `local_storage` property allows you to access a local [Storage](struct.Storage.html)
    /// object.
    ///
    /// It is similar to the [Window::session_storage](struct.Window.html#method.session_storage).
    /// The only difference is that, while data stored in `local_storage` has
    /// no expiration time, data stored in `session_storage` gets cleared when
    /// the browsing session ends - that is, when the browser is closed.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    pub fn local_storage( &self ) -> Storage {
        unsafe {
            js!(
                return @{self.as_ref()}.localStorage;
            ).into_reference_unchecked().unwrap()
        }
    }

    /// The `session_storage` property allows you to access a session [Storage](struct.Storage.html)
    /// object for the current origin.
    ///
    /// It is similar to the [Window::local_storage](struct.Window.html#method.local_storage),
    /// The only difference is that, while data stored in `local_storage` has
    /// no expiration time, data stored in `session_storage` gets cleared when
    /// the browsing session ends.
    ///
    /// A page session lasts for as long as the browser is open and survives over
    /// page reloads and restores. Opening a page in a new tab or window will cause
    /// a new session to be initiated, which differs from how session cookies work.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
    pub fn session_storage( &self ) -> Storage {
        unsafe {
            js!(
                return @{self.as_ref()}.sessionStorage;
            ).into_reference_unchecked().unwrap()
        }
    }

    /// Returns a [Location](struct.Location.html) object which contains
    /// information about the URL of the document and provides methods
    /// for changing that URL and loading another URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/location)
    pub fn location( &self ) -> Option< Location > {
        unsafe {
            js!(
                return @{self}.location;
            ).into_reference_unchecked()
        }
    }
}

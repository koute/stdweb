use webcore::value::Reference;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::window_or_worker::IWindowOrWorker;
use webapi::storage::Storage;
use webapi::location::Location;
use webcore::once::Once;
use webcore::value::Value;

/// A handle to a pending animation frame request.
#[derive(Debug)]
pub struct RequestAnimationFrameHandle(Value);

impl RequestAnimationFrameHandle {
    /// Cancels an animation frame request.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame)
    pub fn cancel(self) {
        js!{
            var val = @{self.0};
            val.window.cancelAnimationFrame(val.request);
            val.callback.drop();
        };
    }
}

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

    /// You should call this method whenever you're ready to update your animation onscreen.
    /// This will request that your animation function be called before the browser performs the next repaint.
    /// The number of callbacks is usually 60 times per second, but will generally match the display refresh
    /// rate in most web browsers as per W3C recommendation. request_animation_frame() calls are paused in most browsers
    /// when running in background tabs or hidden iframes in order to improve performance and battery life.
    ///
    /// The callback method is passed a single argument, a f64, which indicates the current time when
    /// callbacks queued by requestAnimationFrame() begin to fire. Multiple callbacks in a single frame, therefore,
    /// each receive the same timestamp even though time has passed during the computation of every previous callback's workload.
    /// This timestamp is a decimal number, in milliseconds, but with a minimal precision of 1ms (1000 µs).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)
    pub fn request_animation_frame< F: FnOnce(f64) + 'static>( &self, callback: F) -> RequestAnimationFrameHandle {
        let values: Value = js!{
            var callback = @{Once(callback)};
            var request = @{self}.requestAnimationFrame(callback);
            return { request: request, callback: callback, window: @{self} };
        };
        RequestAnimationFrameHandle(values)
    }
}

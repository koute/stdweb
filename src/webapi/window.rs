use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::event_target::{IEventTarget, EventTarget};
use webapi::window_or_worker::IWindowOrWorker;
use webapi::storage::Storage;
use webapi::location::Location;
use webapi::history::History;
use webapi::selection::Selection;
use webcore::once::Once;
use webcore::value::Value;

/// A handle to a pending animation frame request.
#[derive(Debug)]
pub struct RequestAnimationFrameHandle(Value);

impl RequestAnimationFrameHandle {
    /// Cancels an animation frame request.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame)
    pub fn cancel( self ) {
        js! { @(no_return)
            var val = @{&self.0};
            val.window.cancelAnimationFrame(val.request);
            val.callback.drop();
        }
    }
}

/// The `Window` object represents a window containing a DOM document.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window)
// https://html.spec.whatwg.org/#window
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Window")]
#[reference(subclass_of(EventTarget))]
pub struct Window( Reference );

impl IEventTarget for Window {}
impl IWindowOrWorker for Window {}

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
    // https://html.spec.whatwg.org/#the-window-object:dom-alert
    pub fn alert( &self, message: &str ) {
        js!( @(no_return)
            @{self}.alert( @{message} );
        );
    }

    /// The Window.confirm() method displays a modal dialog
    /// with an optional message and two buttons: OK and Cancel.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    // https://html.spec.whatwg.org/#the-window-object:dom-confirm
    pub fn confirm( &self, message: &str ) -> bool {
        js!(
            return @{self}.confirm( @{message} );
        ).try_into().unwrap()
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
    // https://html.spec.whatwg.org/#the-localstorage-attribute:dom-localstorage
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
    // https://html.spec.whatwg.org/#the-sessionstorage-attribute:dom-sessionstorage
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
    // https://html.spec.whatwg.org/#the-window-object:dom-location
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
    // https://html.spec.whatwg.org/#the-window-object:dom-window-requestanimationframe
    pub fn request_animation_frame< F: FnOnce(f64) + 'static>( &self, callback: F) -> RequestAnimationFrameHandle {
        let values: Value = js!{
            var callback = @{Once(callback)};
            var request = @{self}.requestAnimationFrame(callback);
            return { request: request, callback: callback, window: @{self} };
        };
        RequestAnimationFrameHandle(values)
    }

    /// Returns the global [History](struct.History.html) object, which provides methods to
    /// manipulate the browser history.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/history)
    // https://html.spec.whatwg.org/#the-window-object:dom-history
    pub fn history(&self) -> History {
        unsafe {
            js!(
                return @{self}.history;
            ).into_reference_unchecked().unwrap()
        }
    }

    /// Returns the width (in pixels) of the browser window viewport including, if rendered,
    /// the vertical scrollbar.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/window/innerWidth)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-window-innerwidth
    pub fn inner_width(&self) -> i32 {
        js!(
            return @{self}.innerWidth;
        ).try_into().unwrap()
    }

    /// Returns the height (in pixels) of the browser window viewport including, if rendered,
    /// the horizontal scrollbar.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/window/innerHeight)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-window-innerheight
    pub fn inner_height(&self) -> i32 {
        js!(
            return @{self}.innerHeight;
        ).try_into().unwrap()
    }

    /// Returns the width of the outside of the browser window. It represents the width
    /// of the whole browser window including sidebar (if expanded), window chrome
    /// and window resizing borders/handles.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-window-outerheight
    pub fn outer_width(&self) -> i32 {
        js!(
            return @{self}.outerWidth;
        ).try_into().unwrap()
    }

    /// Returns the height of the outside of the browser window. It represents the height
    /// of the whole browser window including sidebar (if expanded), window chrome
    /// and window resizing borders/handles.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-window-outerheight
    pub fn outer_height(&self) -> i32 {
        js!(
            return @{self}.outerHeight;
        ).try_into().unwrap()
    }

    /// The read-only Window property pageYOffset is an alias for scrollY; as such, it returns
    /// the number of pixels the document is currently scrolled along the vertical axis (that is,
    /// up or down), with a value of 0.0 indicating that the top edge of the Document is currently
    /// aligned with the top edge of the window's content area.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageYOffset)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-window-pageyoffset
    pub fn page_y_offset(&self) -> f64 {
        js!(
            return @{self}.pageYOffset;
        ).try_into().unwrap()
    }

    /// This is an alias for scrollX.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageXOffset)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-window-pagexoffset
    pub fn page_x_offset(&self) -> f64 {
        js!(
            return @{self}.pageXOffset;
        ).try_into().unwrap()
    }

    /// The ratio in resolution from physical pixels to CSS pixels
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)
    // https://drafts.csswg.org/cssom-view/#ref-for-dom-window-devicepixelratio
    pub fn device_pixel_ratio(&self) -> f64 {
        js! (
            return @{self}.devicePixelRatio;
        ).try_into().unwrap()
    }

    /// Returns a [Selection](struct.Selection.html) object representing the range of text selected
    /// by the user or the current position of the caret.
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection)
    // https://w3c.github.io/selection-api/#dom-document-getselection
    pub fn get_selection(&self) -> Option<Selection> {
        unsafe {
            js!(
                return @{self}.getSelection();
            ).into_reference_unchecked()
        }
    }
}

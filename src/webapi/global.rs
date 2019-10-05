use webapi::window::window;
use webapi::window_or_worker::IWindowOrWorker;

/// An alias for [window.set_timeout](trait.IWindowOrWorker.html#method.set_timeout).
pub fn set_timeout< F: FnOnce() + 'static >( callback: F, timeout: u32 ) {
    window().set_timeout( callback, timeout );
}

/// An alias for [window.alert](struct.Window.html#method.alert).
pub fn alert( message: &str ) {
    window().alert( message );
}

/// An alias for [window.confirm](struct.Window.html#method.confirm).
pub fn confirm( message: &str ) -> bool {
    return window().confirm( message );
}

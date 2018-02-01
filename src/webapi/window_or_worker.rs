use webcore::value::Reference;

extern fn funcall_adapter< F: FnOnce() >( callback: *mut F ) {
    let callback = unsafe {
        Box::from_raw( callback )
    };

    callback();
}

/// The `IWindowOrWorker` mixin describes several features common to
/// the `Window` and the global scope of web workers.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope)
pub trait IWindowOrWorker: AsRef< Reference > {
    /// Sets a timer which executes a function once after the timer expires.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setTimeout)
    fn set_timeout< F: FnOnce() + 'static >( &self, callback: F, timeout: u32 ) {
        let callback = Box::into_raw( Box::new( callback ) );
        __js_raw_asm!( "\
            Module.STDWEB_PRIVATE.acquire_js_reference( $0 ).setTimeout( function() {\
                Module.STDWEB_PRIVATE.dyncall( 'vi', $1, [$2] );\
            }, $3 );\
        ", self.as_ref().as_raw(), funcall_adapter::< F > as extern fn( *mut F ), callback, timeout );
    }
}

use webcore::object::Object;
use webcore::once::Once;
use webcore::reference_type::ReferenceType;
use webcore::try_from::TryInto;

extern fn funcall_adapter< F: FnOnce() >( callback: *mut F ) {
    let callback = unsafe {
        Box::from_raw( callback )
    };

    callback();
}

/// The `IWindowOrWorker` mixin describes several features common to
/// the `Window` and the global scope of web workers.
///
/// You most likely don't want to `use` this directly; instead
/// you should `use stdweb::traits::*;`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope)
// https://html.spec.whatwg.org/#windoworworkerglobalscope
pub trait IWindowOrWorker: ReferenceType {
    /// Sets a timer which executes a function once after the timer expires.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setTimeout)
    // https://html.spec.whatwg.org/#windoworworkerglobalscope-mixin:dom-settimeout
    fn set_timeout< F: FnOnce() + 'static >( &self, callback: F, timeout: u32 ) {
        let callback = Box::into_raw( Box::new( callback ) );
        __js_raw_asm!( "\
            Module.STDWEB_PRIVATE.acquire_js_reference( $0 ).setTimeout( function() {\
                Module.STDWEB_PRIVATE.dyncall( 'vi', $1, [$2] );\
            }, $3 );\
        ", self.as_ref().as_raw(), funcall_adapter::< F > as extern fn( *mut F ), callback, timeout );
    }

    /// Sets a timer which executes a function once after the timer expires and can be cleared
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/setTimeout)
    // https://html.spec.whatwg.org/#windoworworkerglobalscope-mixin:dom-settimeout
    fn set_clearable_timeout< F: FnOnce() + 'static >( &self, callback: F, timeout: u32 ) -> TimeoutHandle {
        TimeoutHandle(js! (
            const callback = @{Once(callback)};
            return {
                id: setTimeout(callback, @{timeout}),
                callback
            };
        ).try_into().unwrap())
    }
}

#[derive(Debug)]
pub struct TimeoutHandle(Object);

impl TimeoutHandle {
    /// Clears a timer previously established by set_clearable_timeout
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/clearTimeout)
    // https://html.spec.whatwg.org/#windoworworkerglobalscope-mixin:dom-clear-timeout
     pub fn clear( & self ) {
        js! { @(no_return)
            clearTimeout(@{&self.0}.id);
            @{&self.0}.callback.drop();
        }
    }
}

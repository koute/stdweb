use std::panic;
use webcore::ffi;

/// Initializes the library.
///
/// Necessary **only** when compiling **without** `cargo-web`.
#[inline(never)]
#[cold]
pub fn initialize() {
    static mut INITIALIZED: bool = false;
    unsafe {
        if INITIALIZED {
            return;
        }

        INITIALIZED = true;
    }

    #[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))]
    ffi::initialize();

    #[cfg(not(feature = "docs-rs"))]
    stdweb_internal_runtime_initialize!( __js_raw_asm );

    if cfg!( test ) == false {
        panic::set_hook( Box::new( |info| {
            __js_raw_asm!( "console.error( 'Encountered a panic!' );" );
            if let Some( value ) = info.payload().downcast_ref::< String >() {
                __js_raw_asm!( "\
                    console.error( 'Panic error message:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) );\
                ", value.as_ptr(), value.len() );
            }
            if let Some( location ) = info.location() {
                let file = location.file();
                __js_raw_asm!( "\
                    console.error( 'Panic location:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) + ':' + $2 );\
                ", file.as_ptr(), file.len(), location.line() );
            }
        }));
    }
}

/// Runs Emscripten's event loop.
///
/// If you're compiling your project **without** using `cargo-web`
/// **and** you're using an Emscripten-based target (`asmjs-unknown-emscripten`,
/// or `wasm32-unknown-emscripten`) then calling this before returning
/// from `main()` is **mandatory** and will **not** return. (It is, effectively, an infinite loop.)
///
/// If you're using `cargo-web` to build your project then you never need to call this.
pub fn event_loop() {
    ffi::event_loop();
}

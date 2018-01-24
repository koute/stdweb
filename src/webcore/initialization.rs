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

    include!( concat!( env!( "OUT_DIR" ), "/runtime.rs" ) );

    if cfg!( test ) == false {
        panic::set_hook( Box::new( |info| {
            __js_raw_asm!( "console.error( 'Encountered a panic!' );" );
            if let Some( value ) = info.payload().downcast_ref::< String >() {
                __js_raw_asm!( "\
                    console.error( 'Panic error message:', Module.STDWEB.to_js_string( $0, $1 ) );\
                ", value.as_ptr(), value.len() );
            }
            if let Some( location ) = info.location() {
                let file = location.file();
                __js_raw_asm!( "\
                    console.error( 'Panic location:', Module.STDWEB.to_js_string( $0, $1 ) + ':' + $2 );\
                ", file.as_ptr(), file.len(), location.line() );
            }
        }));
    }
}

/// Runs the event loop.
///
/// You should call this before returning from `main()`,
/// otherwise bad things will happen.
///
/// On Emscripten-based targets (`asmjs-unknown-emscripten`,
/// `wasm32-unknown-emscripten`) calling this is **mandatory**
/// and will **not** return. (It is, effectively, an infinite loop.)
///
/// On Rust's native wasm target (`wasm32-unknown-unknown`)
/// calling this is not necessary and doesn't do anything.
pub fn event_loop() {
    ffi::event_loop();
}

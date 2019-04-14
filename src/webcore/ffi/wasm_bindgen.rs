use std::mem;
use wasm_bindgen::prelude::*;

use webcore::initialization::initialize as initialize_crate;

pub fn event_loop() {
}

fn alloc( size: usize ) -> *mut u8 {
    let mut buffer = Vec::with_capacity( size );
    let ptr = buffer.as_mut_ptr();
    mem::forget( buffer );
    ptr
}

fn free( ptr: *mut u8, capacity: usize ) {
    unsafe  {
        let _ = Vec::from_raw_parts( ptr, 0, capacity );
    }
}

pub unsafe fn dealloc( ptr: *mut u8, capacity: usize ) {
    free( ptr, capacity )
}

struct Module( Option< JsValue > );
unsafe impl Sync for Module {}

static mut MODULE: Module = Module( None );

pub fn initialize() {
    #[wasm_bindgen(inline_js = r#"export function wasm_bindgen_initialize( memory, alloc, free ) {
        var Module = {};
        Module.web_malloc = alloc;
        Module.web_free = free;
        Module.web_table = null;
        Object.defineProperty( Module, "HEAP8", {
            get: function() { return new Int8Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAP16", {
            get: function() { return new Int16Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAP32", {
            get: function() { return new Int32Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPU8", {
            get: function() { return new Uint8Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPU16", {
            get: function() { return new Uint16Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPU32", {
            get: function() { return new Uint32Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPF32", {
            get: function() { return new Float32Array( memory.buffer ); }
        });
        Object.defineProperty( Module, "HEAPF64", {
            get: function() { return new Float64Array( memory.buffer ); }
        });
        return Module;
    }"#)]
    extern "C" {
        fn wasm_bindgen_initialize(
            memory: JsValue,
            alloc: &dyn Fn( usize ) -> *mut u8,
            free: &dyn Fn( *mut u8, usize )
        ) -> JsValue;
    }

    let memory = wasm_bindgen::memory();
    unsafe {
        let module = wasm_bindgen_initialize( memory, &alloc, &free );
        MODULE = Module( Some( module ) );
    }
}

#[doc(hidden)]
pub fn get_module() -> JsValue {
    initialize_crate();

    unsafe {
        MODULE.0.as_ref().unwrap().clone()
    }
}

pub mod exports {}

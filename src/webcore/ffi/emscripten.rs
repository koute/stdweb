pub type CallbackFn = Option< unsafe extern "C" fn() >;

extern "C" {
    fn free( ptr: *mut u8 );
    fn emscripten_pause_main_loop();
    fn emscripten_set_main_loop( callback: CallbackFn, fps: i32, simulate_infinite_loop: i32 );
}

pub fn event_loop() {
    unsafe {
        emscripten_set_main_loop( Some( emscripten_pause_main_loop ), 0, 1 );
    }
}

pub unsafe fn dealloc( ptr: *mut u8, _: usize ) {
    free( ptr );
}

pub mod exports {
    extern "C" {
        pub fn emscripten_asm_const_int( code: *const u8, ... ) -> i32;
    }
}

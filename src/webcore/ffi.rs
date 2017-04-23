pub type CallbackFn = Option< unsafe extern "C" fn() >;

extern "C" {
    pub fn free( pointer: *const u8 );
    pub fn emscripten_asm_const_int( code: *const u8, ... ) -> i32;
    pub fn emscripten_asm_const_double( code: *const u8, ... ) -> f64;
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_set_main_loop( callback: CallbackFn, fps: i32, simulate_infinite_loop: i32 );
}

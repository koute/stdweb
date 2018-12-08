pub fn event_loop() {
}

pub unsafe fn dealloc( ptr: *mut u8, capacity: usize ) {
    exports::__web_free( ptr, capacity );
}

pub mod exports {
    use std::mem;

    #[doc(hidden)]
    #[no_mangle]
    pub extern "C" fn __web_malloc( size: usize ) -> *mut u8 {
        let mut buffer = Vec::with_capacity( size );
        let ptr = buffer.as_mut_ptr();
        mem::forget( buffer );
        ptr
    }

    #[doc(hidden)]
    #[no_mangle]
    pub extern "C" fn __web_free( ptr: *mut u8, capacity: usize ) {
        unsafe  {
            let _ = Vec::from_raw_parts( ptr, 0, capacity );
        }
    }
}

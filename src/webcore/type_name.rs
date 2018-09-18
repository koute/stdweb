#[cfg(rust_nightly)]
pub fn type_name< T >() -> &'static str {
    use std::intrinsics;
    unsafe {
        intrinsics::type_name::< T >()
    }
}

#[cfg(not(rust_nightly))]
pub fn type_name< T >() -> &'static str {
    "<Rust type (compile with Rust nightly to see the actual type)>"
}

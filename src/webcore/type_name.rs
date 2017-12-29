#[cfg(feature = "nightly")]
pub fn type_name< T >() -> &'static str {
    use std::intrinsics;
    unsafe {
        intrinsics::type_name::< T >()
    }
}

#[cfg(not(feature = "nightly"))]
pub fn type_name< T >() -> &'static str {
    "<Rust type>"
}

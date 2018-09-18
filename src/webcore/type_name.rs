#[cfg(rust_nightly)]
pub fn type_name_opt< T >() -> Option< &'static str > {
    use std::intrinsics;
    let name = unsafe {
        intrinsics::type_name::< T >()
    };

    Some( name )
}

#[cfg(not(rust_nightly))]
pub fn type_name_opt< T >() -> Option< &'static str > {
    None
}

pub fn type_name< T >() -> &'static str {
    type_name_opt::< T >().unwrap_or( "<Rust type (compile with Rust nightly to see the actual type)>" )
}

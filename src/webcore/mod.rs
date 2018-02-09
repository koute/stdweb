#[macro_use]
pub mod macros;
pub mod initialization;
pub mod value;
pub mod number;
pub mod serialization;
pub mod ffi;
pub mod callfn;
pub mod newtype;
pub mod try_from;
pub mod object;
pub mod array;
pub mod type_name;
pub mod unsafe_typed_array;
pub mod once;

#[cfg(feature = "nightly")]
pub mod void {
    pub type Void = !;
}

#[cfg(not(feature = "nightly"))]
pub mod void;

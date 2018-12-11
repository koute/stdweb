#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod test_js_export;
mod test_derive_reference_type;
mod test_misc;

pub mod exports {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub use super::test_js_export::exports::*;
}

pub fn run_all_tests() {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    test_js_export::run();
    test_derive_reference_type::run();
    test_misc::run();
}

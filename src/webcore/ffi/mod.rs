#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] mod emscripten;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] pub use self::emscripten::*;

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", cargo_web))] mod wasm;
#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", cargo_web))] pub use self::wasm::*;

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))] mod wasm_bindgen;
#[cfg(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", not(cargo_web)))] pub use self::wasm_bindgen::*;

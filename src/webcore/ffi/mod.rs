#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] mod emscripten;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] pub use self::emscripten::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown", not(feature = "wasm-bindgen")))] mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown", not(feature = "wasm-bindgen")))] pub use self::wasm::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "wasm-bindgen"))] mod wasm_bindgen;
#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "wasm-bindgen"))] pub use self::wasm_bindgen::*;

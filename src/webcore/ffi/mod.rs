#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] mod emscripten;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))] pub use self::emscripten::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))] mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))] pub use self::wasm::*;

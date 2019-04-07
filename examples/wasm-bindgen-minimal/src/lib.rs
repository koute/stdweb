#[macro_use]
extern crate stdweb;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let message = "Hello, 世界!";
    js! {
        alert( @{message} );
    }

    Ok(())
}

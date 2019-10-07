use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn test_document() {
    stdweb::web::document();
}
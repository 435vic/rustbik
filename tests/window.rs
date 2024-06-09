use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn has_window() {
    assert_ne!(web_sys::window(), None);
}


use wasm_bindgen::prelude::*;

// This is the function we're going to call from JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

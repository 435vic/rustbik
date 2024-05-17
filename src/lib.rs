pub mod rubik;
pub mod canvas;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(typescript_custom_section)]
const CANVAS_OPTS: &'static str = r#"
interface CanvasOptions {
    alpha: bool,
    antialias: bool,
    depth: bool,
    stencil: bool,
    premultipliedAlpha: bool,
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "CanvasOptions")]
    pub type CanvasOptions;
}

pub(crate) fn ease(t: f32, a: f32) -> f32 {
    t.powf(a) / (t.powf(a) + (1.0 - t).powf(a))
}

#[wasm_bindgen]
pub fn bind(canvas_element: HtmlCanvasElement, opts: Option<CanvasOptions>) -> Result<(), JsValue> {
    let _window = canvas::Canvas::new(canvas_element, opts.map(|o| o.into()))
        .map_err(|e| JsValue::from(e))?;
    Err(JsValue::from_str("Testing errors! :D"))
}

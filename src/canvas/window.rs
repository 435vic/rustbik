use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("Global window object should exist")
}

pub fn performance() -> web_sys::Performance {
    window()
        .performance()
        .expect("Performance should exist")
}

pub fn scale_factor() -> f64 {
    window().device_pixel_ratio()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

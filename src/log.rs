#[macro_export]
macro_rules! info {
    ($($t:tt)*) => { 
        #[cfg(feature = "debug")]
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format_args!($($t)*).to_string()))
    }
}

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => { 
        #[cfg(feature = "debug")]
        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&format_args!($($t)*).to_string()))
    }
}

#[macro_export]
macro_rules! debug {
    ($($t:tt)*) => { 
        #[cfg(feature = "debug")]
        web_sys::console::debug_1(&wasm_bindgen::JsValue::from_str(&format_args!($($t)*).to_string()))
    }
}

#[macro_export]
macro_rules! trace {
    ($($t:tt)*) => { 
        #[cfg(feature = "debug")]
        web_sys::console::trace_1(&wasm_bindgen::JsValue::from_str(&format_args!($($t)*).to_string()))
    }
}

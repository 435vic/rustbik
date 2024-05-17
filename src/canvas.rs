use std::sync::Arc;

use three_d::{Context, context};
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::JsCast;

/// A WebGL2 wrapper for a canvas element.
pub struct Canvas {
    canvas: HtmlCanvasElement,
    context: Context,
}

impl Canvas {
    /// Create a new Canvas with the underlying element and WebGL2 options.
    /// Options are not type checked.
    pub fn new(canvas: HtmlCanvasElement, opts: Option<JsValue>) -> Result<Self, String> {
        let wglctx = if let Some(options) = opts {
            canvas.get_context_with_context_options("webgl2", &options)
        } else {
            canvas.get_context("webgl2")
        };
        let wglctx = wglctx
            .map_err(|e| format!("webgl2 not supported: {:?}", e))?
            .ok_or("webgl2 not supported".to_string())?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|e| format!("failed to get webgl context: {:?}", e))?;
        // Check for WebGL 2 extension support
        wglctx
            .get_extension("EXT_color_buffer_float")
            .map_err(|e| format!("extension EXT_color_buffer_float not available: {:?}", e))?;
        wglctx
            .get_extension("OES_texture_float_linear")
            .map_err(|e| format!("extension OES_texture_float_linear not available: {:?}", e))?;
        wglctx
            .get_extension("OES_texture_half_float_linear")
            .map_err(|e| format!("extension OES_texture_half_float_linear not available: {:?}", e))?;
        
        Ok(
            Self {
                canvas,
                context: Context::from_gl_context(Arc::new(
                    context::Context::from_webgl2_context(wglctx),
                )).map_err(|e| format!("three_d::core failed to create context: {:?}", e))?
            }
        )
    }
}

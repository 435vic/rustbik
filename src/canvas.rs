use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use three_d::{context, Context, Viewport};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, Performance, WebGl2RenderingContext};
use wasm_bindgen::JsCast;

fn request_animation_frame(closure: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("Global window object should exist")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
}

fn performance() -> Performance {
    web_sys::window()
       .expect("Global window object should exist")
       .performance()
       .expect("Performance should exist")
}

pub struct ProgramInput {
    pub frame_time: f64,
    pub time: f64,
}

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

    pub fn gl(&self) -> Context {
        self.context.clone()
    }

    pub fn logical_size(&self) -> (u32, u32) {
        (self.canvas.client_width() as u32, self.canvas.client_height() as u32)
    }

    pub fn viewport(&self) -> Viewport {
        let (w, h) = self.logical_size();
        Viewport::new_at_origo(w, h)
    }

    pub fn run(&self, mut program: impl 'static + FnMut(ProgramInput)) {
        let closure = Rc::new(RefCell::new(None));
        let closure2 = closure.clone();

        let mut elapsed_time = 0.0;
        let mut last_time = 0.0;
        *closure2.borrow_mut() = Some(Closure::new(move || {
            // TODO: add exit condition
            // Need to drop handle on closure with closure.borrow_mut.take()
            let frame_time = performance().now() - last_time;
            elapsed_time += frame_time;
            last_time = performance().now();
            program(ProgramInput {
                time: elapsed_time,
                frame_time
            });
            request_animation_frame(closure.borrow().as_ref().unwrap());
        }));

        request_animation_frame(closure2.borrow().as_ref().unwrap());
    }
}

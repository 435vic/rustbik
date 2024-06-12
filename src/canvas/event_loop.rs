use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::mpsc::{self, Sender, Receiver};

use three_d::Viewport;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Object;
use web_sys::{EventTarget, HtmlCanvasElement, ResizeObserverEntry, ResizeObserverSize};

use super::window::window;
use super::{performance, request_animation_frame, Canvas};
use crate::{info, debug, error, trace};

#[derive(Debug)]
pub enum CanvasEvent {
    PageScroll(f32),
    Resize(u32, u32),
    // TODO: pointer events (and maybe keyboard??)
}

/// Represents the input to a program.
pub struct ProgramInput {
    /// Milliseconds elapsed since the last frame.
    pub frame_time: f64,
    /// Milliseconds elapsed since the program started.
    pub time: f64,
    /// The current viewport.
    viewport: Viewport,
    /// The current canvas size in physical pixels.
    /// On non-Safari browsers, this is the same as the logical size.
    size: (u32, u32),
}

pub struct EventLoop {
    pub canvas: Canvas,
    sender: Sender<CanvasEvent>,
    receiver: Receiver<CanvasEvent>,
    elapsed_time: f64,
    last_frame_time: f64,
    viewport: Viewport,
    size: (u32, u32),
}

impl EventLoop {
    pub fn new(canvas: HtmlCanvasElement, opts: Option<JsValue>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let canvas = Canvas::new(canvas, opts).expect("Should create canvas");
        let size = canvas.logical_size();
        let viewport = canvas.viewport();
        let event_loop = Self {
            canvas,
            sender,
            receiver,
            size,
            viewport,
            elapsed_time: 0.0,
            last_frame_time: 0.0,
        };
        // TODO: register event handlers

        event_loop.register_events();

        event_loop
    }

    pub fn run(mut self, mut program: impl 'static + FnMut(ProgramInput)) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let frame_time = performance().now() - self.last_frame_time;
            self.elapsed_time += frame_time;
            self.last_frame_time = performance().now();

            let event = match self.receiver.try_recv() {
                Ok(event) => {
                    debug!("Event: {:?}", event);
                    Some(event)
                },
                Err(mpsc::TryRecvError::Empty) => None,
                Err(mpsc::TryRecvError::Disconnected) => {
                    panic!("EventLoop receiver disconnected");
                }
            };

            
            let input = ProgramInput {
                frame_time,
                time: self.elapsed_time,
                viewport: self.viewport,
                size: self.size,
            };
            program(input);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn register_events(&self) {
        let sender = self.sender.clone();
        EventLoop::add_event("scroll", move |_event: web_sys::Event| {
            sender.send(
                CanvasEvent::PageScroll(window().scroll_y().unwrap() as f32)
            ).unwrap();
        });

        let sender = self.sender.clone();
        // We're leaving all the ResizeObserver logic up to the javascript side of things.
        // This way we don't need to deal with creating the observer here.
        EventLoop::add_event("rustbik_resize", move |event: web_sys::CustomEvent| {
            let detail = event.detail();
            // Passing the whole entry as the detail property allows us to skip using Reflect
            // to get custom properties.
            let entry: ResizeObserverEntry = detail.dyn_into().expect("Should be a ResizeObserverEntry");
            if !has_device_pixel_support() {
                let rect = entry.content_rect();

                sender.send(
                    CanvasEvent::Resize(rect.width() as u32, rect.height() as u32)
                ).unwrap();
                return;
            }

            let entry: ResizeObserverSize = entry.device_pixel_content_box_size().get(0).unchecked_into();
            sender.send(
                CanvasEvent::Resize(entry.inline_size() as u32, entry.block_size() as u32)
            ).unwrap();
        });
    }

    // We'd need to use an Rc if we want to add an event listener to the canvas.
    // Since there's only one canvas and it takes up the whole window, I think it's fine to
    // use the window's events instead.
    fn add_event<E, F> (name: &'static str, handler: F) where
        F: 'static + FnMut(E),
        E: 'static + AsRef<web_sys::Event> + wasm_bindgen::convert::FromWasmAbi
    {
        let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(E)>);
        window()
            .add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
            .expect("should add event listener");
        closure.forget();
    }
}

// https://github.com/rust-windowing/winit/blob/master/src/platform_impl/web/web_sys/resize_scaling.rs#L285
// The neat thing about this implementation is that it doesn't require any additional libraries or features.
/// This function lazily checks if the browser supports the devicePixelContextBoxSize property.
fn has_device_pixel_support() -> bool {
    thread_local! {
        static DEVICE_PIXEL_SUPPORT: bool = {
            #[wasm_bindgen]
            extern "C" {
                type ResizeObserverEntryExt;

                #[wasm_bindgen(js_class = ResizeObserverEntry, static_method_of = ResizeObserverEntryExt, getter)]
                fn prototype() -> Object;
            }

            let prototype = ResizeObserverEntryExt::prototype();
            let descriptor = Object::get_own_property_descriptor(
                &prototype,
                &JsValue::from_str("devicePixelContextBoxSize"),
            );
            !descriptor.is_undefined()
        }
    }

    DEVICE_PIXEL_SUPPORT.with(|support| *support)
}

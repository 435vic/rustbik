pub mod rubik;
pub mod canvas;
pub mod log;
mod animation;

use std::borrow::BorrowMut;

use animation::{SecondOrderSystem, SecondOrderSystemParameters};
use canvas::window::scale_factor;
use rubik::{CubeAnimationOptions, Move};
use canvas::event_loop::{EventLoop, CanvasEvent};
use three_d::{Camera, ClearState, ColorMaterial, DirectionalLight, PhysicalMaterial, RenderTarget, Srgba, Vec3, Viewport};
use wasm_bindgen::prelude::*;
use web_sys::console::debug;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(typescript_custom_section)]
const CANVAS_OPTS: &'static str = r#"
interface CanvasOptions {
    alpha?: bool,
    antialias?: bool,
    depth?: bool,
    stencil?: bool,
    premultipliedAlpha?: bool,
    failIfMajorPerformanceCaveat?: bool,
    desynchronized?: bool,
    powerPreference?: "default" | "high-performance" | "low-power",
    preserveDrawingBuffer?: bool,
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
pub async fn bind(canvas_element: HtmlCanvasElement, opts: Option<CanvasOptions>) -> Result<(), JsValue> {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();

    let window = EventLoop::new(canvas_element, opts.map(|o| o.into()));
    let (width, height) = window.canvas.logical_size();
    let context = window.canvas.gl();

    let cube_pos_wide = Vec3::new(0.0, 0.0, -3.0);
    let cube_pos_narrow = Vec3::new(-2.0, 3.0, -1.2);

    let mut smooth_scroll = SecondOrderSystem::new(
        SecondOrderSystemParameters {
            freq: 1.0,
            zeta: 0.8,
            r: 0.5,
        },
        0.0,
    );

    let mut cube = rubik::Cube::solved(&context, CubeAnimationOptions::default());
    cube.set_translation(cube_pos_wide);
    cube.queue(Move::from_sequence("U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2").unwrap());
    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(width, height),
        Vec3::new(8.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        three_d::degrees(45.0),
        0.1,
        50.0,
    );
    let light = DirectionalLight::new(&context, 100.0, Srgba::WHITE, &Vec3::new(1.0, 3.0, 2.5));

    let mut phone = (window.canvas.logical_size().0 as f64 / scale_factor()) <= 480.0;
    let mut scroll_y = 0.0;

    cube.set_translation(if phone { cube_pos_narrow } else { cube_pos_wide });
    cube.rotate(1.0, 1.0);

    window.run(move |input| {
        for event in input.events {
            match event {
                CanvasEvent::Resize(width, _) => {
                    let css_width = (width as f64 / scale_factor()) as u32;
                    phone = css_width <= 480;
                    cube.set_translation(if phone { cube_pos_narrow } else { cube_pos_wide });
                },
                CanvasEvent::PageScroll(pos) => {
                    scroll_y = pos;
                },
            }
        }

        let t = input.time as f32;
        let dt = input.frame_time as f32;

        smooth_scroll.update(dt / 1000.0, scroll_y);
        cube.set_spacing({
            let capped_zoom = (smooth_scroll.value() / 60.0).max(0.0);
            let thresh = 2.8;
            if capped_zoom < thresh {
                capped_zoom
            } else {
                (capped_zoom - thresh).exp() + thresh - 1.0
            }
        });

        let theta_speed = (t/10000.0).sin()/8000.0;
        let phi_speed = (t/10000.0).cos()/6000.0;
        // let phi_speed = 0.0;
        cube.rotate(dt*theta_speed, dt*phi_speed);

        // rotate_camera_around_target(&mut camera, Vec3::zero(), dt*theta_speed, dt*phi_speed);

        cube.animate(t);

        RenderTarget::screen(&context, width, height)
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0))
            .render(&camera, &cube, &[&light]);

        // Scroll view with page
        camera.set_view(
            Vec3::new(8.0, -scroll_y / 130.0, 0.0),
            Vec3::new(0.0, -scroll_y / 130.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        camera.set_viewport(input.viewport);
    });

    Ok(())
}

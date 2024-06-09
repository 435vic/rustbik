pub mod rubik;
pub mod canvas;

use rubik::CubeAnimationOptions;
use three_d::{Angle, Camera, ClearState, DirectionalLight, InnerSpace, Quaternion, Rad, RenderTarget, Srgba, Vec3, Viewport, Zero};
use wasm_bindgen::prelude::*;
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

/// Rotate the camera around a given point by two angles (in radians),
/// keeping the camera facing the point.
pub fn rotate_camera_around_target(
    camera: &mut Camera,
    target: Vec3,
    theta: f32,
    phi: f32,
) {
    let distance = (target - camera.position()).magnitude();
    let dir = (target - camera.position()).normalize();
    let horizontal = dir.cross(*camera.up());
    let vertical = horizontal.cross(dir);

    for i in 0..2 {
        let axis = if i == 0 { vertical } else { horizontal };
        let angle = if i == 0 { theta } else { phi };
        let new_position = rotate_around_axis(*camera.position(), axis, angle);
        camera.set_view(new_position * distance, target, vertical.normalize());
    }
}

/// Rotate a vector around an axis by a given angle in radians.
pub fn rotate_around_axis(vector: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let angle = Rad(angle / 2.0);
    let q = Quaternion::from_sv(angle.cos(), angle.sin() * axis).normalize();
    let p = Quaternion::from_sv(0.0, vector).normalize();
    let rotated = (q * p * q.conjugate()).v;
    rotated
}

#[wasm_bindgen]
pub fn bind(canvas_element: HtmlCanvasElement, opts: Option<CanvasOptions>) -> Result<(), JsValue> {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();

    let window = canvas::Canvas::new(canvas_element, opts.map(|o| o.into()))
        .map_err(|e| JsValue::from(e))?;
    let (width, height) = window.logical_size();
    let context = window.gl();

    let cube = rubik::Cube::solved(&context, CubeAnimationOptions::default());
    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(width, height),
        Vec3::new(5.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        three_d::degrees(45.0),
        0.1,
        50.0,
    );
    let light = DirectionalLight::new(&context, 100.0, Srgba::WHITE, &Vec3::new(1.0, 3.0, 2.5));

    window.run(move |input| {
        let t = input.time as f32;
        let dt = input.frame_time as f32;

        let theta_speed = (t/10000.0).sin()/2000.0;
        let phi_speed = (t/10000.0).cos()/8000.0;
        rotate_camera_around_target(&mut camera, Vec3::zero(), dt*theta_speed, dt*phi_speed);

        RenderTarget::screen(&context, width, height)
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0))
            .render(&camera, &cube, &[&light]);
    });

    Ok(())
}

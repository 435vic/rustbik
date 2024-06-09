use three_d::*;

pub fn cube_uvs() -> Vec<Vec2> {vec![
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 1.0),

    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(0.0, 1.0),

    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 0.0),

    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, 0.0),

    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),

    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 0.0),
]}

#[derive(Clone)]
pub struct RubikMaterial {
    pub margin: f32,
    pub radius: f32,
    pub render_states: RenderStates,
}

impl Default for RubikMaterial {
    fn default() -> Self {
        Self {
            margin: 0.2,
            radius: 0.1,
            render_states: RenderStates::default(),
        }
    }
}

impl Material for RubikMaterial {
    fn id(&self) -> u16 {
        0u16
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("rubik.frag").to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            color: true,
            // TODO: Switch to true when using lights
            normal: false,
            uv: true,
            position: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, _: &[&dyn Light]) {
        // camera.color_mapping.use_uniforms(program);
        program.use_uniform("stickerData", Vec2::new(1.0 - self.margin, self.radius));
        // program.use_uniform("cameraPosition", camera.position());

    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

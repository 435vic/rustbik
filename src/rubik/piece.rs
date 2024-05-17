use three_d::{Geometry, Gm, Mat3, Mat4, Mesh, Object, SquareMatrix, Vec3};

use super::graphics::RubikMaterial;
use super::Color;

pub type PieceMaterial = RubikMaterial;

/// A single rubik's cube piece. There are 27 in a cube.
pub struct Piece {
    position: (i32, i32, i32),
    color: (Color, Color, Color),
    pub(crate) transform: Mat4,
    gm: Gm<Mesh, PieceMaterial>
}

impl Piece {
    pub fn new(position: (i32, i32, i32), color: (Color, Color, Color), mesh: Mesh) -> Piece {
        Piece {
            position,
            color,
            transform: Mat4::identity(),
            gm: Gm::new(mesh, PieceMaterial::default())
        }
    }

    pub fn vec(&self) -> Vec3 {
        Vec3::new(self.position.0 as f32, self.position.1 as f32, self.position.2 as f32)
    }

    pub fn cubelet(&self) -> usize {
        let out = ((self.position.0 + 1) * 9 + (1 - self.position.1) * 3 + (self.position.2 + 1)).try_into().unwrap();
        out
    }

    pub fn rotate(&mut self, mat: Mat3) -> Result<(Vec3, Vec3), String> {
        let prev = self.vec();
        let new = mat * prev;
        let mut rot = new - prev;
        let unchanged_axes = (0..3).filter(|&i| rot[i] == 0.0).collect::<Vec<_>>();
        if unchanged_axes.len() == 3 {
            return Ok((prev, new));
        } else if unchanged_axes.len() == 2 {
            rot += mat * rot;
        }

        if (0..3).filter(|&i| rot[i] == 0.0).count() != 1 {
            return Err(format!("before: {:?}, after: {:?}, rot:{:?}", prev, new, rot));
        }
        let diff = (0..3).into_iter().filter(|&i| rot[i] != 0.0).collect::<Vec<_>>();
        let mut color_arr = [self.color.0, self.color.1, self.color.2];
        color_arr.swap(diff[0], diff[1]);
        self.color = (color_arr[0], color_arr[1], color_arr[2]);
        self.position = (new.x as i32, new.y as i32, new.z as i32);
        Ok((prev, new))
    }

    pub fn transform(&mut self, mat: Mat4) {
        self.gm.set_transformation(mat * self.transform);
    }
}

impl Geometry for Piece {
    fn aabb(&self) -> three_d::prelude::AxisAlignedBoundingBox {
        self.gm.aabb()
    }

    fn animate(&mut self, _time: f32) {
        self.gm.animate(_time);
    }

    fn draw(
            &self,
            camera: &three_d::Camera,
            program: &three_d::Program,
            render_states: three_d::RenderStates,
            attributes: three_d::FragmentAttributes,
        ) {
        self.gm.draw(camera, program, render_states, attributes);
    }

    fn id(&self, required_attributes: three_d::FragmentAttributes) -> u16 {
        self.gm.id(required_attributes)
    }

    fn render_with_effect(
            &self,
            material: &dyn three_d::Effect,
            camera: &three_d::Camera,
            lights: &[&dyn three_d::Light],
            color_texture: Option<three_d::ColorTexture>,
            depth_texture: Option<three_d::DepthTexture>,
        ) {
        self.gm.render_with_effect(material, camera, lights, color_texture, depth_texture)
    }

    fn render_with_material(&self, material: &dyn three_d::Material, camera: &three_d::Camera, lights: &[&dyn three_d::Light]) {
        self.gm.render_with_material(material, camera, lights);
    }

    fn vertex_shader_source(&self, required_attributes: three_d::FragmentAttributes) -> String {
        self.gm.vertex_shader_source(required_attributes)
    }
}

impl Object for Piece {
    fn material_type(&self) -> three_d::MaterialType {
        self.gm.material_type()
    }

    fn render(&self, camera: &three_d::Camera, lights: &[&dyn three_d::Light]) {
        self.gm.render(camera, lights);
    }
}

impl<'a> IntoIterator for &'a Piece {
    type Item = &'a dyn Object;
    type IntoIter = std::vec::IntoIter<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self as &dyn Object].into_iter()
    }
}

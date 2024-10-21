use glam::{vec3, Vec3};

use crate::{
    gl::Gl,
    shader::{Shader, ShaderTrait},
};

const AMBIENT_STRENGTH_DEFAULT: Vec3 = vec3(0.2, 0.2, 0.2);
const DIFFUSE_STRENGTH_DEFAULT: Vec3 = vec3(0.5, 0.5, 0.5);
const SPECULAR_STRENGTH_DEFAULT: Vec3 = vec3(1.0, 1.0, 1.0);

pub struct DirectionalLight {
    direction: Vec3,
}

impl DirectionalLight {
    pub fn new(dir: Vec3) -> Self {
        Self { direction: dir }
    }
    pub fn sync(&self, gl: &Gl, shader: &Shader) {
        shader
            .set_vec3(gl, "light.direction", self.direction.into())
            .unwrap();
        shader
            .set_vec3(gl, "light.ambient", AMBIENT_STRENGTH_DEFAULT.into())
            .unwrap();
        shader
            .set_vec3(gl, "light.diffuse", DIFFUSE_STRENGTH_DEFAULT.into())
            .unwrap();
        shader
            .set_vec3(gl, "light.specular", SPECULAR_STRENGTH_DEFAULT.into())
            .unwrap();
    }
}
impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            direction: vec3(-0.2, -1.0, -0.3),
        }
    }
}

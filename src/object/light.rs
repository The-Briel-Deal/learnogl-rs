use std::rc::Rc;

use glam::{vec3, Mat4, Vec3};

use crate::{
    gl::{types::GLfloat, Gl},
    mesh::{Mesh, VertexBuffer},
    shader::{Shader, ShaderTrait},
};

const POSITION_DEFAULT: Vec3 = vec3(0.0, 2.0, 0.0);
const DIRECTION_DEFAULT: Vec3 = vec3(0.0, 0.0, -1.0);
const CUTOFF_DEFAULT: f32 = 12.5;

const AMBIENT_STRENGTH_DEFAULT: Vec3 = vec3(0.2, 0.2, 0.2);
const DIFFUSE_STRENGTH_DEFAULT: Vec3 = vec3(0.5, 0.5, 0.5);
const SPECULAR_STRENGTH_DEFAULT: Vec3 = vec3(1.0, 1.0, 1.0);

const ATTENUATION_CONSTANT_DEFAULT: f32 = 1.0;
const ATTENUATION_LINEAR_DEFAULT: f32 = 0.09;
const ATTENUATION_QUADRATIC_DEFAULT: f32 = 0.032;

#[derive(Debug)]
pub struct LightAttributes {
    pub position: Vec3,
    pub direction: Vec3,
    pub cutoff: f32,

    // Strength of each type of lighting
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,

    // Attenuation
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl Default for LightAttributes {
    fn default() -> Self {
        Self {
            position: POSITION_DEFAULT,
            direction: DIRECTION_DEFAULT,
            cutoff: CUTOFF_DEFAULT,

            // Strength of each type of lighting
            ambient: AMBIENT_STRENGTH_DEFAULT,
            diffuse: DIFFUSE_STRENGTH_DEFAULT,
            specular: SPECULAR_STRENGTH_DEFAULT,

            // Attenuation
            constant: ATTENUATION_CONSTANT_DEFAULT,
            linear: ATTENUATION_LINEAR_DEFAULT,
            quadratic: ATTENUATION_QUADRATIC_DEFAULT,
        }
    }
}
pub trait Light {
    fn pos(&self) -> Vec3;
    fn set_pos(&mut self, gl: &Gl, pos: Vec3);

    fn dir(&self) -> Vec3;
    fn set_dir(&mut self, gl: &Gl, dir: Vec3);

    fn draw(&self, gl: &Gl, view_matrix: Mat4) {}
    fn adjust_zoom(&mut self, degrees: GLfloat) {}
}

pub struct FlashLight {
    pub lit_object_shader: Rc<Shader>,
    attrs: LightAttributes,
}

impl Light for FlashLight {
    fn pos(&self) -> Vec3 {
        self.attrs.position
    }
    fn set_pos(&mut self, gl: &Gl, pos: Vec3) {
        self.attrs.position = pos;
        self.sync_state(gl);
    }

    fn dir(&self) -> Vec3 {
        self.attrs.direction
    }
    fn set_dir(&mut self, gl: &Gl, dir: Vec3) {
        self.attrs.direction = dir;
        self.sync_state(gl);
    }
}

impl FlashLight {
    /// Create a new light source. Leave attrs as None for default values.
    pub fn new(gl: &Gl, attrs: Option<LightAttributes>, lit_object_shader: Rc<Shader>) -> Self {
        let attrs = attrs.unwrap_or_default();

        let mut light = Self {
            lit_object_shader,
            attrs,
        };
        light.sync_state(gl);
        light
    }

    pub fn set_attrs(&mut self, gl: &Gl, attrs: LightAttributes) {
        self.attrs = attrs;
        self.sync_state(gl);
    }

    pub fn attrs(&self) -> &LightAttributes {
        &self.attrs
    }
    pub fn sync_state(&mut self, gl: &Gl) {
        self.lit_object_shader
            .set_vec3(gl, "light.position", self.attrs.position.into())
            .unwrap();
        self.lit_object_shader
            .set_vec3(gl, "light.direction", self.attrs.direction.into())
            .unwrap();
        self.lit_object_shader
            .set_float(gl, "light.cutOff", self.attrs.cutoff.to_radians().cos())
            .unwrap();

        self.lit_object_shader
            .set_vec3(gl, "light.ambient", self.attrs.ambient.into())
            .unwrap();
        self.lit_object_shader
            .set_vec3(gl, "light.diffuse", self.attrs.diffuse.into())
            .unwrap();
        self.lit_object_shader
            .set_vec3(gl, "light.specular", self.attrs.specular.into())
            .unwrap();

        self.lit_object_shader
            .set_float(gl, "light.constant", self.attrs.constant)
            .unwrap();
        self.lit_object_shader
            .set_float(gl, "light.linear", self.attrs.linear)
            .unwrap();
        self.lit_object_shader
            .set_float(gl, "light.quadratic", self.attrs.quadratic)
            .unwrap();
    }
}

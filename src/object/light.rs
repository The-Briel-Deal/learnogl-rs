use std::rc::Rc;

use glam::{vec3, Mat4, Vec3};

use crate::{
    gl::{types::GLfloat, Gl},
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

struct LightAttributes {
    bound_shader: Rc<Shader>,

    position: Vec3,
    direction: Vec3,
    cutoff: f32,

    // Strength of each type of lighting
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,

    // Attenuation
    constant: f32,
    linear: f32,
    quadratic: f32,
}

impl LightAttributes {
    fn new(gl: &Gl, bound_shader: Rc<Shader>) -> Self {
        let attrs = Self {
            bound_shader,

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
        };
        attrs.sync_state(gl);
        attrs
    }
    pub fn set_pos(&mut self, gl: &Gl, pos: Vec3) {
        self.position = pos;
        self.sync_state(gl);
    }
    pub fn pos(&self) -> Vec3 {
        self.position
    }

    pub fn set_dir(&mut self, gl: &Gl, dir: Vec3) {
        self.direction = dir;
        self.sync_state(gl);
    }
    pub fn dir(&self) -> Vec3 {
        self.direction
    }

    fn sync_state(&self, gl: &Gl) {
        self.bound_shader
            .set_vec3(gl, "light.position", self.position.into())
            .unwrap();
        self.bound_shader
            .set_vec3(gl, "light.direction", self.direction.into())
            .unwrap();
        self.bound_shader
            .set_float(gl, "light.cutOff", self.cutoff.to_radians().cos())
            .unwrap();

        self.bound_shader
            .set_vec3(gl, "light.ambient", self.ambient.into())
            .unwrap();
        self.bound_shader
            .set_vec3(gl, "light.diffuse", self.diffuse.into())
            .unwrap();
        self.bound_shader
            .set_vec3(gl, "light.specular", self.specular.into())
            .unwrap();

        self.bound_shader
            .set_float(gl, "light.constant", self.constant)
            .unwrap();
        self.bound_shader
            .set_float(gl, "light.linear", self.linear)
            .unwrap();
        self.bound_shader
            .set_float(gl, "light.quadratic", self.quadratic)
            .unwrap();
    }
}

pub trait Light {
    fn pos(&self) -> Vec3;
    fn set_pos(&mut self, gl: &Gl, pos: Vec3) -> &mut Self;

    fn dir(&self) -> Vec3;
    fn set_dir(&mut self, gl: &Gl, dir: Vec3) -> &mut Self;

    fn draw(&self, _gl: &Gl, _view_matrix: Mat4) {}
    fn adjust_zoom(&mut self, _degrees: GLfloat) {}
}

pub struct FlashLight {
    attrs: LightAttributes,
}

impl FlashLight {
    pub fn new(gl: &Gl, lit_object_shader: Rc<Shader>) -> Self {
        Self {
            attrs: LightAttributes::new(gl, lit_object_shader),
        }
    }
}

impl Light for FlashLight {
    fn pos(&self) -> Vec3 {
        self.attrs.pos()
    }
    fn set_pos(&mut self, gl: &Gl, pos: Vec3) -> &mut Self {
        self.attrs.set_pos(gl, pos);
        self
    }

    fn dir(&self) -> Vec3 {
        self.attrs.dir()
    }
    fn set_dir(&mut self, gl: &Gl, dir: Vec3) -> &mut Self {
        self.attrs.set_dir(gl, dir);
        self
    }
}

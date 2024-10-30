use std::rc::Rc;

use glam::{vec3, Vec3};

use crate::{
    gl::Gl,
    shader::{Shader, ShaderTrait},
};

use super::Light;

const POSITION_DEFAULT: Vec3 = vec3(0.0, 2.0, 0.0);
const DIRECTION_DEFAULT: Vec3 = vec3(0.0, 0.0, -1.0);

const AMBIENT_STRENGTH_DEFAULT: Vec3 = vec3(0.2, 0.2, 0.2);
const DIFFUSE_STRENGTH_DEFAULT: Vec3 = vec3(0.5, 0.5, 0.5);
const SPECULAR_STRENGTH_DEFAULT: Vec3 = vec3(1.0, 1.0, 1.0);

struct DirectionLightAttributes {
    bound_shader: Rc<Shader>,

    direction: Vec3,

    // Strength of each type of lighting
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

impl DirectionLightAttributes {
    fn new(gl: &Gl, bound_shader: Rc<Shader>) -> Self {
        let attrs = Self {
            bound_shader,

            direction: DIRECTION_DEFAULT,

            // Strength of each type of lighting
            ambient: AMBIENT_STRENGTH_DEFAULT,
            diffuse: DIFFUSE_STRENGTH_DEFAULT,
            specular: SPECULAR_STRENGTH_DEFAULT,
        };
        attrs.sync_state(gl);
        attrs
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
            .set_vec3(gl, "dirLight.direction", self.direction.into())
            .unwrap();

        self.bound_shader
            .set_vec3(gl, "dirLight.ambient", self.ambient.into())
            .unwrap();
        self.bound_shader
            .set_vec3(gl, "dirLight.diffuse", self.diffuse.into())
            .unwrap();
        self.bound_shader
            .set_vec3(gl, "dirLight.specular", self.specular.into())
            .unwrap();
    }
}
pub struct DirectionLight {
    attrs: DirectionLightAttributes,
}
impl DirectionLight {
    pub fn new(gl: &Gl, lit_object_shader: Rc<Shader>) -> Self {
        Self {
            attrs: DirectionLightAttributes::new(gl, lit_object_shader),
        }
    }
}

impl Light for DirectionLight {
    fn set_dir(&mut self, gl: &Gl, dir: Vec3) -> &mut dyn Light {
        self.attrs.set_dir(gl, dir);
        self
    }
    fn dir(&self) -> Vec3 {
        self.attrs.dir()
    }

    // This is a no-op since DirectionLights don't have positions.
    fn set_pos(&mut self, gl: &Gl, pos: Vec3) -> &mut dyn Light {
        self
    }
    fn pos(&self) -> Vec3 {
        Vec3::ZERO
    }
}

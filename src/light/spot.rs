use std::rc::Rc;

use glam::{vec3, Vec3};

use crate::{
    gl::Gl,
    shader::{LightCasterShader, Shader, ShaderTrait},
};

use super::Light;

const POSITION_DEFAULT: Vec3 = vec3(0.0, 2.0, 0.0);
const DIRECTION_DEFAULT: Vec3 = vec3(0.0, 0.0, -1.0);
const INNER_CUTOFF_DEFAULT: f32 = 12.5;
const OUTER_CUTOFF_DEFAULT: f32 = 14.5;

const AMBIENT_STRENGTH_DEFAULT: Vec3 = vec3(0.0, 0.0, 0.0);
const DIFFUSE_STRENGTH_DEFAULT: Vec3 = vec3(0.5, 0.5, 0.5);
const SPECULAR_STRENGTH_DEFAULT: Vec3 = vec3(1.0, 1.0, 1.0);

const ATTENUATION_CONSTANT_DEFAULT: f32 = 1.0;
const ATTENUATION_LINEAR_DEFAULT: f32 = 0.09;
const ATTENUATION_QUADRATIC_DEFAULT: f32 = 0.032;

struct SpotLightAttributes {
    bound_shader: Rc<LightCasterShader>,

    position: Vec3,
    direction: Vec3,
    inner_cutoff: f32,
    outer_cutoff: f32,

    // Strength of each type of lighting
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,

    // Attenuation
    constant: f32,
    linear: f32,
    quadratic: f32,
}

impl SpotLightAttributes {
    fn new(gl: &Gl, bound_shader: Rc<LightCasterShader>) -> Self {
        let attrs = Self {
            bound_shader,

            position: POSITION_DEFAULT,
            direction: DIRECTION_DEFAULT,
            inner_cutoff: INNER_CUTOFF_DEFAULT,
            outer_cutoff: OUTER_CUTOFF_DEFAULT,

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

        self.bound_shader.shader
            .set_vec3(gl, "spotLight.position", self.position.into())
            .unwrap();
        self.bound_shader.shader
            .set_vec3(gl, "spotLight.direction", self.direction.into())
            .unwrap();
        self.bound_shader.shader
            .set_float(
                gl,
                "spotLight.innerCutOff",
                self.inner_cutoff.to_radians().cos(),
            )
            .unwrap();
        self.bound_shader.shader
            .set_float(
                gl,
                "spotLight.outerCutOff",
                self.outer_cutoff.to_radians().cos(),
            )
            .unwrap();

        self.bound_shader.shader
            .set_vec3(gl, "spotLight.ambient", self.ambient.into())
            .unwrap();
        self.bound_shader.shader
            .set_vec3(gl, "spotLight.diffuse", self.diffuse.into())
            .unwrap();
        self.bound_shader.shader
            .set_vec3(gl, "spotLight.specular", self.specular.into())
            .unwrap();

        self.bound_shader.shader
            .set_float(gl, "spotLight.constant", self.constant)
            .unwrap();
        self.bound_shader.shader
            .set_float(gl, "spotLight.linear", self.linear)
            .unwrap();
        self.bound_shader.shader
            .set_float(gl, "spotLight.quadratic", self.quadratic)
            .unwrap();
    }
}

pub struct SpotLight {
    attrs: SpotLightAttributes,
}

impl SpotLight {
    pub fn new(gl: &Gl, lit_object_shader: Rc<LightCasterShader>) -> Self {
        Self {
            attrs: SpotLightAttributes::new(gl, lit_object_shader),
        }
    }
}

impl Light for SpotLight {
    fn pos(&self) -> Vec3 {
        self.attrs.pos()
    }
    fn set_pos(&mut self, gl: &Gl, pos: Vec3) -> &mut dyn Light {
        self.attrs.set_pos(gl, pos);
        self
    }

    fn dir(&self) -> Vec3 {
        self.attrs.dir()
    }
    fn set_dir(&mut self, gl: &Gl, dir: Vec3) -> &mut dyn Light {
        self.attrs.set_dir(gl, dir);
        self
    }
}

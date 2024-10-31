use std::rc::Rc;

use glam::{vec3, Mat4, Vec3};

use crate::{
    camera::direction::Degrees,
    gl::Gl,
    object::light_cube::LightCube,
    renderer::{VERTEX_DATA, VERTEX_DATA_STRIDE},
    shader::{LightCasterShader, Shader, ShaderTrait},
};

use super::Light;

const POSITION_DEFAULT: Vec3 = vec3(0.0, 2.0, 0.0);

const AMBIENT_STRENGTH_DEFAULT: Vec3 = vec3(0.0, 0.0, 0.0);
const DIFFUSE_STRENGTH_DEFAULT: Vec3 = vec3(0.5, 0.5, 0.5);
const SPECULAR_STRENGTH_DEFAULT: Vec3 = vec3(1.0, 1.0, 1.0);

const ATTENUATION_CONSTANT_DEFAULT: f32 = 1.0;
const ATTENUATION_LINEAR_DEFAULT: f32 = 0.09;
const ATTENUATION_QUADRATIC_DEFAULT: f32 = 0.032;

struct PointLightAttributes {
    bound_shader: Rc<LightCasterShader>,

    position: Vec3,

    // Strength of each type of lighting
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,

    // Attenuation
    constant: f32,
    linear: f32,
    quadratic: f32,
}

impl PointLightAttributes {
    fn new(gl: &Gl, bound_shader: Rc<LightCasterShader>, index: u8) -> Self {
        let attrs = Self {
            bound_shader,

            position: POSITION_DEFAULT,

            // Strength of each type of lighting
            ambient: AMBIENT_STRENGTH_DEFAULT,
            diffuse: DIFFUSE_STRENGTH_DEFAULT,
            specular: SPECULAR_STRENGTH_DEFAULT,

            // Attenuation
            constant: ATTENUATION_CONSTANT_DEFAULT,
            linear: ATTENUATION_LINEAR_DEFAULT,
            quadratic: ATTENUATION_QUADRATIC_DEFAULT,
        };
        attrs.sync_state(gl, index);
        attrs
    }
    pub fn set_pos(&mut self, gl: &Gl, pos: Vec3, index: u8) {
        self.position = pos;
        self.sync_state(gl, index);
    }
    pub fn pos(&self) -> Vec3 {
        self.position
    }

    fn sync_state(&self, gl: &Gl, index: u8) {
        self.bound_shader.shader
            .set_vec3(
                gl,
                &format!("pointLights[{index}].position"),
                self.position.into(),
            )
            .unwrap();

        self.bound_shader.shader
            .set_vec3(
                gl,
                &format!("pointLights[{index}].ambient"),
                self.ambient.into(),
            )
            .unwrap();
        self.bound_shader.shader
            .set_vec3(
                gl,
                &format!("pointLights[{index}].diffuse"),
                self.diffuse.into(),
            )
            .unwrap();
        self.bound_shader.shader
            .set_vec3(
                gl,
                &format!("pointLights[{index}].specular"),
                self.specular.into(),
            )
            .unwrap();

        self.bound_shader.shader
            .set_float(gl, &format!("pointLights[{index}].constant"), self.constant)
            .unwrap();
        self.bound_shader.shader
            .set_float(gl, &format!("pointLights[{index}].linear"), self.linear)
            .unwrap();
        self.bound_shader.shader
            .set_float(
                gl,
                &format!("pointLights[{index}].quadratic"),
                self.quadratic,
            )
            .unwrap();
    }
}

pub struct PointLight {
    index: u8,
    light_cube: LightCube,
    attrs: PointLightAttributes,
}

impl PointLight {
    pub fn new(gl: &Gl, lit_object_shader: Rc<LightCasterShader>, index: u8) -> Self {
        let light_cube = LightCube::new(gl, POSITION_DEFAULT, &VERTEX_DATA, VERTEX_DATA_STRIDE);
        Self {
            index,
            light_cube,
            attrs: PointLightAttributes::new(gl, lit_object_shader, index),
        }
    }
}

impl Light for PointLight {
    fn pos(&self) -> Vec3 {
        self.attrs.pos()
    }
    fn set_pos(&mut self, gl: &Gl, pos: Vec3) -> &mut dyn Light {
        self.attrs.set_pos(gl, pos, self.index);
        self.light_cube.set_pos(pos);
        self
    }

    // Point Lights don't have a direction, these are a no-op
    fn dir(&self) -> Vec3 {
        Vec3::ZERO
    }
    fn set_dir(&mut self, _gl: &Gl, _dir: Vec3) -> &mut dyn Light {
        self
    }
    fn draw(&self, gl: &Gl, view_matrix: Mat4) {
        self.light_cube.draw(gl, view_matrix)
    }
    fn adjust_zoom(&mut self, degrees: Degrees) {
        self.light_cube.adjust_zoom(degrees);
    }
}

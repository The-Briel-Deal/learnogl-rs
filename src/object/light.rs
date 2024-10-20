use std::rc::Rc;

use glam::{vec3, Mat4, Vec3};

use crate::{
    gl::{types::GLfloat, Gl},
    mesh::{Mesh, VertexBuffer},
    shader::{Shader, ShaderTrait},
};

const POSITION_DEFAULT: Vec3 = vec3(0.0, 2.0, 0.0);

const AMBIENT_STRENGTH_DEFAULT: Vec3 = vec3(0.2, 0.2, 0.2);
const DIFFUSE_STRENGTH_DEFAULT: Vec3 = vec3(0.5, 0.5, 0.5);
const SPECULAR_STRENGTH_DEFAULT: Vec3 = vec3(1.0, 1.0, 1.0);

pub struct LightAttributes {
    position: Vec3,
    // Strength of each type of lighting
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

pub struct Light {
    mesh: Mesh,
    shader: Rc<Shader>,
    lit_object_shader: Rc<Shader>,
    attrs: LightAttributes,
}
impl Default for LightAttributes {
    fn default() -> Self {
        Self {
            position: POSITION_DEFAULT,
            ambient: AMBIENT_STRENGTH_DEFAULT,
            diffuse: DIFFUSE_STRENGTH_DEFAULT,
            specular: SPECULAR_STRENGTH_DEFAULT,
        }
    }
}

impl Light {
    /// Create a new light source. Leave attrs as None for default values.
    pub fn new(
        gl: &Gl,
        attrs: Option<LightAttributes>,
        lit_object_shader: Rc<Shader>,
        vertex_data: &[f32],
        vertex_data_stride: i32,
    ) -> Self {
        let attrs = attrs.unwrap_or(LightAttributes::default());
        let shader = Rc::new(Shader::new(
            gl,
            "src/shader/light_vert.glsl",
            "src/shader/light_source_frag.glsl",
        ));

        let vertex_buffer = VertexBuffer::new(gl, vertex_data, vertex_data_stride);

        vertex_buffer.set_float_attribute_position(gl, "aPos", shader.get_id(), 0, 3);

        let mut mesh = Mesh::new(attrs.position, vertex_buffer);

        mesh.adjust_scale(vec3(0.2, 0.2, 0.2));

        Self {
            mesh,
            shader,
            lit_object_shader,
            attrs,
        }
    }

    pub fn set_pos(&mut self, gl: &Gl, pos: Vec3) {
        self.attrs.position = pos;
        self.sync_state(gl);
    }

    pub fn set_attrs(&mut self, gl: &Gl, attrs: LightAttributes) {
        self.attrs = attrs;
        self.sync_state(gl);
    }

    fn sync_state(&mut self, gl: &Gl) {
        self.mesh.set_pos(self.attrs.position);
        self.lit_object_shader
            .set_vec3(gl, "light.position", self.attrs.position.into())
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
    }

    pub fn pos(&self) -> Vec3 {
        self.attrs.position
    }

    pub fn draw(&mut self, gl: &Gl, view_matrix: Mat4) {
        // I should probably not have draw mutate.
        self.mesh.draw(gl, view_matrix, &self.shader);
    }
    pub fn adjust_zoom(&mut self, degrees: GLfloat) {
        self.mesh.adjust_zoom(degrees);
    }
}

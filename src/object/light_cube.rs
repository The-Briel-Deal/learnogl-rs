use std::rc::Rc;

use glam::{vec3, Mat4, Vec3};

use crate::{
    camera::direction::Degrees,
    gl::{types::GLfloat, Gl},
    mesh::{Mesh, VertexBuffer},
    shader::{LightCubeShader, Shader, ShaderTrait},
};

pub struct LightCube {
    mesh: Mesh,
    shader: Rc<LightCubeShader>,
}

impl LightCube {
    pub fn new(gl: &Gl, pos: Vec3, vertex_data: &[f32], vertex_data_stride: i32) -> Self {
        let shader = Rc::new(LightCubeShader::new(gl));
        let lit_object_vertex_buffer = VertexBuffer::new(gl, vertex_data, vertex_data_stride);

        lit_object_vertex_buffer.set_float_attribute_position(gl, "aPos", shader.shader.get_id(), 0, 3);

        let mut mesh = Mesh::new(pos, lit_object_vertex_buffer);
        mesh.adjust_scale(vec3(0.2, 0.2, 0.2));
        Self { mesh, shader }
    }

    pub fn adjust_blend(&mut self, blend: f32) {
        self.mesh.adjust_blend(blend)
    }
    pub fn draw(&self, gl: &Gl, view_matrix: Mat4) {
        self.mesh.draw(gl, view_matrix, self.shader.as_ref());
    }
    pub fn adjust_zoom(&mut self, zoom: GLfloat) {
        self.mesh.adjust_zoom(zoom);
    }
    pub fn rotate_by(&mut self, rotation: Degrees) {
        self.mesh.rotate_by(rotation);
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.mesh.set_pos(pos)
    }
}

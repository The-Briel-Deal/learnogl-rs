use std::rc::Rc;

use glam::{Mat4, Vec3};

use crate::{
    gl::{types::GLfloat, Gl},
    mesh::{Mesh, VertexBuffer},
    shader::{Shader, ShaderTrait},
};

pub struct Cube {
    mesh: Mesh,
    shader: Rc<Shader>,
}

impl Cube {
    pub fn new(
        gl: &Gl,
        shader: Rc<Shader>,
        pos: Vec3,
        vertex_data: &[f32],
        vertex_data_stride: i32,
    ) -> Self {
        let lit_object_vertex_buffer = VertexBuffer::new(gl, vertex_data, vertex_data_stride);

        lit_object_vertex_buffer.set_float_attribute_position(gl, "aPos", shader.get_id(), 0, 3);
        lit_object_vertex_buffer.set_float_attribute_position(gl, "aNormal", shader.get_id(), 3, 3);
        Self {
            mesh: Mesh::new(pos, lit_object_vertex_buffer),
            shader,
        }
    }
    pub fn adjust_blend(&mut self, blend: f32) {
        self.mesh.adjust_blend(blend)
    }
    pub fn draw(&mut self, gl: &Gl, view_matrix: Mat4) {
        self.mesh.draw(gl, view_matrix, &self.shader);
    }
    pub fn adjust_zoom(&mut self, zoom: GLfloat) {
        self.mesh.adjust_zoom(zoom);
    }
}

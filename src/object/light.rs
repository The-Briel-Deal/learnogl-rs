use glam::{vec3, Mat4, Vec3};

use crate::{
    gl::Gl,
    mesh::{Mesh, VertexBuffer},
    shader::{Shader, ShaderTrait},
};

pub struct Light {
    mesh: Mesh,
}

impl Light {
    pub fn new(gl: &Gl, pos: Vec3, vertex_data: &[f32], vertex_data_stride: i32) -> Self {
        let shader = Shader::new(
            gl,
            "src/shader/light_vert.glsl",
            "src/shader/light_source_frag.glsl",
        );

        let vertex_buffer = VertexBuffer::new(gl, vertex_data, vertex_data_stride);

        vertex_buffer.set_float_attribute_position(gl, "aPos", shader.get_id(), 0, 3);

        let mut mesh = Mesh::new(&shader, pos, vertex_buffer);

        mesh.adjust_scale(vec3(0.2, 0.2, 0.2));

        Self { mesh }
    }

    pub fn set_pos(&mut self, gl: &Gl, pos: Vec3, lit_object_program: &Shader) {
        self.mesh.set_pos(pos);
        lit_object_program
            .set_vec3(gl, "lightPos", pos.into())
            .unwrap();
    }

    pub fn pos(&self) -> Vec3 {
        self.mesh.pos()
    }

    pub fn draw(&mut self, gl: &Gl, view_matrix: Mat4) {
        // I should probably not have draw mutate.
        self.mesh.draw(gl, view_matrix);
    }
}

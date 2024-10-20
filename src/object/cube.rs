use std::rc::Rc;

use glam::{Mat4, Vec3};

use crate::{
    gl::{types::GLfloat, Gl},
    mesh::{Mesh, VertexBuffer},
    renderer::texture::TextureManager,
    shader::{Shader, ShaderTrait},
};

const SHININESS_DEFAULT: f32 = 32.0;

pub struct Cube {
    mesh: Mesh,
    shader: Rc<Shader>,
    material: Material,
}
pub struct Material {
    pub shininess: f32,
}

impl Cube {
    pub fn new(
        gl: &Gl,
        pos: Vec3,
        shader: Rc<Shader>,
        vertex_data: &[f32],
        vertex_data_stride: i32,
    ) -> Self {
        let lit_object_vertex_buffer = VertexBuffer::new(gl, vertex_data, vertex_data_stride);

        lit_object_vertex_buffer.set_float_attribute_position(gl, "aPos", shader.get_id(), 0, 3);
        lit_object_vertex_buffer.set_float_attribute_position(gl, "aNormal", shader.get_id(), 3, 3);
        lit_object_vertex_buffer.set_float_attribute_position(
            gl,
            "aTexCoords",
            shader.get_id(),
            6,
            2,
        );

        let mut texture_manager = TextureManager::new();
        texture_manager.create_texture(
            gl,
            "material.diffuse",
            "static/diffuse_container.png",
            &shader,
            0,
        );
        texture_manager.create_texture(
            gl,
            "material.specular",
            "static/specular_container.png",
            &shader,
            1,
        );
        texture_manager.bind_texture(gl, "material.diffuse", 0);
        texture_manager.bind_texture(gl, "material.specular", 1);
        Self {
            mesh: Mesh::new(pos, lit_object_vertex_buffer),
            shader,
            material: Material {
                shininess: SHININESS_DEFAULT,
            },
        }
    }
    pub fn adjust_blend(&mut self, blend: f32) {
        self.mesh.adjust_blend(blend)
    }
    pub fn draw(&mut self, gl: &Gl, view_matrix: Mat4) {
        self.update_material_uniforms(gl);
        self.mesh.draw(gl, view_matrix, &self.shader);
    }
    pub fn adjust_zoom(&mut self, zoom: GLfloat) {
        self.mesh.adjust_zoom(zoom);
    }

    fn update_material_uniforms(&self, gl: &Gl) {
        self.shader
            .set_float(gl, "material.shininess", self.material.shininess)
            .unwrap();
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

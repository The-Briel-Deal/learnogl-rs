mod texture;
mod vertex;

use std::{ffi::CString, mem::offset_of};

use glam::{Mat4, Vec3};
use texture::Texture;
use vertex::Vertex;

use crate::{
    gl::{
        self,
        types::{GLfloat, GLint, GLuint},
        Gl,
    },
    shader::DrawableShader,
};

struct Transform {
    rotation: GLfloat,
    scale: Vec3,
    translation: Vec3,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,

    vao: u32,
    vbo: u32,
    ebo: u32,
}

pub struct VertexBuffer {
    vbo: u32,
    vao: u32,
    bindingindex: u32,
}

impl Mesh {
    pub fn new(gl: &Gl, vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        Mesh {
            vertices,
            indices,
            textures,

            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }
    fn setup_mesh(mut self, gl: &Gl) -> Self {
        unsafe {
            gl.CreateVertexArrays(1, &mut self.vao);
            gl.CreateBuffers(1, &mut self.vbo);
            gl.CreateBuffers(1, &mut self.ebo);
            // Not positive on what binding index is, I think its used if you want to bind multiple
            // vaos to vbos?
            let binding_index = 0;
            let offset = 0;
            let stride = size_of::<Vertex>() as i32;
            gl.VertexArrayVertexBuffer(self.vao, binding_index, self.vbo, offset, stride);

            let attrib_index = 0;
            let attrib_length = 3;
            let attrib_start = offset_of!(Vertex, position);
            gl.EnableVertexArrayAttrib(self.vao, attrib_index);
            gl.VertexArrayAttribFormat(
                self.vao,
                attrib_index,
                attrib_length as GLint,
                gl::FLOAT,
                gl::FALSE,
                attrib_start as u32,
            );
            gl.VertexArrayAttribBinding(self.vao, attrib_index, binding_index);
        };
        assert_ne!(self.vao, 0);
        assert_ne!(self.vbo, 0);
        assert_ne!(self.ebo, 0);
        self
    }

    pub fn draw(&self, gl: &Gl, view_matrix: Mat4, shader: &dyn DrawableShader) {
        // self.rotate_by(1.0);
        //let transform = &self.transform;

        //let model_matrix = Mat4::IDENTITY
        //    * Mat4::from_translation(transform.translation)
        //    * Mat4::from_rotation_x((transform.rotation / 2.0).to_radians())
        //    * Mat4::from_rotation_y(transform.rotation.to_radians())
        //    * Mat4::from_scale(transform.scale);

        //let projection_matrix =
        //    Mat4::perspective_rh_gl(self.fov.to_radians(), gl.get_aspect_ratio(), 0.1, 100.0);

        //shader.model().set(model_matrix);
        //shader.view().set(view_matrix);
        //shader.projection().set(projection_matrix);

        //shader.shader().enable(gl);
        unsafe {
            gl.BindVertexArray(self.vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}

impl VertexBuffer {
    pub fn new(gl: &Gl, buffer: &[f32], stride: i32) -> Self {
        let mut vertex_buffer = Self {
            vbo: 0,
            vao: 0,
            bindingindex: 0,
        };

        unsafe {
            gl.CreateBuffers(1, &mut vertex_buffer.vbo);

            gl.NamedBufferData(
                vertex_buffer.vbo,
                (std::mem::size_of_val(buffer)) as gl::types::GLsizeiptr,
                buffer.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        };

        unsafe {
            gl.CreateVertexArrays(1, &mut vertex_buffer.vao);

            gl.VertexArrayVertexBuffer(
                vertex_buffer.vao(),
                vertex_buffer.bindingindex,
                vertex_buffer.vbo(),
                0,
                stride * std::mem::size_of::<f32>() as gl::types::GLsizei,
            );
        };

        vertex_buffer
    }

    pub fn set_float_attribute_position(
        &self,
        gl: &Gl,
        shader_attribute_name: &str,
        program: u32,
        start: u32,
        length: u32,
    ) {
        unsafe {
            let c_shader_attribute_name = CString::new(shader_attribute_name).unwrap();
            let attrib = gl.GetAttribLocation(
                program,
                c_shader_attribute_name.as_ptr() as *const gl::types::GLchar,
            );
            if attrib == -1 {
                panic!("\nAttribute not found! Attribute Name: {shader_attribute_name}\n")
            }

            gl.EnableVertexArrayAttrib(self.vao(), attrib as u32);
            gl.VertexArrayAttribFormat(
                self.vao(),
                attrib as u32,
                length as GLint,
                gl::FLOAT,
                gl::FALSE,
                (start as usize * std::mem::size_of::<f32>()) as gl::types::GLuint,
            );
            gl.VertexArrayAttribBinding(self.vao(), attrib as u32, self.bindingindex);
        }
    }

    pub fn vbo(&self) -> GLuint {
        self.vbo
    }
    pub fn vao(&self) -> GLuint {
        self.vao
    }
}

#[cfg(test)]
mod test {
    use glam::{vec3, vec4, Mat4};

    #[test]
    fn test_translate() {
        let mut vec = vec4(1.0, 0.0, 0.0, 1.0);

        let trans = Mat4::from_translation(vec3(1.0, 1.0, 1.0));

        vec = trans * vec;

        assert_eq!(vec, vec4(2.0, 1.0, 1.0, 1.0));
    }
}

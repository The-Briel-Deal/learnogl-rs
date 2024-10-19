use std::ffi::CString;

use glam::{vec3, Mat4, Vec3};

use crate::{
    gl::{
        self,
        types::{GLfloat, GLint, GLuint},
        Gl,
    },
    helper::get_rand_angle,
    shader::{Shader, ShaderTrait},
};

struct Transform {
    rotation: GLfloat,
    scale: Vec3,
    translation: Vec3,
}

pub struct Mesh {
    program: Shader,
    vertex_buffer: VertexBuffer,
    transform: Transform,
    fov: f32,
    texture_blend: GLfloat,
}

pub struct VertexBuffer {
    vbo: u32,
    vao: u32,
    bindingindex: u32,
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

            gl.EnableVertexArrayAttrib(self.vao(), dbg!(attrib) as u32);
            gl.VertexArrayAttribFormat(
                self.vao(),
                attrib as u32,
                length as GLint,
                gl::FLOAT,
                gl::FALSE,
                (start as usize * std::mem::size_of::<f32>()) as gl::types::GLuint,
            );
            gl.VertexArrayAttribBinding(self.vao(), dbg!(attrib) as u32, self.bindingindex);
        }
    }

    pub fn vbo(&self) -> GLuint {
        self.vbo
    }
    pub fn vao(&self) -> GLuint {
        self.vao
    }
}

impl Mesh {
    pub fn new(gl: &Gl, program: &Shader, translation: Vec3, vertex_buffer: VertexBuffer) -> Self {
        let mesh = Mesh {
            program: program.clone(),
            vertex_buffer,
            transform: Transform {
                rotation: get_rand_angle(),
                translation,
                scale: vec3(1.0, 1.0, 1.0),
            },
            fov: 80.0,
            texture_blend: 0.2,
        };

        mesh.vertex_buffer
            .set_float_attribute_position(gl, "aPos", program.get_id(), 0, 3);
        mesh.vertex_buffer
            .set_float_attribute_position(gl, "aTexCoord", program.get_id(), 3, 2);

        mesh
    }

    pub fn adjust_blend(&mut self, percent: f32) {
        self.texture_blend = (self.texture_blend + percent).clamp(0.0, 1.0);
    }
    pub fn blend(&self) -> f32 {
        self.texture_blend
    }

    pub fn rotate_by(&mut self, degrees: GLfloat) {
        let transform = &mut self.transform;
        transform.rotation += degrees;
    }

    pub fn adjust_zoom(&mut self, degrees: GLfloat) {
        self.fov = (self.fov + degrees).clamp(5.0, 80.0);
    }

    pub fn adjust_scale(&mut self, scale: Vec3) {
        self.transform.scale =
            (self.transform.scale * scale).clamp(vec3(0.1, 0.1, 0.1), vec3(10.0, 10.0, 10.0));
    }

    pub fn pos(&self) -> Vec3 {
        self.transform.translation
    }

    pub fn vao(&self) -> GLuint {
        self.vertex_buffer.vao()
    }

    pub fn draw(&mut self, gl: &Gl, view_matrix: Mat4) {
        self.rotate_by(1.0);
        let transform = &self.transform;

        let model_matrix = Mat4::IDENTITY
            * Mat4::from_translation(transform.translation)
            * Mat4::from_rotation_x((transform.rotation / 2.0).to_radians())
            * Mat4::from_rotation_y(transform.rotation.to_radians())
            * Mat4::from_scale(transform.scale);

        let projection_matrix =
            Mat4::perspective_rh_gl(self.fov.to_radians(), gl.get_aspect_ratio(), 0.1, 100.0);

        self.program.set_mat4(gl, "model", model_matrix).unwrap();

        self.program.set_mat4(gl, "view", view_matrix).unwrap();

        self.program
            .set_mat4(gl, "projection", projection_matrix)
            .unwrap();

        unsafe {
            gl.BindVertexArray(self.vao());
            gl.DrawArrays(gl::TRIANGLES, 0, 36);
        }
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

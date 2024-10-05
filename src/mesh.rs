use std::{cell::RefCell, collections::HashMap, os::raw::c_void};

use image::ImageReader;

use crate::{
    gl::{
        self,
        types::{GLfloat, GLuint},
        Gl,
    },
    shader::{Shader, ShaderTrait},
};

pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    texture_map: HashMap<String, GLuint>,
    pub texture_blend: RefCell<GLfloat>,
}

impl Mesh {
    pub fn new(gl: &Gl, program: &Shader) -> Self {
        let mut mesh = Mesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            texture_map: HashMap::new(),
            texture_blend: RefCell::new(0.0),
        };

        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            mesh.texture_map.insert(
                "texture1".to_string(),
                Self::create_texture(gl, "static/container.jpg"),
            );
            program.set_int("texture1", 0).unwrap();

            gl.ActiveTexture(gl::TEXTURE1);
            mesh.texture_map.insert(
                "texture2".to_string(),
                Self::create_texture(gl, "static/awesomeface.png"),
            );
            program.set_int("texture2", 1).unwrap();

            program
                .set_float("textureBlend", *mesh.texture_blend.borrow())
                .unwrap();

            gl.GenVertexArrays(1, &mut mesh.vao);
            gl.BindVertexArray(mesh.vao);
            /* EBO start*/

            gl.GenBuffers(1, &mut mesh.ebo);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);

            let ebo_indicies = [
                0, 1, 3, // Triangle One
                1, 2, 3, // Triangle Two
            ];

            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size_of_val(&ebo_indicies) as isize,
                ebo_indicies.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            /* EBO end */

            gl.GenBuffers(1, &mut mesh.vbo);

            Self::point_attributes_to_buffer(gl, mesh.vbo, program.get_id(), &VERTEX_DATA);
        };

        mesh
    }

    pub fn get_texture(&self, name: &str) -> GLuint {
        *self.texture_map.get(name).unwrap()
    }

    pub fn get_vao(&self) -> GLuint {
        self.vao
    }

    fn point_attributes_to_buffer(gl: &gl::Gl, vbo: u32, program: u32, verticies: &[f32]) {
        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of_val(verticies)) as gl::types::GLsizeiptr,
                verticies.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib =
                gl.GetAttribLocation(program, b"position\0".as_ptr() as *const gl::types::GLchar);
            let color_attrib =
                gl.GetAttribLocation(program, b"color\0".as_ptr() as *const gl::types::GLchar);
            let texture_coord_attrib = gl.GetAttribLocation(
                program,
                b"textureCoord\0".as_ptr() as *const gl::types::GLchar,
            );

            #[allow(clippy::erasing_op)] // I am turning this off so that the last arg is clear.
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                7 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (0 * size_of::<f32>()) as *const c_void,
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                7 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * size_of::<f32>()) as *const c_void,
            );
            gl.VertexAttribPointer(
                texture_coord_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                7 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (5 * size_of::<f32>()) as *const c_void,
            );

            gl.BindBuffer(gl::ARRAY_BUFFER, 0);

            gl.EnableVertexAttribArray(pos_attrib as GLuint);
            gl.EnableVertexAttribArray(color_attrib as GLuint);
            gl.EnableVertexAttribArray(texture_coord_attrib as GLuint);
        }
    }

    fn create_texture(gl: &Gl, path: &str) -> GLuint {
        let img = ImageReader::open(path).unwrap().decode().unwrap().flipv();

        let img_height = img.height();
        let img_width = img.width();
        let data = img.to_rgba8();

        let mut texture: GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img_width as i32,
                img_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        };

        texture
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 28] = [
    // positions   // colors        // texture coords
     0.5,  0.5,    1.0, 0.0, 0.0,   1.0, 1.0,   // top right
     0.5, -0.5,    0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
    -0.5, -0.5,    0.0, 0.0, 1.0,   0.0, 0.0,   // bottom let
    -0.5,  0.5,    1.0, 1.0, 0.0,   0.0, 1.0    // top let 
];

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

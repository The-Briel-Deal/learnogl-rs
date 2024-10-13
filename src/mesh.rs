use std::{cell::RefCell, collections::HashMap, os::raw::c_void};

use glam::{vec3, Mat4, Vec3};
use image::ImageReader;

use crate::{
    gl::{
        self,
        types::{GLfloat, GLuint},
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
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    transform: RefCell<Transform>,
    texture_map: HashMap<String, GLuint>,
    fov: f32,
    pub texture_blend: RefCell<GLfloat>,
}

impl Mesh {
    pub fn new(gl: &Gl, program: &Shader, translation: Vec3) -> Self {
        let mut mesh = Mesh {
            program: program.clone(),
            vao: 0,
            vbo: 0,
            ebo: 0,
            transform: RefCell::new(Transform {
                rotation: get_rand_angle(),
                translation,
                scale: vec3(1.0, 1.0, 1.0),
            }),
            texture_map: HashMap::new(),
            fov: 80.0,
            texture_blend: RefCell::new(0.2),
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

    pub fn rotate_by(&self, degrees: GLfloat) {
        let mut transform = self.transform.borrow_mut();
        transform.rotation += degrees;
    }

    pub fn adjust_zoom(&mut self, degrees: GLfloat) {
        self.fov = (self.fov + degrees).clamp(5.0, 80.0);
    }

    pub fn get_texture(&self, name: &str) -> GLuint {
        *self.texture_map.get(name).unwrap()
    }

    pub fn get_vao(&self) -> GLuint {
        self.vao
    }

    pub fn draw(&self, gl: &Gl, view_matrix: Mat4) {
        self.rotate_by(1.0);
        let transform = self.transform.borrow();

        let model_matrix = Mat4::IDENTITY
            * Mat4::from_translation(transform.translation)
            * Mat4::from_rotation_x((transform.rotation / 2.0).to_radians())
            * Mat4::from_rotation_y(transform.rotation.to_radians())
            * Mat4::from_scale(transform.scale);

        let projection_matrix =
            Mat4::perspective_rh_gl(self.fov.to_radians(), gl.get_aspect_ratio(), 0.1, 100.0);

        let output_matrix = Mat4::IDENTITY * projection_matrix * view_matrix * model_matrix; // * transformation_matrix;

        self.program.set_mat4("transform", output_matrix).unwrap();
        unsafe {
            gl.BindVertexArray(self.get_vao());
            gl.DrawArrays(gl::TRIANGLES, 0, 36);
        }
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
            let texture_coord_attrib = gl.GetAttribLocation(
                program,
                b"textureCoord\0".as_ptr() as *const gl::types::GLchar,
            );

            #[allow(clippy::erasing_op)] // I am turning this off so that the last arg is clear.
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (0 * size_of::<f32>()) as *const c_void,
            );
            gl.VertexAttribPointer(
                texture_coord_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (3 * size_of::<f32>()) as *const c_void,
            );

            gl.BindBuffer(gl::ARRAY_BUFFER, 0);

            gl.EnableVertexAttribArray(pos_attrib as GLuint);
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
static VERTEX_DATA: [f32; 180] = [
    // Vertices                      // Texture Coords
    -0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 0.0_f32,
     0.5_f32, -0.5_f32, -0.5_f32,    1.0_f32, 0.0_f32,
     0.5_f32,  0.5_f32, -0.5_f32,    1.0_f32, 1.0_f32,
     0.5_f32,  0.5_f32, -0.5_f32,    1.0_f32, 1.0_f32,
    -0.5_f32,  0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,
    -0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 0.0_f32,

    -0.5_f32, -0.5_f32,  0.5_f32,    0.0_f32, 0.0_f32,
     0.5_f32, -0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,
     0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 1.0_f32,
     0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 1.0_f32,
    -0.5_f32,  0.5_f32,  0.5_f32,    0.0_f32, 1.0_f32,
    -0.5_f32, -0.5_f32,  0.5_f32,    0.0_f32, 0.0_f32,

    -0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,
    -0.5_f32,  0.5_f32, -0.5_f32,    1.0_f32, 1.0_f32,
    -0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,
    -0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,
    -0.5_f32, -0.5_f32,  0.5_f32,    0.0_f32, 0.0_f32,
    -0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,

     0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,
     0.5_f32,  0.5_f32, -0.5_f32,    1.0_f32, 1.0_f32,
     0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,
     0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,
     0.5_f32, -0.5_f32,  0.5_f32,    0.0_f32, 0.0_f32,
     0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,

    -0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,
     0.5_f32, -0.5_f32, -0.5_f32,    1.0_f32, 1.0_f32,
     0.5_f32, -0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,
     0.5_f32, -0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,
    -0.5_f32, -0.5_f32,  0.5_f32,    0.0_f32, 0.0_f32,
    -0.5_f32, -0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,

    -0.5_f32,  0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32,
     0.5_f32,  0.5_f32, -0.5_f32,    1.0_f32, 1.0_f32,
     0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,
     0.5_f32,  0.5_f32,  0.5_f32,    1.0_f32, 0.0_f32,
    -0.5_f32,  0.5_f32,  0.5_f32,    0.0_f32, 0.0_f32,
    -0.5_f32,  0.5_f32, -0.5_f32,    0.0_f32, 1.0_f32
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

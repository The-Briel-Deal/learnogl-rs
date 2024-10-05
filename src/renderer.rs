use std::{
    borrow::Borrow,
    cell::RefCell,
    ffi::{c_void, CString},
    ptr::null,
    rc::Rc,
};

use glutin::prelude::GlDisplay;
use image::ImageReader;

use crate::{
    gl::{
        self,
        types::{GLfloat, GLuint},
        Gl,
    },
    logging::setup_logging,
    shader::{Shader, ShaderTrait},
};

pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    texture: GLuint,
    texture2: GLuint,
    pub texture_blend: RefCell<GLfloat>,
}

impl Mesh {
    fn new(gl: &Gl, program: &Shader) -> Self {
        let mut mesh = Mesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            texture: 0,
            texture2: 0,
            texture_blend: RefCell::new(0.0),
        };

        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            mesh.texture = Mesh::create_texture(gl, "static/container.jpg");
            program.set_int("texture1", 0).unwrap();

            gl.ActiveTexture(gl::TEXTURE1);
            mesh.texture2 = Mesh::create_texture(gl, "static/awesomeface.png");
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

            Renderer::point_attributes_to_buffer(&gl, mesh.vbo, program.get_id(), &VERTEX_DATA);
        };

        mesh
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

pub struct Renderer {
    program: Shader,
    pub mesh_list: Vec<Mesh>,
    gl: Rc<Gl>,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        let gl = Rc::new(gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        }));
        setup_logging(&gl);

        let program = Shader::new(gl.clone(), "src/shader/vert.glsl", "src/shader/frag.glsl");
        let mesh = Mesh::new(gl.borrow(), &program);

        Self {
            program,
            mesh_list: vec![mesh],
            gl,
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
    pub fn draw(&self) {
        self.draw_with_clear_color(0.1, 0.1, 0.1, 0.9)
    }

    pub fn draw_with_clear_color(
        &self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        unsafe {
            let mesh = &self.mesh_list[0];
            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.program.enable();
            self.program
                .set_float("textureBlend", *mesh.texture_blend.borrow())
                .unwrap();
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, mesh.texture);
            self.gl.ActiveTexture(gl::TEXTURE1);
            self.gl.BindTexture(gl::TEXTURE_2D, mesh.texture2);
            self.gl.BindVertexArray(mesh.vao);
            self.gl
                .DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }
    }
    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.Viewport(0, 0, width, height) }
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

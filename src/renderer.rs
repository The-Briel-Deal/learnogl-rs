use std::{ffi::CString, rc::Rc};

use glutin::prelude::GlDisplay;

use crate::{
    gl::{
        self, get_gl_string,
        types::{GLfloat, GLuint},
        Gl,
    },
    shader::{Shader, ShaderTrait},
};

pub struct Renderer {
    program: Shader,
    vao: GLuint,
    vbo: GLuint,
    gl: Rc<Gl>,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Renderer {
        let gl = Rc::new(gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        }));

        if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
            println!("Running on {}", renderer.to_string_lossy());
        }
        if let Some(version) = get_gl_string(&gl, gl::VERSION) {
            println!("OpenGL Version {}", version.to_string_lossy());
        }
        if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
            println!("Shaders version on {}", shaders_version.to_string_lossy());
        }
        unsafe {
            let mut renderer = Self {
                program: Shader::new(gl.clone(), "src/shader/vert.glsl", "src/shader/frag.glsl"),
                vao: std::mem::zeroed(),
                vbo: std::mem::zeroed(),
                gl,
            };

            renderer.gl.GenVertexArrays(1, &mut renderer.vao);
            renderer.gl.BindVertexArray(renderer.vao);

            renderer.gl.GenBuffers(1, &mut renderer.vbo);

            Self::point_attributes_to_buffer(
                &renderer.gl,
                renderer.vbo,
                renderer.program.get_id(),
                &VERTEX_DATA,
            );

            renderer
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

            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            gl.BindBuffer(gl::ARRAY_BUFFER, 0);

            gl.EnableVertexAttribArray(pos_attrib as GLuint);
            gl.EnableVertexAttribArray(color_attrib as GLuint);
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
            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.program.enable();
            self.gl.BindVertexArray(self.vao);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.Viewport(0, 0, width, height) }
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 6] = [
    -0.5, -0.5, // Bottom Left
     0.0,  0.5, // Top Center
     0.5, -0.5, // Bottom Right
];

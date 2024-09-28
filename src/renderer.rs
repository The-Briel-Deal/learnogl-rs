use std::{ffi::CString, os::raw::c_void};

use glutin::prelude::GlDisplay;

use crate::{gl::{
    self, create_shader, get_gl_string,
    types::{GLfloat, GLuint},
    Gl,
}, helper::add_null_term};

pub struct Renderer {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    gl: Gl,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

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
            let vertex_shader =
                create_shader(&gl, gl::VERTEX_SHADER, &add_null_term(VERTEX_SHADER_SOURCE));

            let fragment_shader = create_shader(
                &gl,
                gl::FRAGMENT_SHADER,
                &add_null_term(FRAGMENT_SHADER_SOURCE),
            );

            let mut renderer = Self {
                program: gl.CreateProgram(),
                vao: std::mem::zeroed(),
                vbo: std::mem::zeroed(),
                gl: gl::Gl::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    gl_display.get_proc_address(symbol.as_c_str()).cast()
                }),
            };

            gl.AttachShader(renderer.program, vertex_shader);
            gl.AttachShader(renderer.program, fragment_shader);

            gl.LinkProgram(renderer.program);

            gl.UseProgram(renderer.program);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            gl.GenVertexArrays(1, &mut renderer.vao);
            gl.BindVertexArray(renderer.vao);

            gl.GenBuffers(1, &mut renderer.vbo);

            Self::point_attributes_to_buffer(&gl, renderer.vbo, renderer.program);

            renderer
        }
    }
    fn point_attributes_to_buffer(gl: &gl::Gl, vbo: u32, program: u32) {
        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
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
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const c_void,
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
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);

            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.Viewport(0, 0, width, height) }
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];

const VERTEX_SHADER_SOURCE: &[u8] = include_bytes!("shaders/vert.glsl");
const FRAGMENT_SHADER_SOURCE: &[u8] = include_bytes!("shaders/frag.glsl");


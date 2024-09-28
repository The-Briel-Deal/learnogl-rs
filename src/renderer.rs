use std::{ffi::CString, os::raw::c_void};

use glutin::prelude::GlDisplay;

use crate::{
    gl::{
        self, create_shader, get_gl_string,
        types::{GLfloat, GLuint},
        Gl,
    },
    helper::add_null_term,
};

const VERTEX_SHADER_SOURCE: &[u8] = include_bytes!("shaders/vert.glsl");
const FRAGMENT_SHADER_SOURCE: &[u8] = include_bytes!("shaders/frag.glsl");
const SECOND_FRAGMENT_SHADER_SOURCE: &[u8] = include_bytes!("shaders/second_frag.glsl");

pub struct Renderer {
    program: GLuint,
    program2: GLuint,
    vao: GLuint,
    vao2: GLuint,
    vbo: GLuint,
    vbo2: GLuint,
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

            let second_fragment_shader = create_shader(
                &gl,
                gl::FRAGMENT_SHADER,
                &add_null_term(SECOND_FRAGMENT_SHADER_SOURCE),
            );

            let mut renderer = Self {
                program: gl.CreateProgram(),
                program2: gl.CreateProgram(),
                vao: std::mem::zeroed(),
                vao2: std::mem::zeroed(),
                vbo: std::mem::zeroed(),
                vbo2: std::mem::zeroed(),
                gl: gl::Gl::load_with(|symbol| {
                    let symbol = CString::new(symbol).unwrap();
                    gl_display.get_proc_address(symbol.as_c_str()).cast()
                }),
            };

            gl.AttachShader(renderer.program, vertex_shader);
            gl.AttachShader(renderer.program, fragment_shader);
            gl.LinkProgram(renderer.program);

            gl.AttachShader(renderer.program2, vertex_shader);
            gl.AttachShader(renderer.program2, second_fragment_shader);
            gl.LinkProgram(renderer.program2);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            gl.GenVertexArrays(1, &mut renderer.vao);
            gl.BindVertexArray(renderer.vao);

            gl.GenBuffers(1, &mut renderer.vbo);

            Self::point_attributes_to_buffer(&gl, renderer.vbo, renderer.program, &VERTEX_DATA);

            gl.GenVertexArrays(1, &mut renderer.vao2);
            gl.BindVertexArray(renderer.vao2);

            gl.GenBuffers(1, &mut renderer.vbo2);

            Self::point_attributes_to_buffer(&gl, renderer.vbo2, renderer.program, &SECOND_VERTEX);

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
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
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

            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.UseProgram(self.program);
            self.gl.BindVertexArray(self.vao);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);

            self.gl.UseProgram(self.program2);
            self.gl.BindVertexArray(self.vao2);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.Viewport(0, 0, width, height) }
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.25, -0.25,  1.0,  0.0,  0.0, // Bottom Left
     0.25,  0.75,  0.0,  1.0,  0.0, // Top Center
     0.75, -0.25,  0.0,  0.0,  1.0, // Bottom Right
];
#[rustfmt::skip]
static SECOND_VERTEX: [f32; 15] = [
    -0.75, -0.75,  1.0,  0.0,  0.0,  // Bottom Left
     -0.25,  0.25,  0.0,  1.0,  0.0, // Top Center
     0.25, -0.75,  0.0,  0.0,  1.0,  // Bottom Right
];

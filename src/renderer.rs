use std::{cell::RefCell, ffi::CString, os::raw::c_void, rc::Rc};

use glutin::prelude::GlDisplay;

use crate::{
    gl::{
        self, get_gl_string,
        types::{GLfloat, GLuint},
        Gl,
    },
    helper::calculate_center_of_triangle,
    shader::{Shader, ShaderTrait},
};

pub struct Renderer {
    program: Shader,
    program2: Shader,
    vao: GLuint,
    vao2: GLuint,
    vbo: GLuint,
    vbo2: GLuint,
    uni_float: RefCell<f32>,
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
                program2: Shader::new(gl.clone(), "src/shader/vert.glsl", "src/shader/frag.glsl"),
                vao: std::mem::zeroed(),
                vao2: std::mem::zeroed(),
                vbo: std::mem::zeroed(),
                vbo2: std::mem::zeroed(),
                uni_float: RefCell::new(0.0),
                gl,
            };

            //#[allow(clippy::identity_op)]
            //renderer.program.set_vecf2(
            //    "center",
            //    calculate_center_of_triangle(
            //        (VERTEX_DATA[0], VERTEX_DATA[1]),
            //        (VERTEX_DATA[5 + 0], VERTEX_DATA[5 + 1]),
            //        (VERTEX_DATA[10 + 0], VERTEX_DATA[10 + 1]),
            //    ),
            //).unwrap();

            renderer
                .program
                .set_float("uniColor", *renderer.uni_float.borrow())
                .unwrap();

            renderer.gl.GenVertexArrays(1, &mut renderer.vao);
            renderer.gl.BindVertexArray(renderer.vao);

            renderer.gl.GenBuffers(1, &mut renderer.vbo);

            Self::point_attributes_to_buffer(
                &renderer.gl,
                renderer.vbo,
                renderer.program.get_id(),
                &VERTEX_DATA,
            );

            renderer.gl.GenVertexArrays(1, &mut renderer.vao2);
            renderer.gl.BindVertexArray(renderer.vao2);

            renderer.gl.GenBuffers(1, &mut renderer.vbo2);

            Self::point_attributes_to_buffer(
                &renderer.gl,
                renderer.vbo2,
                renderer.program.get_id(),
                &SECOND_VERTEX,
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
            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            *self.uni_float.borrow_mut() += 0.01;

            self.program
                .set_float("uniColor", *self.uni_float.borrow())
                .unwrap();

            self.program.enable();
            self.gl.BindVertexArray(self.vao);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);

            self.program2.enable();
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
    -0.25, -0.25,  0.5,  0.0,  0.0, // Bottom Left
     0.25,  0.75,  0.0,  0.5,  0.0, // Top Center
     0.75, -0.25,  0.0,  0.0,  0.5, // Bottom Right
];
#[rustfmt::skip]
static SECOND_VERTEX: [f32; 15] = [
    -0.75, -0.75,  1.0,  0.0,  0.0, // Bottom Left
    -0.25,  0.25,  0.0,  1.0,  0.0, // Top Center
     0.25, -0.75,  0.0,  0.0,  1.0, // Bottom Right
];

use std::{borrow::Borrow, ffi::CString, rc::Rc};

use glam::vec3;
use glutin::prelude::GlDisplay;

use crate::{
    camera::Camera,
    gl::{self, types::GLfloat, Gl},
    logging::setup_logging,
    mesh::Mesh,
    shader::{Shader, ShaderTrait},
};

pub struct Renderer {
    program: Shader,
    pub mesh_list: Vec<Mesh>,
    pub camera: Rc<Camera>,
    gl: Rc<Gl>,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        let gl = Rc::new(gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        }));

        unsafe { gl.Enable(gl::DEPTH_TEST) };
        setup_logging(&gl);

        let program = Shader::new(gl.clone(), "src/shader/vert.glsl", "src/shader/frag.glsl");

        let camera = Rc::new(Camera::new());

        #[rustfmt::skip]
        let cube_positions = [
            vec3( 0.0,  0.0,  0.0),
            vec3( 2.0,  5.0, -15.0),
            vec3(-1.5, -2.2, -2.5),
            vec3(-3.8, -2.0, -12.3),
            vec3( 2.4, -0.4, -3.5),
            vec3(-1.7,  3.0, -7.5),
            vec3( 1.3, -2.0, -2.5),
            vec3( 1.5,  2.0, -2.5),
            vec3( 1.5,  0.2, -1.5),
            vec3(-1.3,  1.0, -1.5)
        ];

        let mesh_list = Vec::from(
            cube_positions.map(|pos| Mesh::new(gl.borrow(), camera.clone(), &program, pos)),
        );

        Self {
            program,
            mesh_list,
            camera,
            gl,
        }
    }

    pub fn draw(&self, delta_time: f32) {
        self.camera.adjust_yaw(100.0 * delta_time);
        self.draw_with_clear_color(0.1, 0.1, 0.1, 0.9);
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
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.program.enable();
            for mesh in &self.mesh_list {
                self.program
                    .set_float("textureBlend", *mesh.texture_blend.borrow())
                    .unwrap();
                self.gl.ActiveTexture(gl::TEXTURE0);
                self.gl
                    .BindTexture(gl::TEXTURE_2D, mesh.get_texture("texture1"));
                self.gl.ActiveTexture(gl::TEXTURE1);
                self.gl
                    .BindTexture(gl::TEXTURE_2D, mesh.get_texture("texture2"));
                mesh.draw(&self.gl)
            }
        }
    }
    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.Viewport(0, 0, width, height) }
    }
}

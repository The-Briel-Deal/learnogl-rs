mod texture;

use std::{borrow::Borrow, ffi::CString};

use glam::vec3;
use glutin::prelude::GlDisplay;
use texture::TextureManager;
use winit::keyboard::KeyCode;

use crate::{
    camera::Camera,
    gl::{self, types::GLfloat, Gl},
    logging::setup_logging,
    mesh::Mesh,
    shader::{Shader, ShaderTrait},
};

type PositionDelta2D = (f64, f64);

pub struct Renderer {
    program: Shader,
    textures: TextureManager,
    pub mesh_list: Vec<Mesh>,
    camera: Camera,
    gl: Gl,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        unsafe { gl.Enable(gl::DEPTH_TEST) };
        setup_logging(&gl);

        let program = Shader::new(&gl, "src/shader/vert.glsl", "src/shader/frag.glsl");

        let mut textures = TextureManager::new();
        textures.create_texture(&gl, "container", "static/container.jpg", &program, 0);
        textures.create_texture(&gl, "awesomeface", "static/awesomeface.png", &program, 1);

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

        let mesh_list = Vec::from(cube_positions.map(|pos| Mesh::new(gl.borrow(), &program, pos)));

        Self {
            program,
            textures,
            mesh_list,
            gl,
            camera: Camera::new(),
        }
    }

    pub fn handle_movement_keys(&self, keys: Vec<KeyCode>, delta_time: f32) {
        self.camera.handle_movement(keys, delta_time);
    }
    pub fn handle_texture_blends_keys(&mut self, keys: Vec<KeyCode>) {
        let mesh_list = &mut self.mesh_list;
        keys.iter().for_each(|key| match key {
            KeyCode::KeyJ => mesh_list
                .iter_mut()
                .for_each(|mesh| mesh.adjust_blend(-0.01)),
            KeyCode::KeyK => mesh_list
                .iter_mut()
                .for_each(|mesh| mesh.adjust_blend(0.01)),
            _ => (),
        })
    }

    pub fn handle_mouse_input(&mut self, delta: PositionDelta2D) {
        self.camera.adjust_yaw(delta.0 as f32 / 10.0);
        self.camera.adjust_pitch(-(delta.1 as f32 / 10.0));
    }

    pub fn draw(&mut self, _delta_time: f32) {
        self.draw_with_clear_color(0.1, 0.1, 0.1, 0.9);
    }

    pub fn adjust_zoom(&mut self, degrees: GLfloat) {
        for mesh in &mut self.mesh_list {
            mesh.adjust_zoom(degrees);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.Viewport(0, 0, width, height) }
    }

    fn draw_with_clear_color(
        &mut self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        unsafe {
            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.program.enable(&self.gl);
            for mesh in &mut self.mesh_list {
                self.program
                    .set_float(&self.gl, "textureBlend", mesh.blend())
                    .unwrap();
                /* Bind Textures */
                self.textures.bind_texture(&self.gl, "awesomeface", 0);
                self.textures.bind_texture(&self.gl, "container", 1);
                mesh.draw(&self.gl, self.camera.view_matrix())
            }
        }
    }
}

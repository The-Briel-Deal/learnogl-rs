pub mod texture;

use std::{ffi::CString, rc::Rc};

use glam::{vec3, Vec3};
use glutin::prelude::GlDisplay;
use winit::keyboard::KeyCode;

use crate::{
    camera::Camera,
    gl::{self, types::GLfloat, Gl},
    light::{SpotLight, Light},
    logging::setup_logging,
    object::cube::Cube,
    shader::Shader,
    timer::Timer,
};

type PositionDelta2D = (f64, f64);

pub struct Renderer {
    light_source: Box<dyn Light>,
    lit_objects: Vec<Cube>,
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

        let lit_object_program = Rc::new(Shader::new(
            &gl,
            "src/shader/light_casters_vert.glsl",
            "src/shader/light_casters_frag.glsl",
        ));

        let light_source = Box::new(SpotLight::new(&gl, Rc::clone(&lit_object_program)));

        let lit_objects = Vec::from(LIT_CUBE_POSITIONS.map(|pos| {
            Cube::new(
                &gl,
                pos,
                Rc::clone(&lit_object_program),
                &VERTEX_DATA,
                VERTEX_DATA_STRIDE,
            )
        }));

        let camera = Camera::new();
        Self {
            light_source,
            lit_objects,
            gl,
            camera,
        }
    }

    pub fn handle_movement_keys(&mut self, keys: Vec<KeyCode>, delta_time: f32) {
        self.camera.handle_movement(keys, delta_time);
    }
    pub fn handle_texture_blends_keys(&mut self, keys: Vec<KeyCode>) {
        let mesh_list = &mut self.lit_objects;
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

    pub fn draw(&mut self, timer: &Timer) {
        self.draw_with_clear_color(timer, 0.1, 0.1, 0.1, 0.9);
    }

    pub fn adjust_zoom(&mut self, degrees: GLfloat) {
        self.light_source.adjust_zoom(degrees);
        for mesh in &mut self.lit_objects {
            mesh.adjust_zoom(degrees);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe { self.gl.Viewport(0, 0, width, height) }
    }

    fn draw_with_clear_color(
        &mut self,
        timer: &Timer,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        let gl = &self.gl;
        unsafe {
            gl.ClearColor(red, green, blue, alpha);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.light_source
                .set_pos(gl, self.camera.pos())
                .set_dir(gl, self.camera.get_forwards_dir())
                .draw(gl, self.camera.view_matrix());

            for lit_object in &mut self.lit_objects {
                lit_object.rotate_by(10.0 * timer.delta_time());
                lit_object.draw(gl, self.camera.view_matrix())
            }
        }
    }
}

#[rustfmt::skip]
static LIT_CUBE_POSITIONS: [Vec3; 10] = [
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

const VERTEX_DATA_STRIDE: i32 = 8;

#[rustfmt::skip]
static VERTEX_DATA: [f32; 288] = [
     // positions      // normals        // texture coords
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
     0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
     0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
     0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,
     0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 0.0,
     0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
     0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,

    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
     0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
     0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
    ];

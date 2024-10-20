// TODO: Texture isn't currently being used due to lighting changes.
#[allow(dead_code)]
mod texture;

use std::ffi::CString;

use glam::{vec3, Vec3};
use glutin::prelude::GlDisplay;
use winit::keyboard::KeyCode;

use crate::{
    camera::Camera,
    gl::{self, types::GLfloat, Gl},
    logging::setup_logging,
    mesh::{Mesh, VertexBuffer},
    object::light::Light,
    shader::{Shader, ShaderTrait},
    timer::Timer,
};

const AMBIENT_LIGHTING_CONSTANT: f32 = 0.3;
const SPECULAR_STRENGTH_CONSTANT: f32 = 0.9;

type PositionDelta2D = (f64, f64);

pub struct Renderer {
    light_source: Light,
    lit_object_program: Shader,
    lit_objects: Vec<Mesh>,
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

        let lit_object_program = Shader::new(
            &gl,
            "src/shader/light_vert.glsl",
            "src/shader/lit_object_frag.glsl",
        );

        let light_source = Light::new(&gl, vec3(0.0, 2.0, 0.0), &VERTEX_DATA, VERTEX_DATA_STRIDE);

        let lit_objects = Vec::from(LIT_CUBE_POSITIONS.map(|pos| {
            let lit_object_vertex_buffer = VertexBuffer::new(&gl, &VERTEX_DATA, VERTEX_DATA_STRIDE);

            lit_object_vertex_buffer.set_float_attribute_position(
                &gl,
                "aPos",
                lit_object_program.get_id(),
                0,
                3,
            );
            lit_object_vertex_buffer.set_float_attribute_position(
                &gl,
                "aNormal",
                lit_object_program.get_id(),
                3,
                3,
            );
            Mesh::new(&lit_object_program, pos, lit_object_vertex_buffer)
        }));

        lit_object_program
            .set_vec3(&gl, "objectColor", (1.0, 0.5, 0.31))
            .unwrap();
        lit_object_program
            .set_vec3(&gl, "lightColor", (1.0, 1.0, 1.0))
            .unwrap();
        lit_object_program
            .set_float(&gl, "ambientLightConstant", AMBIENT_LIGHTING_CONSTANT)
            .unwrap();

        let camera = Camera::new();
        lit_object_program
            .set_vec3(&gl, "viewPos", camera.pos().into())
            .unwrap();
        lit_object_program
            .set_float(&gl, "specularStrength", SPECULAR_STRENGTH_CONSTANT)
            .unwrap();

        Self {
            light_source,
            lit_object_program,
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
        unsafe {
            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let time_elapsed = timer.elapsed();
            self.light_source.set_pos(
                &self.gl,
                vec3(
                    time_elapsed.sin(),
                    self.light_source.pos().y,
                    self.light_source.pos().z,
                ),
                &self.lit_object_program,
            );
            self.light_source.draw(&self.gl, self.camera.view_matrix());

            for lit_object in &mut self.lit_objects {
                lit_object.draw(&self.gl, self.camera.view_matrix())
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

const VERTEX_DATA_STRIDE: i32 = 6;

#[rustfmt::skip]
static VERTEX_DATA: [f32; 216] = [
    /* Vertex Pos */    /* Face Normal */
    -0.5, -0.5, -0.5,    0.0,  0.0, -1.0,
     0.5, -0.5, -0.5,    0.0,  0.0, -1.0, 
     0.5,  0.5, -0.5,    0.0,  0.0, -1.0, 
     0.5,  0.5, -0.5,    0.0,  0.0, -1.0, 
    -0.5,  0.5, -0.5,    0.0,  0.0, -1.0, 
    -0.5, -0.5, -0.5,    0.0,  0.0, -1.0, 

    -0.5, -0.5,  0.5,    0.0,  0.0, 1.0,
     0.5, -0.5,  0.5,    0.0,  0.0, 1.0,
     0.5,  0.5,  0.5,    0.0,  0.0, 1.0,
     0.5,  0.5,  0.5,    0.0,  0.0, 1.0,
    -0.5,  0.5,  0.5,    0.0,  0.0, 1.0,
    -0.5, -0.5,  0.5,    0.0,  0.0, 1.0,

    -0.5,  0.5,  0.5,   -1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5,   -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,   -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,   -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5,   -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5,   -1.0,  0.0,  0.0,

     0.5,  0.5,  0.5,    1.0,  0.0,  0.0,
     0.5,  0.5, -0.5,    1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,    1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,    1.0,  0.0,  0.0,
     0.5, -0.5,  0.5,    1.0,  0.0,  0.0,
     0.5,  0.5,  0.5,    1.0,  0.0,  0.0,

    -0.5, -0.5, -0.5,    0.0, -1.0,  0.0,
     0.5, -0.5, -0.5,    0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,    0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,    0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,    0.0, -1.0,  0.0,
    -0.5, -0.5, -0.5,    0.0, -1.0,  0.0,

    -0.5,  0.5, -0.5,    0.0,  1.0,  0.0,
     0.5,  0.5, -0.5,    0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,    0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,    0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,    0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,    0.0,  1.0,  0.0];

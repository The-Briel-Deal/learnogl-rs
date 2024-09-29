use std::{fs, rc::Rc};

use crate::{
    gl::{self, create_shader, types::GLuint, Gl},
    helper::add_null_term,
};

pub trait ShaderTrait {
    fn get_id(&self) -> GLuint;

    fn enable(&self);

    fn set_bool(&self, name: &str, val: bool) -> Result<(), String>;
    fn set_int(&self, name: &str, val: i32) -> Result<(), String>;
    fn set_float(&self, name: &str, val: f32) -> Result<(), String>;
}

pub struct Shader {
    program_id: GLuint,
    gl: Rc<Gl>,
}

impl ShaderTrait for Shader {
    fn get_id(&self) -> GLuint {
        self.program_id
    }

    fn enable(&self) {
        unsafe {
            self.gl.UseProgram(self.program_id);
        }
    }

    fn set_bool(&self, name: &str, val: bool) -> Result<(), String> {
        match self.get_uniform_id(name) {
            Ok(id) => {
                self.enable();
                unsafe {
                    self.gl.Uniform1i(id, val.into());
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn set_int(&self, name: &str, val: i32) -> Result<(), String> {
        match self.get_uniform_id(name) {
            Ok(id) => {
                self.enable();
                unsafe {
                    self.gl.Uniform1i(id, val);
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn set_float(&self, name: &str, val: f32) -> Result<(), String> {
        match self.get_uniform_id(name) {
            Ok(id) => {
                self.enable();
                unsafe {
                    self.gl.Uniform1f(id, val);
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

impl Shader {
    pub fn new(gl: Rc<Gl>, vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_shader_source = fs::read(vertex_path).unwrap();
        let vertex_shader = unsafe {
            create_shader(
                &gl,
                gl::VERTEX_SHADER,
                &add_null_term(&vertex_shader_source),
            )
        };

        let fragment_shader_source = fs::read(fragment_path).unwrap();
        let fragment_shader = unsafe {
            create_shader(
                &gl,
                gl::FRAGMENT_SHADER,
                &add_null_term(&fragment_shader_source),
            )
        };

        let program_id = unsafe { gl.CreateProgram() };
        unsafe {
            gl.AttachShader(program_id, vertex_shader);
            gl.AttachShader(program_id, fragment_shader);
            gl.LinkProgram(program_id);
            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);
        };

        Self { program_id, gl }
    }

    fn get_uniform_id(&self, name: &str) -> Result<i32, String> {
        let uniform_id = unsafe {
            self.gl.GetUniformLocation(
                self.program_id,
                add_null_term(name.as_bytes()).as_ptr().cast(),
            )
        };

        if uniform_id == -1 {
            return Err(String::from("Uniform ID was returned as -1"));
        }

        Ok(uniform_id)
    }
}

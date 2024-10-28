use std::{
    alloc::{alloc, GlobalAlloc, Layout},
    borrow::Borrow,
    cell::RefCell,
    fs,
    ops::Deref,
    rc::Rc,
};

use glam::{Mat4, Vec3};

use crate::{
    gl::{
        self, create_shader,
        types::{GLfloat, GLuint},
        Gl,
    },
    helper::add_null_term,
};

pub trait ShaderTrait {
    fn get_id(&self) -> GLuint;

    fn enable(&self, gl: &Gl);

    fn set_bool(&self, gl: &Gl, name: &str, val: bool) -> Result<(), String>;
    fn set_int(&self, gl: &Gl, name: &str, val: i32) -> Result<(), String>;
    fn set_float(&self, gl: &Gl, name: &str, val: f32) -> Result<(), String>;
    fn set_vec2(&self, gl: &Gl, name: &str, val: (f32, f32)) -> Result<(), String>;
    fn set_vec3(&self, gl: &Gl, name: &str, val: (f32, f32, f32)) -> Result<(), String>;
    fn get_vec3(&self, gl: &Gl, name: &str) -> Result<Vec3, String>;
    fn set_mat4(&self, gl: &Gl, name: &str, val: Mat4) -> Result<(), String>;
}

#[derive(Clone)]
pub struct Shader {
    program_id: GLuint,
}

impl ShaderTrait for Shader {
    fn get_id(&self) -> GLuint {
        self.program_id
    }

    fn enable(&self, gl: &Gl) {
        unsafe {
            gl.UseProgram(self.program_id);
        }
    }

    fn set_bool(&self, gl: &Gl, name: &str, val: bool) -> Result<(), String> {
        match self.get_uniform_id(gl, name) {
            Ok(id) => {
                self.enable(gl);
                unsafe {
                    gl.Uniform1i(id, val.into());
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn set_int(&self, gl: &Gl, name: &str, val: i32) -> Result<(), String> {
        match self.get_uniform_id(gl, name) {
            Ok(id) => {
                self.enable(gl);

                unsafe {
                    gl.Uniform1i(id, val);
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn set_float(&self, gl: &Gl, name: &str, val: f32) -> Result<(), String> {
        match self.get_uniform_id(gl, name) {
            Ok(id) => {
                self.enable(gl);
                unsafe {
                    gl.Uniform1f(id, val);
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn set_vec2(&self, gl: &Gl, name: &str, val: (f32, f32)) -> Result<(), String> {
        match self.get_uniform_id(gl, name) {
            Ok(id) => {
                self.enable(gl);
                unsafe {
                    gl.Uniform2f(id, val.0, val.1);
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
    fn set_vec3(&self, gl: &Gl, name: &str, val: (f32, f32, f32)) -> Result<(), String> {
        match self.get_uniform_id(gl, name) {
            Ok(id) => {
                self.enable(gl);
                unsafe {
                    gl.Uniform3f(id, val.0, val.1, val.2);
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
    fn get_vec3(&self, gl: &Gl, name: &str) -> Result<Vec3, String> {
        match self.get_uniform_id(gl, name) {
            Ok(id) => {
                self.enable(gl);
                unsafe {
                    let params: *mut GLfloat = [0.0_f32, 0.0_f32, 0.0_f32].as_mut_ptr();
                    gl.GetUniformfv(self.program_id, id, params);
                    let x = *params;
                    let y = *params.add(1);
                    let z = *params.add(2);
                    Ok(Vec3 { x, y, z })
                }
            }
            Err(err) => Err(err),
        }
    }
    fn set_mat4(&self, gl: &Gl, name: &str, val: Mat4) -> Result<(), String> {
        match self.get_uniform_id(gl, name) {
            Ok(id) => {
                self.enable(gl);
                unsafe {
                    gl.UniformMatrix4fv(id, 1, gl::FALSE, val.as_ref().as_ptr());
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

impl Shader {
    pub fn new(gl: &Gl, vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_shader_source = fs::read(vertex_path).unwrap();
        let vertex_shader =
            unsafe { create_shader(gl, gl::VERTEX_SHADER, &add_null_term(&vertex_shader_source)) };

        let fragment_shader_source = fs::read(fragment_path).unwrap();
        let fragment_shader = unsafe {
            create_shader(
                gl,
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

        Self { program_id }
    }

    fn get_uniform_id(&self, gl: &Gl, name: &str) -> Result<i32, String> {
        let uniform_id = unsafe {
            gl.GetUniformLocation(
                self.program_id,
                add_null_term(name.as_bytes()).as_ptr().cast(),
            )
        };

        if uniform_id == -1 {
            return Err(String::from("Uniform ID was returned as -1"));
        }

        Ok(uniform_id)
    }
    pub fn get_uniform(&self, gl: &Gl, name: &str) -> Uniform {
        let id = self.get_uniform_id(gl, name);
        Uniform {
            gl: gl.clone(),
            uniform_id: id.unwrap(),
            val: RefCell::new(Vec3::ZERO),
            shader_id: self.program_id,
        }
    }
}

pub struct Uniform {
    gl: Gl,
    shader_id: u32,
    uniform_id: i32,
    val: RefCell<Vec3>,
}

impl Deref for Uniform {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let params: *mut f32 = [0.0, 0.0, 0.0].as_mut_ptr();
            self.gl
                .GetUniformfv(self.shader_id, self.uniform_id, params);
            let x = *params.add(0);
            let y = *params.add(1);
            let z = *params.add(2);
            *self.val.borrow_mut() = Vec3 { x, y, z };
        };
        unsafe { &*self.val.as_ptr() }
    }
}

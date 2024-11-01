use std::{fs, marker::PhantomData, mem::zeroed};

use glam::{Mat4, Vec3};

use crate::{
    gl::{
        self, create_shader,
        types::{GLfloat, GLsizei, GLuint},
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
}

pub trait DrawableShader {
    fn model(&self) -> &Uniform<Mat4>;
    fn view(&self) -> &Uniform<Mat4>;
    fn projection(&self) -> &Uniform<Mat4>;
    fn shader(&self) -> &Shader;
}

pub struct LightCasterShader {
    pub shader: Shader,
    pub model: Uniform<Mat4>,
    pub view: Uniform<Mat4>,
    pub projection: Uniform<Mat4>,
}

impl LightCasterShader {
    pub fn new(gl: &Gl) -> Self {
        let shader = Shader::new(
            gl,
            "src/shader/light_casters_vert.glsl",
            "src/shader/light_casters_frag.glsl",
        );
        let model = Uniform::new(gl, &shader, "model");
        let view = Uniform::new(gl, &shader, "view");
        let projection = Uniform::new(gl, &shader, "projection");
        Self {
            shader,
            model,
            view,
            projection,
        }
    }
}
pub struct LightCubeShader {
    pub shader: Shader,
    pub model: Uniform<Mat4>,
    pub view: Uniform<Mat4>,
    pub projection: Uniform<Mat4>,
}

impl LightCubeShader {
    pub fn new(gl: &Gl) -> Self {
        let shader = Shader::new(
            gl,
            "src/shader/light_cube_vert.glsl",
            "src/shader/light_cube_frag.glsl",
        );
        let model = Uniform::new(gl, &shader, "model");
        let view = Uniform::new(gl, &shader, "view");
        let projection = Uniform::new(gl, &shader, "projection");
        Self {
            shader,
            model,
            view,
            projection,
        }
    }
}

impl DrawableShader for LightCubeShader {
    fn model(&self) -> &Uniform<Mat4> {
        &self.model
    }
    fn view(&self) -> &Uniform<Mat4> {
        &self.view
    }
    fn projection(&self) -> &Uniform<Mat4> {
        &self.projection
    }
    fn shader(&self) -> &Shader {
        &self.shader
    }
}

impl DrawableShader for LightCasterShader {
    fn model(&self) -> &Uniform<Mat4> {
        &self.model
    }
    fn view(&self) -> &Uniform<Mat4> {
        &self.view
    }
    fn projection(&self) -> &Uniform<Mat4> {
        &self.projection
    }
    fn shader(&self) -> &Shader {
        &self.shader
    }
}

pub struct Uniform<T> {
    gl: Gl,
    shader_id: u32,
    uniform_id: i32,
    resource_type: PhantomData<T>,
}

impl<T> Uniform<T> {
    pub fn new(gl: &Gl, shader: &Shader, name: &str) -> Self {
        let uniform_id = shader.get_uniform_id(gl, name).unwrap();
        Self {
            gl: gl.clone(),
            shader_id: shader.program_id,
            uniform_id,
            resource_type: PhantomData,
        }
    }
}

pub trait UniformGetSet<T> {
    fn get(&self) -> T;
    fn set(&self, val: T);
}

impl UniformGetSet<Vec3> for Uniform<Vec3> {
    fn get(&self) -> Vec3 {
        unsafe {
            let params: *mut f32 = [0.0, 0.0, 0.0].as_mut_ptr();
            self.gl
                .GetUniformfv(self.shader_id, self.uniform_id, params);
            let x = *params.add(0);
            let y = *params.add(1);
            let z = *params.add(2);

            Vec3 { x, y, z }
        }
    }
    fn set(&self, val: Vec3) {
        unsafe {
            self.gl
                .ProgramUniform3f(self.shader_id, self.uniform_id, val.x, val.y, val.z)
        }
    }
}

impl UniformGetSet<f32> for Uniform<f32> {
    fn get(&self) -> f32 {
        unsafe {
            let params: *mut f32 = [0.0].as_mut_ptr();
            self.gl
                .GetUniformfv(self.shader_id, self.uniform_id, params);
            *params
        }
    }
    fn set(&self, val: f32) {
        unsafe {
            self.gl
                .ProgramUniform1f(self.shader_id, self.uniform_id, val)
        }
    }
}

impl UniformGetSet<Mat4> for Uniform<Mat4> {
    fn get(&self) -> Mat4 {
        unsafe {
            let mut params: [f32; 16] = zeroed();
            let params_ptr: *mut f32 = params.as_mut_ptr();
            self.gl
                .GetUniformfv(self.shader_id, self.uniform_id, params_ptr);
            Mat4::from_cols_array(&params)
        }
    }
    fn set(&self, val: Mat4) {
        const COUNT: GLsizei = 1;
        unsafe {
            self.gl.ProgramUniformMatrix4fv(
                self.shader_id,
                self.uniform_id,
                COUNT,
                gl::FALSE,
                val.as_ref().as_ptr(),
            )
        }
    }
}

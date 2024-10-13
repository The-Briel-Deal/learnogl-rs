use std::{collections::HashMap, os::raw::c_void};

use image::ImageReader;

use crate::{
    gl::{self, types::GLuint, Gl},
    shader::{Shader, ShaderTrait},
};

#[derive(Default)]
pub struct TextureManager {
    texture_name_map: HashMap<String, GLuint>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_texture(
        &mut self,
        gl: &Gl,
        name: &str,
        path: &str,
        program: &Shader,
        index: i32,
    ) {
        let img = ImageReader::open(path).unwrap().decode().unwrap().flipv();

        let img_height = img.height();
        let img_width = img.width();
        let data = img.to_rgba8();

        let mut texture: GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img_width as i32,
                img_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        };
        dbg!(texture);
        program.set_int(name, index).unwrap();

        self.texture_name_map.insert(name.to_string(), texture);
    }

    pub fn bind_texture(&self, gl: &Gl, name: &str, unit: GLuint) {
        let texture = self.get_texture(name).unwrap();
        unsafe {
            gl.BindTextureUnit(unit, *texture);
        }
    }

    fn get_texture(&self, name: &str) -> Option<&GLuint> {
        self.texture_name_map.get(name)
    }
}

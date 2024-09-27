#![allow(clippy::all)]
include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

use std::ffi::CStr;

use types::{GLenum, GLuint};
pub use Gles2 as Gl;

pub fn get_gl_string(gl: &Gl, variant: GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

pub unsafe fn create_shader(gl: &Gl, shader: GLenum, source: &[u8]) -> GLuint {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(
        shader,
        1,
        [source.as_ptr().cast()].as_ptr(),
        std::ptr::null(),
    );
    gl.CompileShader(shader);
    shader
}

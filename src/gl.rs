#![allow(clippy::all)]
include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

mod uniform;

use std::ffi::CStr;

use glutin::{
    config::Config,
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Version},
    display::GetGlDisplay,
    prelude::GlDisplay,
};
use types::{GLenum, GLuint};
use winit::{raw_window_handle::HasWindowHandle, window::Window};

impl Gl {
    pub fn get_aspect_ratio(&self) -> f32 {
        let mut data: [types::GLint; 4] = [0, 0, 0, 0];
        let data_ptr = data.as_mut_ptr();

        unsafe { self.GetIntegerv(VIEWPORT, data_ptr) };

        let x = data[0];
        let y = data[1];
        let width = data[2];
        let height = data[3];

        (width - x) as f32 / (height - y) as f32
    }
}

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

pub fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    let context_attributes = ContextAttributesBuilder::new()
        .with_debug(true)
        .build(raw_window_handle);

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    }
}

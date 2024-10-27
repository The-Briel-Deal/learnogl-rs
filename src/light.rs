mod direction;
mod point;
mod spot;

use glam::{Mat4, Vec3};

use crate::gl::{types::GLfloat, Gl};

pub use self::spot::SpotLight;

pub trait Light {
    fn pos(&self) -> Vec3;
    fn set_pos(&mut self, gl: &Gl, pos: Vec3) -> &mut dyn Light;

    fn dir(&self) -> Vec3;
    fn set_dir(&mut self, gl: &Gl, dir: Vec3) -> &mut dyn Light;

    fn draw(&self, _gl: &Gl, _view_matrix: Mat4) {}
    fn adjust_zoom(&mut self, _degrees: GLfloat) {}
}

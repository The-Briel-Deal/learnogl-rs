use std::{ cell::RefCell, ops::Mul};

use glam::{vec3, Mat4, Vec3};

const WORLD_ORIGIN: Vec3 = vec3(0.0, 0.0, 0.0);

pub struct Camera {
    pos: RefCell<Vec3>,
    _dir: Vec3,
    _right: Vec3,
    up: Vec3,

    front: Vec3,

    _target: Vec3,

    rotation: RefCell<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera::default()
    }

    pub fn rotate(&self) {
        let mut rotation = self.rotation.borrow_mut();
        let mut pos = self.pos.borrow_mut();

        *rotation += 0.1;
        let radius = 10.0_f32;

        pos.x = rotation.to_radians().sin().mul(radius);
        pos.z = rotation.to_radians().cos().mul(radius);
    }

    pub fn look_at_target(&self) -> Mat4 {
        Mat4::look_at_rh(*self.pos.borrow(), *self.pos.borrow() + self.front, self.up)
    }
    pub fn move_right(&self, distance: f32) {
        let mut pos = self.pos.borrow_mut();
        *pos -= self.front.cross(self.up).normalize().mul(distance);
    }
    pub fn move_left(&self, distance: f32) {
        let mut pos = self.pos.borrow_mut();
        *pos += self.front.cross(self.up).normalize().mul(distance);
    }
    pub fn move_forward(&self, distance: f32) {
        let mut pos = self.pos.borrow_mut();
        *pos += self.front * distance;
    }
    pub fn move_backward(&self, distance: f32) {
        let mut pos = self.pos.borrow_mut();
        *pos -= self.front * distance;
    }
}

impl Default for Camera {
    fn default() -> Self {
        let camera_pos = vec3(0.0, 0.0, 3.0);

        let camera_target = WORLD_ORIGIN;
        let camera_dir = (camera_pos - camera_target).normalize();

        let up = vec3(0.0, 1.0, 0.0);
        let camera_right = up.cross(camera_dir);

        let camera_up = camera_dir.cross(camera_right);
        Camera {
            pos: RefCell::new(camera_pos),
            _dir: camera_dir,
            _right: camera_right,
            up: camera_up,

            front: vec3(0.0, 0.0, -1.0),

            _target: camera_target,

            rotation: RefCell::new(0.0),
        }
    }
}

mod direction;
use std::cell::RefCell;

use direction::{Degrees, Direction};
use glam::{vec3, Mat4, Vec3};
use winit::keyboard::KeyCode;

const SPEED: f32 = 2.0;

pub struct Camera {
    pos: RefCell<Vec3>,
    dir: Direction,
    up: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera::default()
    }

    pub fn look_at_target(&self) -> Mat4 {
        Mat4::look_at_rh(
            *self.pos.borrow(),
            *self.pos.borrow() + self.dir.euler(),
            self.up,
        )
    }
    fn get_right_dir(&self) -> Vec3 {
        -self.dir.euler().cross(self.up).normalize()
    }
    fn get_left_dir(&self) -> Vec3 {
        self.dir.euler().cross(self.up).normalize()
    }
    fn get_forwards_dir(&self) -> Vec3 {
        self.dir.euler()
    }
    fn get_backwards_dir(&self) -> Vec3 {
        -self.dir.euler()
    }

    pub fn handle_movement(&self, keys: Vec<KeyCode>, delta_time: f32) {
        let mut dir = Vec3::ZERO;
        for key in keys {
            dir += match key {
                KeyCode::KeyW => self.get_forwards_dir(),
                KeyCode::KeyA => self.get_right_dir(),
                KeyCode::KeyS => self.get_backwards_dir(),
                KeyCode::KeyD => self.get_left_dir(),
                _ => panic!("Key passed to handle movement that wasn't expected."),
            }
        }
        let mut camera_position = self.pos.borrow_mut();
        *camera_position += dir.normalize_or_zero() * SPEED * delta_time;
    }

    pub fn pitch(&self) -> Degrees {
        self.dir.pitch()
    }
    pub fn set_pitch(&self, pitch: Degrees) {
        self.dir.set_pitch(pitch);
    }
    pub fn adjust_pitch(&self, pitch: Degrees) {
        self.dir.adjust_pitch(pitch);
    }

    pub fn yaw(&self) -> Degrees {
        self.dir.yaw()
    }
    pub fn set_yaw(&self, yaw: Degrees) {
        self.dir.set_yaw(yaw);
    }
    pub fn adjust_yaw(&self, yaw: Degrees) {
        self.dir.adjust_yaw(yaw);
    }
}

impl Default for Camera {
    fn default() -> Self {
        let camera_pos = vec3(0.0, 0.0, 3.0);

        let camera_dir = Direction::new();

        let up = vec3(0.0, 1.0, 0.0);
        let camera_right = up.cross(camera_dir.euler());

        let camera_up = camera_dir.euler().cross(camera_right);
        Camera {
            pos: RefCell::new(camera_pos),
            dir: camera_dir,
            up: camera_up,
        }
    }
}

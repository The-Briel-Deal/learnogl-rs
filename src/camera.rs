use std::{
    cell::RefCell,
    ops::Mul,
};

use glam::{vec3, Mat4, Vec3};

const WORLD_ORIGIN: Vec3 = vec3(0.0, 0.0, 0.0);
type Degrees = f32;
struct InnerDirection {
    yaw: Degrees,
    pitch: Degrees,
    euler_dir: Vec3,
}
pub struct Direction {
    inner_direction: RefCell<InnerDirection>,
}

impl Direction {
    fn new() -> Self {
        Self::default()
    }
    fn update_euler(&self) {
        let mut inner = self.inner_direction.borrow_mut();
        let yaw = inner.yaw;
        let pitch = inner.pitch;
        inner.euler_dir = Vec3 {
            x: yaw.to_radians().cos() * pitch.to_radians().cos(),
            y: pitch.to_radians().sin(),
            z: yaw.to_radians().sin() * pitch.to_radians().cos(),
        };
    }

    fn euler(&self) -> Vec3 {
        self.inner_direction.borrow().euler_dir
    }

    fn yaw(&self) -> Degrees {
        self.inner_direction.borrow().yaw
    }
    fn set_yaw(&self, yaw: Degrees) {
        self.inner_direction.borrow_mut().yaw = yaw;
        self.update_euler();
    }
    /// Adjusts yaw by the specified degrees (via adding).
    fn adjust_yaw(&self, yaw: Degrees) {
        self.inner_direction.borrow_mut().yaw += yaw;
        self.update_euler();
    }

    fn pitch(&self) -> Degrees {
        self.inner_direction.borrow().pitch
    }
    fn set_pitch(&self, pitch: Degrees) {
        self.inner_direction.borrow_mut().pitch = pitch;
        self.update_euler();
    }
    /// Adjusts pitch by the specified degrees (via adding).
    fn adjust_pitch(&self, pitch: Degrees) {
        self.inner_direction.borrow_mut().pitch += pitch;
        self.update_euler();
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            inner_direction: RefCell::new(InnerDirection {
                yaw: 0.0,
                pitch: 0.0,
                euler_dir: vec3(0.0, 0.0, -1.0),
            }),
        }
    }
}

pub struct Camera {
    pos: RefCell<Vec3>,
    dir: Direction,
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
        Mat4::look_at_rh(
            *self.pos.borrow(),
            *self.pos.borrow() + self.dir.euler(),
            self.up,
        )
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

        let camera_target = WORLD_ORIGIN;
        let camera_dir = Direction::new();

        let up = vec3(0.0, 1.0, 0.0);
        let camera_right = up.cross(camera_dir.euler());

        let camera_up = camera_dir.euler().cross(camera_right);
        Camera {
            pos: RefCell::new(camera_pos),
            dir: camera_dir,
            _right: camera_right,
            up: camera_up,

            front: vec3(0.0, 0.0, -1.0),

            _target: camera_target,

            rotation: RefCell::new(0.0),
        }
    }
}

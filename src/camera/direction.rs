use std::cell::RefCell;

use glam::{vec3, Vec3};

pub type Degrees = f32;
struct InnerDirection {
    yaw: Degrees,
    pitch: Degrees,
    euler_dir: Vec3,
}
pub struct Direction {
    inner_direction: RefCell<InnerDirection>,
}

impl Direction {
    pub fn new() -> Self {
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

    pub fn euler(&self) -> Vec3 {
        self.inner_direction.borrow().euler_dir
    }

    pub fn yaw(&self) -> Degrees {
        self.inner_direction.borrow().yaw
    }
    pub fn set_yaw(&self, yaw: Degrees) {
        self.inner_direction.borrow_mut().yaw = yaw;
        self.update_euler();
    }
    /// Adjusts yaw by the specified degrees (via adding).
    pub fn adjust_yaw(&self, yaw: Degrees) {
        self.set_yaw(self.yaw() + yaw);
        self.update_euler();
    }

    pub fn pitch(&self) -> Degrees {
        self.inner_direction.borrow().pitch
    }
    pub fn set_pitch(&self, pitch: Degrees) {
        // Clamping pitch to the 180 degrees in front of you.
        self.inner_direction.borrow_mut().pitch = pitch.clamp(-89.0, 89.0);
        self.update_euler();
    }
    /// Adjusts pitch by the specified degrees (via adding).
    pub fn adjust_pitch(&self, pitch: Degrees) {
        self.set_pitch(self.pitch() + pitch);
        self.update_euler();
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            inner_direction: RefCell::new(InnerDirection {
                yaw: -90.0,
                pitch: 0.0,
                euler_dir: vec3(0.0, 0.0, -1.0),
            }),
        }
    }
}

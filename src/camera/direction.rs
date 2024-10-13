use glam::{vec3, Vec3};

pub type Degrees = f32;
pub struct Direction {
    yaw: Degrees,
    pitch: Degrees,
    euler_dir: Vec3,
}

impl Direction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn euler(&self) -> Vec3 {
        self.euler_dir
    }
    fn update_euler(&mut self) {
        let yaw = self.yaw;
        let pitch = self.pitch;
        self.euler_dir = Vec3 {
            x: yaw.to_radians().cos() * pitch.to_radians().cos(),
            y: pitch.to_radians().sin(),
            z: yaw.to_radians().sin() * pitch.to_radians().cos(),
        };
    }

    pub fn yaw(&self) -> Degrees {
        self.yaw
    }
    pub fn set_yaw(&mut self, yaw: Degrees) {
        self.yaw = yaw;
        self.update_euler();
    }
    pub fn adjust_yaw(&mut self, yaw: Degrees) {
        self.set_yaw(self.yaw() + yaw);
        self.update_euler();
    }

    pub fn pitch(&self) -> Degrees {
        self.pitch
    }
    pub fn set_pitch(&mut self, pitch: Degrees) {
        self.pitch = pitch.clamp(-89.0, 89.0);
        self.update_euler();
    }
    pub fn adjust_pitch(&mut self, pitch: Degrees) {
        self.set_pitch(self.pitch() + pitch);
        self.update_euler();
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            yaw: -90.0,
            pitch: 0.0,
            euler_dir: vec3(0.0, 0.0, -1.0),
        }
    }
}

use std::time::Instant;

pub struct Timer {
    last_instant: Instant,
    last_delta: f32,
    elapsed: f32,
}

impl Timer {
    /// Time since last call to delta time in seconds.
    pub fn delta_time(&self) -> f32 {
        self.last_delta
    }

    pub fn reset(&mut self) {
        self.last_delta = self.last_instant.elapsed().as_secs_f32();
        self.elapsed += self.last_delta;
        self.last_instant = Instant::now();
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            last_instant: Instant::now(),
            last_delta: 0.0,
            elapsed: 0.0,
        }
    }
}

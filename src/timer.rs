use std::{cell::RefCell, time::Instant};

pub struct Timer {
    last_instant: RefCell<Instant>,
    last_delta: RefCell<f32>,
}

impl Timer {
    /// Time since last call to delta time in seconds.
    pub fn delta_time(&self) -> f32 {
        *self.last_delta.borrow()
    }

    pub fn reset(&self) {
        *self.last_delta.borrow_mut() = self.last_instant.borrow().elapsed().as_secs_f32();
        *self.last_instant.borrow_mut() = Instant::now();
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            last_instant: RefCell::new(Instant::now()),
            last_delta: RefCell::new(0.0),
        }
    }
}

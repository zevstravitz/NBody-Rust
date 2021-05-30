use js_sys::Math::random;
use std::fmt;
use crate::{AVG_RADIUS, WINDOW_X};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Particle {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
    pub radius: usize
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.x_pos, self.y_pos)
    }
}

#[wasm_bindgen]
impl Particle {
    // pub fn get_me(ptr: &Particle) -> Particle {
    //     *ptr
    // }

    pub fn get_x(&self) -> i32 {
        self.x_pos
    }

    pub fn get_y(&self) -> i32 {
        self.y_pos
    }
}

impl Particle {
    pub(crate) fn new() -> Particle {
        // Create normal distribution
        // let mut rng = rand::thread_rng();
        // let normal = Normal::new(crate::AVG_RADIUS, 10);

        // Radius of circle
        let radius = (random() as usize) * crate::AVG_RADIUS;

        // Return new Particle
        Particle {
            x_pos: (random() as i32) * crate::WINDOW_X,
            y_pos: (random() as i32) * crate::WINDOW_Y,
            x_vel: ((random() as i32) * 20 + -10),
            y_vel: ((random() as i32) * 20 + -10),
            radius
        }
    }

    pub fn update_position(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
    }
}
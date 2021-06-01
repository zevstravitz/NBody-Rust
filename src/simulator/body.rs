use js_sys::Math::random;
use std::fmt;
use crate::{AVG_RADIUS, WINDOW_X, DIMENSIONS};
use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub pos: [f64; DIMENSIONS],
    pub vel: [f64; DIMENSIONS],
    pub radius: u32
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.pos[0], self.pos[1])
    }
}

impl Body {
    pub(crate) fn new() -> Body {
        // Create normal distribution
        // let mut rng = rand::thread_rng();
        // let normal = Normal::new(crate::AVG_RADIUS, 10);

        // Radius of circle
        let radius = (random() * (crate::AVG_RADIUS as f64)) as u32;

        // Return new Body
        Body {
            pos: [random() * 40.0 + 400.0,
                    random()  * 40.0 + 400.0,
                    random() * 40.0 + 400.0],
            vel: [(random()  * 20.0 + -5.0),
                  (random() * 10.0 + -5.0),
                  (random() * 10.0 + -5.0)],
            radius
        }
    }

    pub fn get_x(&self) -> f64 {
        self.pos[0]
    }

    pub fn get_y(&self) -> f64 {
        self.pos[1]
    }

    pub fn dist(&self, other: &Body) -> f64 {
        let mut temp = 0.0;
        for dim in 0..DIMENSIONS {
            temp += (other.pos[dim] - self.pos[dim])
        }
        (temp).powf(0.5)
    }

    pub fn unit_to(&self, other: &Body, dist: f64) -> [f64; DIMENSIONS] {
        let mut unit = [0.0; DIMENSIONS];
        for dim in 0..DIMENSIONS {
            unit[dim] = (self.pos[dim] - self.pos[dim])/dist
        }
        unit
    }

    pub fn update_position(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
    }
}
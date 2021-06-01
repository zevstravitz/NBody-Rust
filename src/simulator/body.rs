use js_sys::Math::random;
use std::fmt;
use crate::{AVG_RADIUS, WINDOW_X, DIMENSIONS};
use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub pos: [f64; DIMENSIONS],
    pub vel: [f64; DIMENSIONS],
    pub radius: f64
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
        let radius = crate::AVG_RADIUS;
        // alt:  (random() * 5.0) + crate::AVG_RADIUS;

        // Return new Body
        Body {
            pos: [250.0 + random() * 500.0,
                  250.0 + random()  * 500.0,
                   random() * 1000.0],
            vel: [(random()  * 10.0 + -5.0),
                  (random() * 10.0 + -5.0),
                  (random() * 10.0 + -5.0)],
            radius
        }
    }

    pub fn get_dim(&self, dim: usize) -> f64 {
        self.pos[dim]
    }

    pub fn dist(&self, other: &Body) -> f64 {
        let mut temp = 0.0;
        for dim in 0..DIMENSIONS {
            temp += (other.pos[dim] - self.pos[dim]).powi(2)
        }
        (temp).powf(0.5)
    }

    pub fn unit_to(&self, other: &Body, dist: f64) -> [f64; DIMENSIONS] {
        let mut unit = [0.0; DIMENSIONS];
        for dim in 0..DIMENSIONS {
            unit[dim] = (other.pos[dim] - self.pos[dim])/dist
        }
        unit
    }

    pub fn update_position(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
    }
}
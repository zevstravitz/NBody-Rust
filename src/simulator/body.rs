use js_sys::Math::random;
use std::fmt;
use crate::{AVG_RADIUS, WINDOW_X};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Body {
    pub x_pos: f64,
    pub y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    pub radius: usize
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.x_pos, self.y_pos)
    }
}

impl Body {
    // pub fn get_me(ptr: &Body) -> Body {
    //     *ptr
    // }

    pub fn get_x(&self) -> f64 {
        self.x_pos
    }

    pub fn get_y(&self) -> f64 {
        self.y_pos
    }
}

impl Body {
    pub(crate) fn new() -> Body {
        // Create normal distribution
        // let mut rng = rand::thread_rng();
        // let normal = Normal::new(crate::AVG_RADIUS, 10);

        // Radius of circle
        let radius = (random() as usize) * crate::AVG_RADIUS;

        // Return new Body
        Body {
            x_pos: random() * (crate::WINDOW_X as f64),
            y_pos: random() * (crate::WINDOW_Y as f64),
            x_vel: (random() * 20.0 + -10.0),
            y_vel: (random() * 20.0 + -10.0),
            radius
        }
    }

    pub fn update_position(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
    }
}
use rayon::prelude::*;
use wasm_bindgen::prelude::*;
extern crate wasm_bindgen;
use std::mem::size_of;

use crate::utils;
use crate::log;

mod body;
use body::{Body};

mod models;

#[wasm_bindgen]
pub struct Simulator {
    bodies: Vec<Body>
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
impl Simulator {
    pub fn new(num_bodies: usize) -> Simulator {
        utils::set_panic_hook();

        let mut bodies: Vec<Body> = Vec::with_capacity(num_bodies);
        for idx in 0..num_bodies {
            // log!("{}", idx);
            bodies.push(Body::new());
        }

        Simulator {
            bodies
        }
    }

    pub fn get_x(&self, i: usize) -> f64 {
        if let Some(body) = self.bodies.get(i) {
            return body.get_x();
        }
        0.0
    }

    pub fn get_y(&self, i: usize) -> f64 {
        if let Some(body) = self.bodies.get(i) {
            return body.get_y();
        }
        0.0
    }

    pub fn next_state(&mut self) {
        self.update_positions();
        self.update_velocities();
    }

    pub fn bodies(&self) -> *const Body {
        self.bodies.as_ptr()
    }

    // this probably shouldn't be here
    pub fn body_size(&self) -> usize {
        size_of::<Body>()
    }
    
    pub fn greet(&self, name: &str) {
        alert(&format!("Hello, {}!", name));
    }
}

impl Simulator {
    fn update_positions(&mut self) {
        // log!("{:?}", self.bodies);
        // self.bodies.par_iter_mut().for_each(|body: &mut Body| {
        //     body.update_position();
        // });
        for body in self.bodies.iter_mut() {
            body.update_position();
        }
    }

    fn update_velocities(&mut self) {
        models::all_body_update(self);
    }
}
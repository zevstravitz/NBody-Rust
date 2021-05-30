mod particle;
use particle::{Particle};
use rayon::prelude::*;
use wasm_bindgen::prelude::*;
extern crate wasm_bindgen;
use std::mem::size_of;

#[wasm_bindgen]
pub struct Simulator {
    particles: Vec<Particle>
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
impl Simulator {
    pub fn new(num_particles: usize) -> Simulator {
        let mut particles: Vec<Particle> = Vec::with_capacity(num_particles);
        for idx in 0..num_particles {
            particles.push(Particle::new());
        }

        Simulator {
            particles
        }
    }

    pub fn next_state(&mut self) {
        self.update_positions();
        // self.update_velocities();
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    // this probably shouldn't be here
    pub fn particle_size(&self) -> usize {
        size_of::<Particle>()
    }
    
    pub fn greet(&self, name: &str) {
        alert(&format!("Hello, {}!", name));
    }
}


impl Simulator {
    fn update_positions(&mut self) {
        println!("{:?}", self.particles);
        self.particles.par_iter_mut().for_each(|particle: &mut Particle| {
            particle.update_position();
        });
        println!("Done here...");
    }

    fn update_velocities(&mut self) {}
}
mod particle;
use particle::{Particle};
use rayon::prelude::*;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub(crate) struct Simulator {
    pub(crate) particles: Vec<Particle>
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

        return Simulator {
            particles
        }
    }
    
    pub fn greet(name: &str) {
        alert(&format!("Hello, {}!", name));
    }

    pub fn next_state(&mut self) {
        self.update_positions();
        // self.update_velocities();
    }

    fn update_positions(&mut self) {
        println!("{:?}", self.particles);
        self.particles.par_iter_mut().for_each(|particle: &mut Particle| {
            particle.update_position();
        });
        println!("Done here...");
    }

    fn update_velocities(&mut self) {
        ;
    }
}
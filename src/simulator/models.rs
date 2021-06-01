use crate::simulator::{Simulator, Body};
use crate::{G_CONSTANT, DIMENSIONS, DAMPENING_FACTOR};

pub fn all_body_update(sim: &mut Simulator) {
    for i in 0..sim.bodies.len() {
        let mut d_v = [0.0; DIMENSIONS];
        for j in 0..sim.bodies.len() {
            if i == j {
                continue
            }

            let dist = sim.bodies[i].dist(&sim.bodies[j]);
            let unit_vec = sim.bodies[i].unit_to(&sim.bodies[j], dist);

            let numerator = (G_CONSTANT * (sim.bodies[i].radius as f64) * (sim.bodies[j].radius as f64))/(dist.powi(2) + (DAMPENING_FACTOR as f64).powi(2));
            for dim in 0..DIMENSIONS {
                d_v[dim] = numerator * unit_vec[dim];
            }
        }

        for dim in 0..DIMENSIONS {
            sim.bodies[i].vel[dim] += d_v[dim];
        }
    }
}
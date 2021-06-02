use crate::simulator::{Simulator, Body};
use crate::{G_CONSTANT, DIMENSIONS, DAMPENING_FACTOR};
use crate::log;

pub fn all_body_update(sim: &mut Simulator) {
    for i in 0..sim.bodies.capacity() {
        let mut d_v = [0.0; DIMENSIONS];
        for j in 0..sim.bodies.capacity() {
            if i == j {
                continue
            }

            // Distance between particles
            let dist = sim.bodies[i].dist(&sim.bodies[j]);

            // Force unit vector -> Indicates direction
            let unit_vec = sim.bodies[i].unit_to(&sim.bodies[j], dist);

            // Force Scalar
            let force = (G_CONSTANT * (sim.bodies[i].mass) * (sim.bodies[j].mass))/(dist.powi(2) + DAMPENING_FACTOR.powi(2));

            // Get dV Vector
            for dim in 0..DIMENSIONS {
                d_v[dim] = force * unit_vec[dim];
            }
        }

        // Apply dV vector to
        for dim in 0..DIMENSIONS {
            sim.bodies[i].vel[dim] += d_v[dim];
        }
    }
}

pub fn barnes_hut_update(sim: &mut Simulator) {}

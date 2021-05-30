use js_sys::Math::random;
use std::fmt;
use crate::{AVG_RADIUS, WINDOW_X};

#[derive(Debug)]
pub struct Particle {
    pos: [isize; 2],
    pub vel: [isize; 2],
    pub radius: usize
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.pos[0], self.pos[1])
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
            pos: [
                (random() as isize) * crate::WINDOW_X,
                (random() as isize) * crate::WINDOW_Y
            ],
            vel: [
                ((random() as isize) * 20 + -10),
                ((random() as isize) * 20 + -10)
            ],
            radius
        }
    }

    pub fn update_position(&mut self) {
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
    }
}
/// Logic
/// 
/// 1) Initialize Octree 
/// 
/// 2) Insert all bodies in Octree
///   a) For a given body, determine it's quadrant
///     i) If the octent is empty -> put body in.  
///     ii) If the octent is an internal node, update the node and and Insert() again
///     iii) If octent is an external node. Replace with internal node and put both bodies in that internal node again using Insert()
/// 3) Calcuate force on all bodies

/// Functions
/// fn new() -> Octree (Initializes empty octree with root node and 8 child octents)
/// fn insert(body) -> None takes a body and inserts in the quadtree
/// fn body_to_quadrant(&mut self, body) -> None (maps to octant)
/// fn update_force(&mut self, body) -> None (Updates the force for all nodes)
/// 

use crate::{DIMENSIONS, BOUNDS};
mod body;

struct Octant {
    content: BHTree,
    min: [f64; DIMENSIONS],
    max: [f64; DIMENSIONS]
}

enum BHTree {
    Empty,
    Internal(Octree),
    External(body::Body)
}

struct Octree {
    com: Option<COM>,
    children: [Octant; 8]
}

struct COM {
    pos: [f64; 3]
}


impl Octree {
    fn new(sim: &mut sim) -> {
        let mut octree = Octant {
            content: BHTree::Empty,
            min: [BOUNDS; DIMENSIONS],
            max: [-1*BOUNDS; DIMENSIONS]
        }

        self.insert_bodies(&sim.bodies)
    }

    fn insert_bodies(&mut self, bodies: &Vec<Body>) {
        for body in bodies.iter() {
            self.insert(&body)
        }
    }

    fn insert(&mut self, &body: Body) {
        //insert body with x,y 
        // if children(x,y) -> empty 
        // add it there 

        oct = find_quadrant
        
    }

    fn find_quadrant(&self, body: &Body) -> Octree {
        
    }
}



// public class BHTree {
//     private Body body;     // body or aggregate body stored in this node
//     private Quad quad;     // square region that the tree represents
//     private BHTree NW;     // tree representing northwest quadrant
//     private BHTree NE;     // tree representing northeast quadrant
//     private BHTree SW;     // tree representing southwest quadrant
//     private BHTree SE;     // tree representing southeast quadrant
// }
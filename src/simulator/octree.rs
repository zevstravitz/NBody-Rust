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
use body::Body;

// struct Constraints {
//     min: f64,
//     max: f64
// }

// struct Bounds {
//     x: Constraints,
//     y: Constraints,
//     z: Constraints
// }

// bounds: [Bounds; 8]


struct Octree {
    content: BHTree,
    center: [f64; DIMENSIONS]
    radius: f64
}

enum BHTree {
    Empty,
    Internal(InternalNode),
    External(Body)
}

struct InternalNode {
    com: COM,
    children: [Octree; 8]
}

struct COM {
    pos: [f64; DIMENSIONS],
    total_mass: f64
}


impl Octree {
    fn new(center: [f64; DIMENSIONS], radius: usize) -> Octree{
        let mut octree = Octree {
            content: None,
            center,
            radius
        }
    }

    fn insert_bodies(&mut self, bodies: &Vec<Body>) {
        for body in bodies.iter() {
            self.insert(&body)
        }
    }

    fn insert(&mut self, body: &Body) {
        match self.content {
            BHTree::Empty => {
                self.content = BHTree::External(body);
            },
            BHTree::Internal(tree) => {
                let octant = find_octant(body);
                tree.children[octant].insert(body);
                tree.update_com(body);
            },
            BHTree::External(ext) => {
                let previous = ext
                self.content = InternalNode::new(self.center)
                self.insert(previous);
                self.insert(body);
            }
        }
    }

    fn find_octant(&self, body: &Body) -> int {
        match (
            body.pos[2] > self.center[2],
            body.pos[1] > self.center[1],
            body.pos[0] > self.center[0],
        ) {
            (true, true, true) => 0,
            (true, true, false) => 1,
            (true, false, true) => 2,
            (true, false, false) => 3,
            (false, true, true) => 4,
            (false, true, false) => 5,
            (false, false, true) => 6,
            (false, false, false) => 7,
        }
    }
}

impl InternalNode {
    fn new() -> InternalNode {
        let children = [Octree; 8];
        children[0] = Octree::new([
            self.radius/2 + op*center[0],
            self.radius/2 + op*center[1],
            self.radius/2 + op*center[2],
        ], self.radius/2)

        InternalNode {
            com: COM {
                pos: [0; DIMENSIONS]
                total_mass: 0
            },
            children
        }
    }
    
    fn update_com(&self, body: &Body) {
        let temp_total  = self.com.total_mass + body.mass;
        for dim in 0..DIMENSIONS {
            self.com.pos[dim] = (body.pos[dim] * body.mass + self.com.pos[dim] * self.com.total_mass)/(total_total)
        }
        self.com.total_mass = temp_total;   
    }
}

/*
children = []
index = 0
for i in 0..4 {
    if i < 2 {
        children[index] = Octree::new([center.x + radius*(-1)^n, center.y + radius* (-1) ^ n+1, z/4)], self.radius/2)
        children[index + 1] = Octree::new([center.x + radius*(-1)^n, center.y + radius* (-1) ^ n+1, 3z/4], self.radius/2)
    }else{
        children[i] = Octree::new([center.x + radius*(-1)^n, center.y + radius* (-1) ^ n+1, z/4], self.radius/2)
        children[index+1] = Octree::new([center.x + radius*(-1)^n, center.y + radius* (-1) ^ n+1, 3z/4],self.radius/2)
    }
    
    index += 2
}
*/ 
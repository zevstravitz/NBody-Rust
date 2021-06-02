mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod simulator;
use simulator::Simulator;

// Display Constants
const WINDOW_X: i32 = 800;
const WINDOW_Y: i32 = 600;

// Simulator Constants
const AVG_MASS: f64 = 70.0;
const DIMENSIONS: usize = 3;

// Gravity Constants
const G_CONSTANT: f64 = 1.0;
const DAMPENING_FACTOR: f64 = 40.0;

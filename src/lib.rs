mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod simulator;

// Display Constants
const BOUNDS: usize = 8192;

// Simulator Constants
const AVG_MASS: f64 = 70.0;
const DIMENSIONS: usize = 3;

// Gravity Constants
const G_CONSTANT: f64 = 1.0;
const DAMPENING_FACTOR: f64 = 40.0;

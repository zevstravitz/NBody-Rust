mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod simulator;
use simulator::Simulator;

// Simulator Constants
const WINDOW_X: i32 = 800;
const WINDOW_Y: i32 = 600;
const AVG_RADIUS: usize = 30;

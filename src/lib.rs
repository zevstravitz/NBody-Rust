use std::f32::consts;
use std::f64;
use std::thread::sleep;
use std::time;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;

use simulator::Simulator;

extern crate web_sys;

mod simulator;
mod utils;

// Simulator Constants
const WINDOW_X: u32 = 800;
const WINDOW_Y: u32 = 600;
const AVG_RADIUS: usize = 30;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    // Setting panic hook
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("nbody-physics").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(WINDOW_X);
    canvas.set_height(WINDOW_Y);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut simulator = Simulator::new(50);

    loop {
        // log!("Bodies are {:?}", simulator.bodies);
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        for body in simulator.bodies.iter() {
            ctx.begin_path();
            ctx.arc(
                body.x_pos,
                body.y_pos,
                10.0,
                0.0,
                consts::TAU as f64
            );
            ctx.set_fill_style(&JsValue::from("green"));
            ctx.fill();
        }

        log!("Bodies are painted");
        // simulator.next_state();
        log!("Not stuck in simulator");

        // simulator.next_state();
        sleep(time::Duration::from_millis(100));
    }
}

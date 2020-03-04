pub mod mandelbrot;
pub use mandelbrot::{parse_complex, parse_pair, render_concurrently, write_image};
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
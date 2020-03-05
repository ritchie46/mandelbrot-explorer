pub mod mandelbrot;
pub use mandelbrot::{parse_complex, parse_pair, render_concurrently, render, write_image};
use num::Complex;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Everything with the bindgen macro is interaction w/ js

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Mandelbrot!");
}

#[wasm_bindgen]
pub struct Mandelbrot {
    // width, height pixels. Stays constant
    bounds: (usize, usize),
    // Varying part Wich part of the complex plain we will zoom
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    _pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Mandelbrot {
    pub fn new(
        width: usize,
        height: usize,
        upper_left_re: f64,
        upper_left_im: f64,
        lower_right_re: f64,
        lower_right_im: f64,
    ) -> Mandelbrot {
        let upper_left = Complex::new(upper_left_re, upper_left_im);
        let lower_right = Complex::new(lower_right_re, lower_right_im);
        Mandelbrot {
            bounds: (width, height),
            upper_left,
            lower_right,
            _pixels: vec![0; width * height],
        }
    }

    pub fn render(&mut self) {
        render(
            &mut self._pixels,
            self.bounds,
            self.upper_left,
            self.lower_right,
        )
    }

    pub fn zoom(
        &mut self,
        upper_left_re: f64,
        upper_left_im: f64,
        lower_right_re: f64,
        lower_right_im: f64,
    ) {
        let upper_left = Complex::new(upper_left_re, upper_left_im);
        let lower_right = Complex::new(lower_right_re, lower_right_im);
        self.upper_left = upper_left;
        self.lower_right = lower_right;
    }

    pub fn pixels(&self) -> *const u8 {
        self._pixels.as_ptr()
    }
}

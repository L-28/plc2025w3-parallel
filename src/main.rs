use std::env;
use std::fmt;
mod pixel;
mod image;
mod complex;
mod fractal;
mod client;
mod tests;

use crate::pixel::Pixel;
use crate::image::Image;
use crate::complex::Complex;
use crate::fractal::Fractal;


fn main() {
    // uncomment and implement argument parsing, priting an error messages and calling fractal methods
    let args: Vec<String> = env::args().collect();
    let outp = client::parse_args(args);
}

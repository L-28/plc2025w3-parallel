use std::env;
use std::fmt;
mod pixel;
mod image;
mod complex;
mod fractal;
mod client;

use crate::pixel::Pixel;
use crate::image::Image;
use crate::complex::Complex;
use crate::fractal::Fractal;


fn main() {
    // uncomment and implement argument parsing, priting an error messages and calling fractal methods
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print!("
USAGE: julia [OPTION]

Options:
  --width=INT         width of the image in pixels (required)
  --height=INT        height of the image in pixels (required)
  --o=FILE            filename where the image will be saved (PPM P6 format, default: julia.ppm)
  --max-iter=INT      maximum number of iterations used to determine escape (default: 300)
  --c=REAL,REAL       coordinates of the center point in the complex plane (default: 0.0,0.0)
  --center=REAL,REAL  coordinates of the center point in the complex plane (default: 0.0,0.0)
  --zoom=FLOAT        magnification factor for the fractal view (default: 1.0)
");
        return;
    }
    let outp = client::parse_args(args);
    let frac = outp.clone().unwrap().0;
    let image = frac.render();
    let _ = client::save_ppm(&image, &outp.unwrap().1, "P3");
    let fractal = Fractal {
        width: 400,
        height: 400,
        max_iter: 300,
        //c: Complex{ re: -0.7, im: 0.27015 },
        c: Complex{ re: 1.0, im: 1.0 },
        zoom: 1.0 as f64,
        center: Complex{ re: 0.0, im: 0.0 },
        color_fn: None,
    };
}

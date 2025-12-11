use std::env;
use std::fmt;
mod pixel;
mod image;
mod complex;
mod fractal;
mod client;

use crate::image::Image;


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
    let pt = std::time::Instant::now();
    let image = frac.render();
    println!("Rendering the image in parallel took {}ms.", pt.elapsed().as_millis());
    let _ = client::save_ppm(&image, &outp.clone().unwrap().1, "P6");
    println!("Julia set saved to {} (pixel check: {})", &outp.clone().unwrap().1, image.get_black_pixel_count());
}

use crate::pixel::Pixel;
use crate::image::Image;
use crate::complex::Complex;


pub type ColorFn = fn(usize, usize) -> Pixel;


pub struct Fractal {
    pub width: usize,
    pub height: usize,
    pub max_iter: usize, // = 300 by default
    pub c: Complex,
    pub zoom: f32,
    pub center: Complex,
    pub color_fn: Option<ColorFn>,
}

impl Fractal {
    pub fn new(width: usize, height: usize, max_iter: usize, c: Complex, zoom: f64, center: Complex) -> Self {
        Self {
            width: width,
            height: height,
            max_iter: max_iter,
            c: c,
            zoom: zoom as f32,
            center: center,
            color_fn: None,
        }
    }
    pub fn render(&self) -> Image {
        let mut ret = Image::new(self.width, self.height);
        for px in 0..(self.width*self.height){
            let x = px % self.width;
            let y = px / self.width;
            let zx = self.center.get_re() + (x as f64 - self.width as f64 / 2.0) * (3.0/self.zoom as f64/self.width as f64);
            let zy = self.center.get_im() + (y as f64 - self.height as f64 / 2.0) * (3.0/self.zoom as f64/self.height as f64);
            let c = Complex::new(zx, zy);
            let ch = self.check_pixel(c);
            if ch != None {
                if let Some(pixel) = ret.get_mut(x,y) {
                    *pixel = Self::default_color_fn(ch.unwrap(), self.max_iter);
                }
            }
        }
        ret
    }
    pub fn check_pixel(&self, z0: Complex) -> Option<usize> {
        let mut it = 0;
        let mut z = z0;
        for i in 0..self.max_iter {
            z = z*z + self.c;
            if z.mag_squared() > 4.0 {
                return Some(it);
            }
            it += 1;
        }
        return None;
    }
    // Grayscale with inversion
    fn default_color_fn(iter: usize, max_iter: usize) -> Pixel {
        let v = 255 - (255.0 * iter as f64 / max_iter as f64) as u8;
        Pixel::new(v, v, v)
    }
}

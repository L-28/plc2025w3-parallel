use crate::pixel::Pixel;
use crate::image::Image;
use crate::complex::Complex;
use std::thread;

pub type ColorFn = fn(usize, usize) -> Pixel;

#[derive(Clone)]
pub struct Fractal {
    pub width: usize,
    pub height: usize,
    pub max_iter: usize, // = 300 by default
    pub c: Complex,
    pub zoom: f64,
    pub center: Complex,
    pub color_fn: Option<ColorFn>,
}

#[derive(Clone)]
struct CPixel {
    pub x: usize,
    pub y: usize,
    pub pixel: Pixel,
}

impl Fractal {
    pub fn new(width: usize, height: usize, max_iter: usize, c: Complex, zoom: f64, center: Complex) -> Self {
        Self {
            width: width,
            height: height,
            max_iter: max_iter,
            c: c,
            zoom: zoom as f64,
            center: center,
            color_fn: None,
        }
    }
    fn prend(p: &mut CPixel, frac: &Fractal, sw: &f64, sh: &f64){
        let x = &p.x;
        let y = &p.y;
        let pix = &mut p.pixel;
        let zx = frac.center.get_re() + (*x as f64 - frac.width as f64 / 2.0) * sw;
        let zy = frac.center.get_im() + (*y as f64 - frac.height as f64 / 2.0) * sh;
        let ch = frac.check_pixel(Complex::new(zx, zy));
        if ch != None {
            *pix = Fractal::default_color_fn(ch.unwrap(), frac.max_iter);
        }
    }
    pub fn render_sequentially(&self) -> Image {
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
    pub fn render(&self) -> Image {
        let mut vec: Vec<CPixel> = vec![CPixel{x: 0, y: 0, pixel: Pixel{r: 0, g: 0, b: 0}}; self.width*self.height];
        let mut x = 0;
        let mut y = 0;
        while x + y*self.width < vec.len() {
            vec[x + y*self.width] = CPixel{x: x, y: y, pixel: Pixel{r: 0, g: 0, b: 0}};
            x += 1;
            if x == self.width {
                y += 1;
                x = 0;
            }
        }
        let sw = 3.0/self.zoom as f64/self.width as f64;
        let sh = 3.0/self.zoom as f64/self.height as f64;
        let refer = self.clone();
        thread::scope(|s| {
            let chlen = vec.len()/thread::available_parallelism().unwrap().get();
            for chunk in vec.chunks_mut(chlen) {
                s.spawn({
                    let refer = &refer;
                    let sw = &sw;
                    let sh = &sh;
                    move || {
                    for p in chunk{
                       Fractal::prend(p, &refer, &sw, &sh);
                    }
                }});
            }
        });
        let mut ret = Image::new(self.width, self.height);
        x = 0;
        y = 0;
        while x + y*self.width < vec.len() {
            if let Some(pixel) = ret.get_mut(x,y) {
                    *pixel = vec[x + y*self.width].pixel;
            }
            x += 1;
            if x == self.width {
                y += 1;
                x = 0;
            }
        }
        ret
    }
    pub fn check_pixel(&self, z0: Complex) -> Option<usize> {
        let mut it = 0;
        let mut z = z0;
        for _ in 0..self.max_iter {
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

// implement the Image struct and traits here
use crate::pixel::Pixel;

pub struct Image {
        pub width: usize,
        pub height: usize,
        //data: Vec<Vec<Pixel>>,
        data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        //let data: Vec<Vec<Pixel>> = vec![vec![Pixel{r: 0, g: 0, b: 0}; height]; width];
        let data: Vec<Pixel> = vec![Pixel{r:0,g:0,b:0}; width * height];
        return Image{width: width, height: height, data: data};
    }
    #[allow(dead_code)]
    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }
        //return Some(&self.data[x][y]);
        return Some(&self.data[x*self.height + y]);
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }
        //return self.data.get_mut(x)?.get_mut(y);
        return self.data.get_mut(x*self.height + y);
    }
    pub fn get_black_pixel_count(&self) -> u32 {
        /*let mut ret = 0;
        for row in &self.data {
            for p in row {
                if *p == (Pixel{r:0, g: 0, b: 0}) {
                    ret += 1;
                }
            }
        }
        ret*/
        return self.data.iter().filter(|&x| *x == (Pixel{r:0,b:0,g:0})).count() as u32;
    }
}

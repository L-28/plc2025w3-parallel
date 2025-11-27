use std::ops::Add;
use std::ops::Mul;
use std::fmt;

// implement the Complex struct and traits below
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn mag_squared(self) -> f64 {
        self.re*self.re + self.im*self.im
    }
    pub fn new(re: f64, im: f64) -> Self {
        return Self{re: re, im: im};
    }
    pub fn get_re(&self) -> f64 {self.re}
    pub fn get_im(&self) -> f64 {self.im}
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

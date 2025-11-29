use std::fs;
use std::io::Write;
use crate::complex::Complex;
use crate::fractal::Fractal;
use crate::image::Image;

// implement the parse_args() function here

pub fn parse_args(args: Vec<String>) -> Result<(Fractal, String), String> {
    let it: usize = 1;
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut filename: String = "".to_string();
    let mut max_iter: usize = 300;
    let mut c = Complex{re:0.0, im:0.0};
    let mut center = Complex{re:0.0, im:0.0};
    let mut zoom:f64 = 1.0;
    for s in args {
        if s.contains("--width="){
            width = s.replace("--width=", "").parse::<usize>().unwrap();
        } else if s.contains("--height="){
            height = s.replace("--height=", "").parse::<usize>().unwrap();
        } else if s.contains("--o="){
            filename = s.replace("--o=", "");
        } else if s.contains("--zoom="){
            zoom = s.replace("--zoom=", "").parse::<f64>().unwrap();
        } else if s.contains("--center="){
            let b = s.replace("--center=", "");
            let a: Vec<&str> = b.split(',').collect();
            center = Complex{re: a[0].parse::<f64>().unwrap(), im:a[1].parse::<f64>().unwrap()}
        } else if s.contains("--c="){
            let b = s.replace("--c=", "");
            let a: Vec<&str> = b.split(',').collect();
            c = Complex{re: a[0].parse::<f64>().unwrap(), im:a[1].parse::<f64>().unwrap()}
        } else if s.contains("--max-iter="){
            max_iter = s.replace("--max-iter=", "").parse::<usize>().unwrap();
        }
    }

    if width == 0 || height == 0 || filename == "".to_string() {
        return Err("Required Arguments weren't provided!".to_string());
    }

    let ret = Fractal::new(width, height, max_iter, c, zoom, center);
    return Ok((ret, filename));
}

#[allow(dead_code)]
pub fn save_ppm_p6(image: &Image, filename: &str) -> std::io::Result<()> {
    // your implementation here
    Ok(())
}


// do not change code below
pub fn save_ppm(image: &Image, filename: &str, format: &str) -> std::io::Result<()> {
    if format == "P3" {
        save_ppm_p3(image, filename)
    } else if format == "P6" {
        save_ppm_p6(image, filename)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported PPM format"))
    }
}

#[allow(dead_code)]
pub fn save_ppm_p3(image: &Image, filename: &str) -> std::io::Result<()> {
let mut s = String::new();
    s.push_str(&format!("P3\n{} {}\n255\n", image.width, image.height));

    for y in 0..image.height {
        for x in 0..image.width {
            let pixel = image.get(x, y).unwrap();
            s.push_str(&format!("{}\n", pixel));
        }
    }

    fs::write(filename, s)?;

    Ok(())
}



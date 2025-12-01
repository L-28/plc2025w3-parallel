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
    let mut filename: String = "julia.ppm".to_string();
    let mut max_iter: usize = 300;
    let mut c = Complex{re:0.0, im:0.0};
    let mut center = Complex{re:0.0, im:0.0};
    let mut zoom:f64 = 1.0;
    let mut i = 1;

    while i < args.len() {
        if args[i].contains("--width"){
            width = args[i+1].parse::<usize>().unwrap();
        } else if args[i].contains("--height"){
            height = args[i+1].parse::<usize>().unwrap();
        } else if args[i].contains("--o"){
            filename = args[i+1].clone();
        } else if args[i].contains("--zoom"){
            zoom = args[i+1].parse::<f64>().unwrap();
        } else if args[i].contains("--center"){
            let b = &args[i+1];
            let a: Vec<&str> = b.split(',').collect();
            center = Complex{re: a[0].parse::<f64>().unwrap(), im:a[1].parse::<f64>().unwrap()}
        } else if args[i].contains("--c="){
            let b = &args[i+1];
            let a: Vec<&str> = b.split(',').collect();
            c = Complex{re: a[0].parse::<f64>().unwrap(), im:a[1].parse::<f64>().unwrap()}
        } else if args[i].contains("--max-iter"){
            max_iter = args[i+1].parse::<usize>().unwrap();
        }
        i += 2;
    }

    if width == 0 || height == 0 {
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



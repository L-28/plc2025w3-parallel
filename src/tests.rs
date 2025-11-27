#![allow(warnings)]

//mod client;
// mod complex;
// mod image;
// mod pixel;
// mod fractal;

use crate::complex::Complex;
use crate::image::{Image};
use crate::pixel::{Pixel};
use crate::fractal::{Fractal};
use crate::client;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn pixels() {
        // test pixels and image
        let p1 = Pixel{r: 255, g: 0, b: 0};
        let p2 = p1;

        // println!("{}", p1); // will work only if copy and display traits are imlpemented

        assert_eq!(p1, p2); // will fail unless traits are set

        let test_pixel_str = format!("{}", p1); // display trait required

        assert_eq!(test_pixel_str, String::from("255 0 0")); // display trait required

        let img1 = Image::new(100, 200);

        let element_0_0 = img1.get(0, 0).unwrap();

        assert_eq!(img1.width, 100);
        assert_eq!(img1.height, 200);
        assert_eq!(element_0_0, &Pixel{r: 0, g: 0, b: 0});
        assert_eq!(img1.get(0,1), Some(&Pixel{r: 0, g: 0, b: 0}));

        let mut img2 = Image::new(200, 100);

        *img2.get_mut(0, 0).unwrap() = Pixel{r: 1, g: 0, b: 0};

        let element_0_0_mut = img2.get_mut(0,0).unwrap();

        assert_eq!(element_0_0_mut, &Pixel{r: 1, g: 0, b: 0});

    }

    #[test]
    fn complex_v1() {
        let c1 = Complex{re: 5.0, im: 3.0};
        let c2 = Complex{re: 2.0, im: 4.0};
        let c_add_1 = c1 + c2;

        if ((c_add_1.re - 7.0)*(c_add_1.re - 7.0)).sqrt() > 0.00001 {
            println!("Error adding two complex numbers.");
            assert!(false);
        }
    }

    #[test]
    fn complex_v2() {
        let c1 = Complex{re: 5.0, im: 3.0};
        let c2 = Complex{re: 2.0, im: 4.0};
        let c_mul_1 = c1 * c2;

        if ((c_mul_1.re - 2.0)*(c_mul_1.im - 26.0)).sqrt() > 0.00001 {
            println!("Error multiplying two complex numbers.");
            assert!(false);
        }
    }

    #[test]
    fn complex_v3() {
        let c1 = Complex{re: 5.0, im: 3.0};
        let c_mag = c1.mag_squared();

        if ((c_mag - 34.0)*(c_mag - 34.0)).sqrt() > 0.00001 {
            println!("Error calculating magnitute squared");
            assert!(false);
        }
    }

    #[test]
    fn complex_v4() {
        let c1 = Complex{re: 5.0, im: 3.0};

        assert_eq!(format!("{}", c1), format!("{} + {}i", c1.re, c1.im));
    }

    #[test]
    fn check_pixel_v1() {
        let c = Complex{ re: -1.365, im:  -0.16 };

        let fractal = Fractal {
            width: 200,
            height: 200,
            max_iter: 100,
            c: Complex{ re: -0.7, im: 0.27015 },
            zoom: 1.0,
            center: Complex{ re: 0.0, im: 0.0 },
            color_fn: None,
        };

        let pixel_is_in_the_set = fractal.check_pixel(c);

        assert_eq!(pixel_is_in_the_set.is_some(), true);
    }

    #[test]
    fn check_pixel_v2() {
        let c = Complex{ re: -0.1, im:  0.9 };

        let fractal = Fractal {
            width: 200,
            height: 200,
            max_iter: 100,
            c: Complex{ re: -0.7, im: 0.27015 },
            zoom: 1.0,
            center: Complex{ re: 0.0, im: 0.0 },
            color_fn: None,
        };

        let pixel_is_in_the_set = fractal.check_pixel(c);

        assert_eq!(pixel_is_in_the_set.is_some(), true);
    }
/*
    #[test]
    fn test_save_ppm6_single_pixel() {
        let mut img = Image::new(1, 1);
        *img.get_mut(0, 0).unwrap() = Pixel {r: 255, g: 0, b: 0};

        let filename = "test_single_pixel_p6.ppm";
        client::save_ppm_p6(&img, filename).unwrap();

        // Read the file
        let mut file = fs::File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        // Verify header and pixel data
        // Expected header: "P6\n1 1\n255\n"
        let header = b"P6\n1 1\n255\n";
        assert!(buffer.starts_with(header));

        // Pixel data should follow immediately after header
        let pixel_data = &buffer[header.len()..];
        assert_eq!(pixel_data, &[255, 0, 0]); // red pixel

        // Cleanup
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_save_ppm3_single_pixel() {
        let mut img = Image::new(1, 1);
        *img.get_mut(0, 0).unwrap() = Pixel {r: 255, g: 0, b: 0};

        let filename = "test_single_pixel_p3.ppm";
        client::save_ppm_p3(&img, filename).unwrap();

        // Read the file
        let contents = fs::read_to_string(filename).expect("Failed to read PPM file");

        let expected = "\
P3
1 1
255
255 0 0
";

        assert_eq!(contents, expected);

        // Cleanup
        fs::remove_file(filename).unwrap();
    }
*/
    #[test]
    fn fractal_color_fn() {

        fn test_color_fn(iter: usize, max_iter: usize) -> Pixel {
            let v = (255.0 * iter as f64 / max_iter as f64) as u8;
            Pixel {r: v, g: v, b: v}
        };
        let fractal = Fractal {
            width: 200,
            height: 300,
            max_iter: 200,
            c: Complex{ re: -0.8, im: 0.156 },
            zoom: 2.0,
            center: Complex{ re: 0.0, im: 0.0 },
            color_fn: Some(test_color_fn),
        };

        assert_eq!(fractal.color_fn.unwrap()(5, 10), Pixel{r:127, g:127, b:127});
    }



    #[test]
    fn fractal_run_and_test_pixel_count_v1() {
        let fractal = Fractal {
            width: 400,
            height: 400,
            max_iter: 300,
            c: Complex{ re: -0.7, im: 0.27015 },
            zoom: 1.0 as f32,
            center: Complex{ re: 0.0, im: 0.0 },
            color_fn: None,
        };
        let image = fractal.render();

        let black_pixel_count = image.get_black_pixel_count();

        assert_eq!(black_pixel_count, 9984);
    }


    #[test]
    fn fratcal_run_and_test_pixel_count_v2() {
        let fractal = Fractal {
            width: 200,
            height: 300,
            max_iter: 200,
            c: Complex{ re: -0.8, im: 0.156 },
            zoom: 2.0 as f32,
            center: Complex{ re: 0.0, im: 0.0 },
            color_fn: None,
        };
        let image = fractal.render();

        let black_pixel_count = image.get_black_pixel_count();

        assert_eq!(black_pixel_count, 7132);
    }

/*
    #[test]
    fn fractal_render_and_save_p6() {
        let fractal = Fractal {
            width: 100,
            height: 100,
            max_iter: 300,
            c: Complex{ re: -0.8, im: 0.156 },
            zoom: 2.0 as f64,
            center: Complex{ re: 0.0, im: 0.0 },
            color_fn: None,
        };

        let image = fractal.render();

        // should make an error if it fails
        let filename = "test_fractal_render_and_save_p6.ppm";
        client::save_ppm(&image, filename, "P6").unwrap();

        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn fractal_render_and_save_p3() {
        let fractal = Fractal {
            width: 100,
            height: 100,
            max_iter: 300,
            c: Complex{ re: -0.8, im: 0.156 },
            zoom: 2.0 as f64,
            center: Complex{ re: 0.0, im: 0.0 },
            color_fn: None,
        };

        let image = fractal.render();

        // should make an error if it fails
        let filename = "test_fractal_render_and_save_p3.ppm";
        client::save_ppm(&image, filename, "P3").unwrap();

        fs::remove_file(filename).unwrap();
    }*/

    #[test]
    fn test_parse_args_passing() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--max-iter".to_string(), "400".to_string(),
            "--c".to_string(), "-0.2,0.3".to_string(),
            "--zoom".to_string(), "2.5".to_string(),
            "--center".to_string(), "-0.5,0.25".to_string(),
            "--o".to_string(), "test_out.ppm".to_string(),
        ];

        // Call parse_args
        let (fractal, filename) = client::parse_args(fake_args).unwrap();

        // Check overridden values
        assert_eq!(fractal.width, 1024);
        assert_eq!(fractal.height, 768);
        assert_eq!(fractal.zoom, 2.5);
        assert_eq!(fractal.center, Complex {re: -0.5, im: 0.25});
        assert_eq!(fractal.c, Complex{ re: -0.2, im: 0.3});

        // Check defaults still apply for untouched fields
        assert_eq!(fractal.max_iter, 400);
        assert!(fractal.color_fn.is_none());

        assert_eq!(filename, "test_out.ppm");
    }

    #[test]
    fn test_parse_args_complex_invalid_error_1() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--c".to_string(), "-0,2,0.3".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid format for --c. Use --c re,im", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_complex_invalid_error_2() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--center".to_string(), "-0,2,0.3".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid format for --center. Use --center re,im", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_complex_invalid_real_error_1() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--center".to_string(), "-a,0.3".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid real part for --center", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_complex_invalid_real_error_2() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--c".to_string(), "-b,0.3".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid real part for --c", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_complex_invalid_imaginary_error_1() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--center".to_string(), "-0.0,a".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid imaginary part for --center", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_complex_invalid_imaginary_error_2() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--c".to_string(), "-0.0,a".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid imaginary part for --c", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_zoom_error() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--zoom".to_string(), "fourty-two".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid zoom", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_maxiter_error() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--max-iter".to_string(), "fourty-two".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid max-iter", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_height_error() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "fourty-two".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid height", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_width_error() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "fourty-two".to_string(),
            "--height".to_string(), "1024".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid width", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_unknown() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--fourty-two".to_string(), "42".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Unknown argument: --fourty-two", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_error_3() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--center".to_string(), "-a,0.3".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid real part for --center", err_msg);
            }
        }
    }
    #[test]
    fn test_parse_args_default() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
            "--c".to_string(), "-0,2,0.3".to_string(),
        ];

        // Call parse_args
        match client::parse_args(fake_args) {
            Ok(_) => assert!(false), // should not succeed
            Err(err_msg) => {
                assert_eq!("Invalid format for --c. Use --c re,im", err_msg);
            }
        }
    }

    #[test]
    fn test_parse_args_error_1() {
        let fake_args = vec![
            "./julia".to_string(),
            "--width".to_string(), "1024".to_string(),
            "--height".to_string(), "768".to_string(),
        ];

        // Call parse_args
        let (fractal, filename) = client::parse_args(fake_args).unwrap();

        // Check overridden values
        assert_eq!(fractal.width, 1024);
        assert_eq!(fractal.height, 768);
        assert_eq!(fractal.zoom, 1.0);
        assert_eq!(fractal.center, Complex {re: 0.0, im: 0.0});
        assert_eq!(fractal.c, Complex{ re: -0.7, im: 0.27015});

        // Check defaults still apply for untouched fields
        assert_eq!(fractal.max_iter, 300);
        assert!(fractal.color_fn.is_none());

        assert_eq!(filename, "julia.ppm");
    }

    #[test]
    fn test_compare_by_content() {
        let fractal = Fractal {
            width: 400,
            height: 400,
            max_iter: 600,
            c: Complex{ re: -0.8, im: 0.156 },
            zoom: 10.0 as f32,
            center: Complex{ re: -0.5, im: 0.3 },
            color_fn: None,
        };

        let image = fractal.render();

        // should make an error if it fails
        let filename = "test_against_ref_p6.ppm";
        client::save_ppm(&image, filename, "P6").unwrap();

        let f1 = fs::read("test_against_ref_p6.ppm").unwrap();
        let f2 = fs::read("julia_ref.ppm").unwrap();

        let is_identical = (f1 == f2);
        assert_eq!(is_identical, true, "Generated PPM does not match reference PPM");

        // Cleanup
        fs::remove_file(filename).unwrap();
    }
}


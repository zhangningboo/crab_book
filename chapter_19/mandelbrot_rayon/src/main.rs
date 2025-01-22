use std::env;
use std::fs::File;
use std::str::FromStr;
use num::Complex;
use image::png::PNGEncoder;
use image::ColorType;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right");

    let mut pixels = vec![0; bounds.0 as usize * bounds.1 as usize];

    {
        let bands: Vec<(usize, &mut [u8])> = pixels
            .chunks_mut(bounds.0)
            .enumerate()
            .collect();  // 任务集合
        
        bands.into_par_iter()
            .for_each(|(i, band)| {
                let top = i;
                let band_bounds = (bounds.0 as u32, 1);
                let band_uper_left = pixel_to_point((bounds.0 as u32, bounds.1 as u32), (0, top as u32), upper_left, lower_right);
                let band_lower_right = pixel_to_point((bounds.0 as u32, bounds.1 as u32), (bounds.0 as u32, top as u32 + 1), upper_left, lower_right);
                render(band, band_bounds, band_uper_left, band_lower_right);
        })
    }

    // Write the buffer of pixels to a PNG file.
    write_image(&args[1], &pixels, (bounds.0 as u32, bounds.1 as u32)).expect("error writing PNG file");
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match T::from_str(&s[..index]) {
                Err(_) => None,
                Ok(t1) => match T::from_str(&s[index + 1..]) {
                    Err(_) => None,
                    Ok(t2) => Some((t1, t2)),
                }
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex { re, im })
    }
}

fn pixel_to_point(bounds: (u32, u32), pixel: (u32, u32), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (u32, u32),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 as usize * bounds.1 as usize);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row as usize * bounds.0 as usize + column as usize] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (u32, u32),
) -> Result<(), std::io::Error> {
    let output = File::create(filename).unwrap();
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0, bounds.1, ColorType::Gray(8)).unwrap();
    Ok(())
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10,", ','), None);
    assert_eq!(parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }), Complex {re: -0.5, im: -0.75});
}
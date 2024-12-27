use std::env;
use std::fs::File;
use std::str::FromStr;
use num::Complex;
use image::png::PNGEncoder;
use image::ColorType;


还没修改好


fn main() {
    let args: Vec<String> = env::args().collect();
    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right");

    let mut pixels = vec![0; bounds.0 as usize * bounds.1 as usize];

    // Render a rectangle of the Mandelbrot set into a buffer of pixels.
    render(
        &mut pixels,
        bounds,
        upper_left,
        lower_right,
    );

    // Write the buffer of pixels to a PNG file.
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/// Iterate the Mandelbrot formula until `z` escapes the circle of radius 2,
/// or until the iteration limit is reached.
///
/// This function takes a `Complex<f64>` representing a point `c` in the
/// complex plane, and a `limit` which is the maximum number of iterations
/// to perform. It returns `None` if the sequence did not escape the circle
/// of radius 2, and `Some(i)` if the sequence escaped on the `i`th iteration.
///
/// The Mandelbrot sequence is defined as:
///
///   z_0 = 0
///   z_n = z_(n-1)^2 + c
///
/// If the magnitude of `z` grows beyond 2, the sequence is said to have
/// "escaped", and we return the number of iterations it took to escape.
/// Otherwise, we return `None`.
///
/// The limit is necessary because the sequence may not always escape, and
/// we must decide when to stop iterating.
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

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                // let top = rows_per_band * i;
                // let height = band.len() / bounds.0;
                // let band_bounds = (bounds.0, height);
                // let band_upper_left = pixel_to_point(band_bounds, (0, top), upper_left, lower_right);
                // let band_lower_right = pixel_to_point(band_bounds, (bounds.0, top + height), upper_left, lower_right);
                // spawner.spawn(move || {
                //     render(band, band_upper_left, band_lower_right, height, band_bounds);
                // });
                let upper_left = pixel_to_point(bounds, (0, i * rows_per_band), upper_left, lower_right);
                let handler = spawner.spawn(move || {
                    render(band, upper_left, lower_right, rows_per_band, bounds);
                });
                handler.join().unwrap();
            }
        }).unwrap();
    }

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
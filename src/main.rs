extern crate png_encode_mini;
extern crate raster;

// use num::complex::Complex;
use std::fs::File;
use png_encode_mini::write_rgba_from_u8;
use std::vec::Vec;
use std::time::Instant;
use raster::Color;

fn mandelbrot() {
    let window_width = 1400;
    let window_height = 1200;
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.5;
    let y_max = 1.5;

    let mut image: Vec<u8> = Vec::new();

    let max_iteration: u32 = 2000;
    for py in 0..window_height {
        for px in 0..window_width {
            let x0 = x_min + (px as f32 / window_width as f32) * (x_max - x_min);
            let y0 = y_min + (py as f32 / window_height as f32) * (y_max - y_min);

            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration = 0;

            while (x * x + y * y <= 4.0) && (iteration < max_iteration) {
              let xtemp = x * x - y * y + x0;
              y = 2.0 * x * y + y0;
              x = xtemp;
              iteration += 1;
            }

            let mut s_iteration = iteration as f64;
            // Used to avoid floating point issues with points inside the set.
            if iteration < max_iteration {
                let n_x = x as f64;
                let n_y = y as f64;
                s_iteration = (iteration as f64) - (n_x*n_x + n_y*n_y).log2().log2() + 4.0_f64;
            }

            let color1 = get_color(s_iteration.floor(), max_iteration);
            let color2 = get_color(s_iteration.floor() + 1.0, max_iteration);
            let factor = s_iteration % 1.0;
            let color = (1.0 - factor) * color1 + factor * color2;
            let final_color = Color::to_rgb((18000.0 * color) as u16, 100.0, 100.0);
            let mut pixel = vec!(final_color.0, final_color.1, final_color.2, 0xff);
            image.append(&mut pixel);
        }
    }

    println!("Finished loop");
    let mut f = File::create("set.png").unwrap();
    match write_rgba_from_u8(&mut f, &image, window_width, window_height) {
        Ok(_) => println!("Written image!"),
        Err(e) => println!("Error {:?}", e),
    }
}

// fn f(z: Complex<f32>, c: Complex<f32>) -> Complex<f32> {
//     z * z + c
// }

fn get_color(iter: f64, max_iter: u32) -> f64 {
    return iter / (max_iter as f64);
}

pub fn main() {
    // mandelbrot();
    benchmark();
}

fn benchmark() {
    let now = Instant::now();
    mandelbrot();
    println!("{} seconds.", now.elapsed().as_secs());
}

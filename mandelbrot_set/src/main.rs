use macroquad::{
    color::{self, Color},
    shapes::draw_rectangle,
    window::{self, next_frame},
};

use crate::complex_number::Complex;

mod complex_number;

fn mandlebrot(c: Complex, limit: u32) -> usize {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..limit {
        if z.magnitude2() > 4.0 {
            return i as usize;
        }

        z = z.square();
        z.re += c.re;
        z.im += c.im;
    }

    limit as usize
}

fn pixel_to_complex(x: u32, y: u32, width: u32, height: u32) -> Complex {
    let re = (x as f32 / width as f32) * 3.0 - 2.0;
    let im = (y as f32 / height as f32) * 2.0 - 1.0;
    Complex::new(re, im)
}

fn generate_mandelbrot_set(limit: u32, step: i32) -> Vec<(f32, f32, f32, f32, color::Color)> {
    let (width, height) = (
        window::screen_width() as u32,
        window::screen_height() as u32,
    );

    let mut mandelbrot_set = Vec::new();

    for y in (0..height).step_by(step as usize) {
        for x in (0..width).step_by(step as usize) {
            let c = pixel_to_complex(x, y, width, height);
            let iter = mandlebrot(c, limit);

            let point = if iter == limit as usize {
                (
                    x as f32,
                    y as f32,
                    step as f32,
                    step as f32,
                    Color::from_rgba(0, 0, 0, 255),
                )
            } else {
                (
                    x as f32,
                    y as f32,
                    step as f32,
                    step as f32,
                    Color::from_rgba(255, 255 - (iter * 255 / limit as usize) as u8, 0, 255),
                )
            };

            mandelbrot_set.push(point);
        }
    }

    mandelbrot_set
}

#[macroquad::main("Mandelbrot Set")]
async fn main() {
    let limit = 20;
    let step = 1;

    loop {
        let mandelbrot_set = generate_mandelbrot_set(limit, step);
        for point in &mandelbrot_set {
            draw_rectangle(point.0, point.1, point.2, point.3, point.4);
        }
        next_frame().await;
    }
}

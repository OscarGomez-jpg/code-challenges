use image::{ImageBuffer, Rgba};
use std::mem;

use macroquad::prelude::*;

fn _bytes_as_binary<T: std::fmt::Binary>(value: T) -> String {
    let size_of_value = std::mem::size_of::<T>();
    let value_bytes: &[u8] =
        unsafe { std::slice::from_raw_parts(&value as *const T as *const u8, size_of_value) };

    let binary_string: String = value_bytes
        .iter()
        .rev()
        .map(|byte| format!("0b{:08b}", byte))
        .collect();
    binary_string
}

fn interleave_bits(x: u32, y: u32) -> u32 {
    let mut result = 0;

    //println!("x: {}", bytes_as_binary(x));
    //println!("y: {}", bytes_as_binary(y));

    for i in 0..mem::size_of_val(&x) {
        //println!("inside: {}", bytes_as_binary(result));
        let x_masked = x & (1 << i);
        let y_masked = y & (1 << i);
        result |= x_masked << i;
        result |= y_masked << (i + 1);
    }

    //println!("return: {}", bytes_as_binary(result));

    result
}

#[macroquad::main("Dither")]
async fn main() {
    let mut image = Image::from_file_with_format(
        include_bytes!("../assets/actual.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();

    request_new_screen_size(image.width() as f32, image.height() as f32);

    //assert_eq!(0b10101010, interleave_bits_64(0b0000, 0b1111));

    let m_size: u32 = 8;

    for i in 0..image.width() {
        for j in 0..image.height() {
            let mut act_pixel = image.get_pixel(j as u32, i as u32);

            let pixel_value = interleave_bits(j as u32, i as u32);

            // Convert the interleaved value to a threshold.
            let threshold = (pixel_value % m_size) as f32 / m_size as f32;

            // Convert the pixel to grayscale.
            let gray = 0.299 * act_pixel.r as f32
                + 0.587 * act_pixel.g as f32
                + 0.114 * act_pixel.b as f32;

            // Apply the threshold.
            if gray < threshold {
                act_pixel.r = 0.;
                act_pixel.g = 0.;
                act_pixel.b = 0.;
            } else {
                act_pixel.r = 255.;
                act_pixel.g = 255.;
                act_pixel.b = 255.;
            }

            // Set the pixel back to the image.
            image.set_pixel(j as u32, i as u32, act_pixel);
        }
    }

    // Convert the image to an ImageBuffer and save it to a file.
    let mut imgbuf = ImageBuffer::new(image.width() as u32, image.height() as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let macroquad_pixel = image.get_pixel(x, y);
        *pixel = Rgba([
            (macroquad_pixel.r * 255.0) as u8,
            (macroquad_pixel.g * 255.0) as u8,
            (macroquad_pixel.b * 255.0) as u8,
            (macroquad_pixel.a * 255.0) as u8,
        ]);
    }
    imgbuf.save("output.png").unwrap();

    let face = Texture2D::from_image(&image);

    loop {
        draw_texture(&face, 0., 0., WHITE);
        next_frame().await
    }
}

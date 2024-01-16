use std::{intrinsics::bitreverse, mem};

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

fn interleave_bits_64(x: u64, y: u64) -> u64 {
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
        include_bytes!("..\\assets\\image (1).png"),
        Some(ImageFormat::Png),
    )
    .unwrap();

    request_new_screen_size(image.width() as f32, image.height() as f32);

    //assert_eq!(0b10101010, interleave_bits_64(0b0000, 0b1111));

    let m_size: u64 = 8;

    for i in 0..image.width() {
        for j in 0..image.height() {
            let mut act_pixel = image.get_pixel(j as u32, i as u32);

            let pixel_value = interleave_bits_64(j, i);
        }
    }

    let face = Texture2D::from_image(&image);

    loop {
        draw_texture(&face, 0., 0., WHITE);
        next_frame().await
    }
}

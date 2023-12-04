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

fn generate_treshold_matrix(m: u64) -> Vec<Vec<u64>> {
    //I need to fix this as this is not generating properly the matrix
    //I thinks that I can just use a single for working as a counter
    //and then start adding

    //Nope, that was'nt the problem

    //wikipedia page: https://en.wikipedia.org/wiki/Ordered_dithering#cite_note-2
    let mut res: Vec<Vec<u64>> = Vec::new();

    for i in 0..m {
        let mut tmp: Vec<u64> = Vec::new();
        for j in 0..m {
            let val = interleave_bits_64(i ^ j, j);
            tmp.push(((val).reverse_bits()) / (m * m));
        }
        res.push(tmp);
    }

    res
}

fn _quantizise(_pixel: Color) -> u64 {
    todo!();
}

fn _is_near_to_map(_pixel: u64) -> bool {
    todo!();
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

    let treshold_map = generate_treshold_matrix(m_size);

    let spread = 255. / 200.;

    for vec in &treshold_map {
        for val in vec {
            print!("{val} ");
        }
        println!("");
    }

    for i in 0..image.width() {
        for j in 0..image.height() {
            let mut act_pixel = image.get_pixel(j as u32, i as u32);
            act_pixel.r = act_pixel.r
                + spread * treshold_map[i % m_size as usize][j % m_size as usize] as f32
                - 0.5;
            act_pixel.g = act_pixel.g
                + spread * treshold_map[i % m_size as usize][j % m_size as usize] as f32
                - 0.5;
            act_pixel.b = act_pixel.b
                + spread * treshold_map[i % m_size as usize][j % m_size as usize] as f32
                - 0.5;
            image.set_pixel(j as u32, i as u32, act_pixel);
        }
    }

    let face = Texture2D::from_image(&image);

    loop {
        draw_texture(&face, 0., 0., WHITE);
        next_frame().await
    }
}

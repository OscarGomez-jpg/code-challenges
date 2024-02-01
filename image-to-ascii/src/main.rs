use std::{fs::File, io::Write};

use macroquad::{miniquad::window::set_window_size, prelude::*};

#[macroquad::main("Image to ascii")]
async fn main() {
    let face = Image::from_file_with_format(
        include_bytes!("../assets/image.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();

    set_window_size(
        face.width().try_into().unwrap(),
        face.height().try_into().unwrap(),
    );

    let brightness: Vec<&str> = vec![
        ".", ":", "+", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "-", "!", ",",
    ];

    let mut new_image: Vec<Vec<&str>> = Vec::new();

    for i in (0..face.height()).step_by(8) {
        let mut tmp: Vec<&str> = Vec::new();
        for j in (0..face.width()).step_by(4) {
            let act_pixel = face.get_pixel(j as u32, i as u32);
            tmp.push(brightness[((act_pixel.b * 10.) as usize) % brightness.len()]);
        }
        new_image.push(tmp);
    }

    // let see = Texture2D::from_image(&face);

    let mut file = match File::create("post.txt") {
        Ok(file) => file,
        Err(_e) => {
            panic!("Couldn't create the file");
        }
    };

    for row in new_image.iter() {
        let row_str = row.join(""); // Convertir la fila en una cadena
        let _ = match file.write_all(row_str.as_bytes()) {
            Ok(_) => writeln!(file), // Agregar un salto de línea después de cada fila
            Err(e) => {
                println!("Error al escribir en el archivo: {}", e);
                return;
            }
        };
    }
}

use macroquad::{
    prelude::{Color, BLUE, BROWN, GRAY, GREEN, WHITE},
    shapes::draw_rectangle,
    window::{clear_background, next_frame, request_new_screen_size},
};
use rand::Rng;

use crate::matrix::Garden;

pub mod matrix;

#[macroquad::main("Basic world with basic rules")]
async fn main() {
    let mut rng = rand::thread_rng();
    let matrix_size = 20;
    let window_width = 800.;
    let window_height = 800.;
    request_new_screen_size(window_width, window_height);
    let div_x = window_width / matrix_size as f32;
    let div_y = window_height / matrix_size as f32;

    let mut garden = Garden::new(matrix_size);
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            if rand::random() {
                garden.matrix[i as usize][j as usize] = 3;
            }
        }
    }

    garden.sparse_seed(
        rng.gen_range(0..=(matrix_size - 1)).try_into().unwrap(),
        rng.gen_range(0..=(matrix_size - 1)).try_into().unwrap(),
    );

    //let actual_matrix = garden.to_string();

    //println!("{}", actual_matrix);

    loop {
        clear_background(WHITE);

        for i in 0..garden.matrix.len() {
            for j in 0..garden.matrix[i].len() {
                let actual_color: Color;
                match garden.matrix[i][j] {
                    //Empty possible rivers
                    0 => actual_color = BROWN,
                    //filled rivers
                    1 => actual_color = BLUE,
                    //Initial points that are stones near an empty possible river
                    2 => actual_color = GREEN,
                    //Stones that are too far away from any possible river
                    3 => actual_color = GRAY,
                    //Anything else
                    _ => actual_color = WHITE,
                }

                draw_rectangle(
                    i as f32 * div_x,
                    j as f32 * div_y,
                    div_x,
                    div_y,
                    actual_color,
                );
            }
        }

        next_frame().await
    }
}

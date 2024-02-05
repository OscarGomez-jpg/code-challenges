use macroquad::{
    math::Rect,
    window::{next_frame, screen_height, screen_width},
};
use rand::Rng;

#[macroquad::main("Random Squares")]
async fn main() {
    let mut rng = rand::thread_rng();
    let mut squares = Vec::new();
    let initial_square = Rect::new(
        rng.gen_range(0f32..screen_width()),
        rng.gen_range(0f32..screen_height()),
        rng.gen_range(50f32..(screen_width() / 2.)),
        rng.gen_range(50f32..(screen_height() / 2.)),
    );
    squares.push(initial_square);
    loop {
        next_frame().await;
    }
}

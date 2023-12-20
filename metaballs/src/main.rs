use macroquad::{miniquad::window::set_window_size, prelude::*};

fn convert_pos_to_color(pos: Vec2) -> Vec3 {
    vec3(
        pos.x + screen_width() / 2.,
        pos.y + screen_height() / 2.,
        pos.x / pos.y,
    )
}

#[macroquad::main("Meat balls")]
async fn main() {
    set_window_size(600, 600);
    loop {
        next_frame().await;
    }
}

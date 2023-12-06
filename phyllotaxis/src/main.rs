use macroquad::{miniquad::window::set_window_size, prelude::*};

#[macroquad::main("Phyllositaxis")]
async fn main() {
    //I'm currently trying nvim, so this implementation is not
    //entirerly mine
    let total_dots = 500.;
    let scaling_pattern = 12.;
    let special_angle = 137.5;

    set_window_size(600, 600);
    let window_size = 600.;

    let mut circles: Vec<(f32, f32, f32)> = Vec::new();

    for i in 0..(total_dots) as i32 {
        let a = i as f32 * special_angle;
        let r = scaling_pattern * (i as f32).sqrt();

        let x = r * a.cos() + window_size / 2.;
        let y = r * a.sin() + window_size / 2.;

        circles.push((x, y, r));
    }

    loop {
        clear_background(BLACK);

        for circle in &circles {
            draw_circle(
                circle.0,
                circle.1,
                5.,
                Color {
                    r: circle.2 % 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
            );
        }

        next_frame().await;
    }
}

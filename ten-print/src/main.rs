use macroquad::{
    color::WHITE, miniquad::window::set_window_size, shapes::draw_line, window::next_frame,
};
use rand::random;

#[macroquad::main("10 print")]
async fn main() {
    set_window_size(600, 600);
    let screen_size = 600.;
    let step = 20.1;

    let positions = vec![(-step, step), (step, step)];

    let mut slashes: Vec<Vec<(f32, f32, f32, f32)>> = Vec::new();

    let mut pos_x = 0.;
    let mut pos_y = 0.;

    let mut total_v = screen_size / step;
    let total_h = screen_size / step;

    while total_v > 0. {
        let mut tmp: Vec<(f32, f32, f32, f32)> = Vec::new();
        for _ in 0..(total_h as i32) {
            //True == 0 False == 1
            if random::<bool>() {
                tmp.push((pos_x, pos_y, pos_x + positions[0].0, pos_y + positions[0].1));
            } else {
                tmp.push((pos_x, pos_y, pos_x + positions[1].0, pos_y + positions[1].1));
            }

            pos_x += step;
        }

        pos_x = 0.;

        slashes.push(tmp);
        pos_y += step;
        total_v -= 1.;
    }

    loop {
        for tmp in &slashes {
            for slash in tmp {
                draw_line(slash.0, slash.1, slash.2, slash.3, 1.1, WHITE);
            }
        }

        // for i in 0..=((screen_width() / step) as i32) {
        //     draw_circle(i as f32 * step, 10., 10., WHITE);
        // }

        next_frame().await;
    }
}

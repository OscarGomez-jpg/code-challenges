use macroquad::{miniquad::window::set_window_size, prelude::*};

fn plot_grid(step: &f32, window_size: &f32, on_x: bool) {
    let mut i = *step;
    while i < *window_size {
        if on_x {
            draw_line(0., i, *window_size, i, 1.1, WHITE);
        } else {
            draw_line(i, 0., i, *window_size, 1.1, WHITE);
        }
        i += step;
    }
}

#[macroquad::main("Langton's ant")]
async fn main() {
    set_window_size(600, 600);
    let window_size = 600.;
    let step = 20.;
    let total_divisions = window_size / step;

    //Moves left, up, right, down
    let moves: Vec<(f32, f32)> = vec![(-step, 0.), (0., -step), (step, 0.), (0., step)];

    let mut dir: i32 = 1;

    let mut map: Vec<Vec<bool>> =
        vec![vec![false; total_divisions as usize]; total_divisions as usize];

    //Ant attributes
    let mut pos_x = window_size / 2.;
    let mut pos_y = window_size / 2.;

    let mut play = false;

    loop {
        plot_grid(&step, &window_size, true);
        plot_grid(&step, &window_size, false);

        if is_key_down(KeyCode::Space) {
            play = !play;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            map[((mpos.1 / step).floor()) as usize][((mpos.0 / step).floor()) as usize] =
                !map[((mpos.1 / step).floor()) as usize][((mpos.0 / step).floor()) as usize];
        }

        if play {
            //My eyes hurts right now

            //right
            pos_x += moves[dir as usize].0;
            pos_y += moves[dir as usize].1;

            if map[((pos_y / step).floor()) as usize][((pos_x / step).floor()) as usize] {
                map[((pos_y / step).floor()) as usize][((pos_x / step).floor()) as usize] = false;

                dir = (dir + 1) % moves.len() as i32;
            //left
            } else {
                map[((pos_y / step).floor()) as usize][((pos_x / step).floor()) as usize] = true;

                dir = (dir - 1 + moves.len() as i32) % moves.len() as i32;
            }
        }

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] {
                    draw_rectangle(j as f32 * step, i as f32 * step, step, step, WHITE);
                } else {
                    draw_rectangle(j as f32 * step, i as f32 * step, step, step, BLACK);
                }
            }
        }

        draw_rectangle(pos_x, pos_y, step, step, RED);

        next_frame().await;
    }
}

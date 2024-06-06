use ::rand::Rng;
use macroquad::{
    prelude::*,
    shapes::draw_circle,
    window::{next_frame, request_new_screen_size},
};

const STEP: f32 = 20.;
const HALF_STEP: f32 = STEP / 2.;
const WIDTH: f32 = 600.;
const HEIGHT: f32 = 600.;
const COLUMNS: f32 = WIDTH / STEP;
const ROWS: f32 = HEIGHT / STEP;

fn get_state(a: f32, b: f32, c: f32, d: f32) -> i32 {
    a as i32 + b as i32 * 2 + c as i32 * 4 + d as i32 * 8
}

#[macroquad::main("Random Squares")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    request_new_screen_size(WIDTH, HEIGHT);

    let mut field = [[0.; COLUMNS as usize]; ROWS as usize];

    for i in 0..(ROWS as usize) {
        for j in 0..(COLUMNS as usize) {
            field[i][j] = rng.gen_range(0f32..1f32);
            // print!("{:?} ", field[i][j]);
        }
        // println!();
    }

    loop {
        for i in 0..field.len() {
            for j in 0..field[0].len() {
                draw_circle(
                    j as f32 * STEP + HALF_STEP,
                    i as f32 * STEP + HALF_STEP,
                    5.,
                    Color {
                        r: field[i][j],
                        g: 0.1,
                        b: 0.1,
                        a: 1.,
                    },
                );
            }
        }

        let mut row = 0;

        while row < field.len() - 1 {
            let mut column = 0;
            while column < field[0].len() - 1 {
                if column < field[0].len() - 1 {
                    draw_circle(
                        column as f32 * STEP + STEP,
                        row as f32 * STEP + HALF_STEP,
                        3.,
                        WHITE,
                    );
                }

                if row < field.len() - 1 {
                    draw_circle(
                        column as f32 * STEP + HALF_STEP,
                        row as f32 * STEP + STEP,
                        3.,
                        WHITE,
                    );
                }

                let a = Vec2::new(column as f32 * STEP + STEP, row as f32 * STEP + HALF_STEP);
                let b = Vec2::new(
                    column as f32 * STEP + STEP,
                    row as f32 * STEP + HALF_STEP + STEP,
                );
                let c = Vec2::new(column as f32 * STEP + HALF_STEP, row as f32 * STEP + STEP);
                let d = Vec2::new(
                    column as f32 * STEP + HALF_STEP + STEP,
                    row as f32 * STEP + STEP,
                );

                // draw_circle(a.x, a.y, 3., GREEN);
                // draw_circle(b.x, b.y, 3., VIOLET);
                // draw_circle(c.x, c.y, 3., GREEN);
                // draw_circle(d.x, d.y, 3., BLUE);

                let state = get_state(
                    field[row][column],
                    field[row][column + 1],
                    field[row + 1][column],
                    field[row + 1][column + 1],
                );

                match state {
                    1 => {}
                    2 => {}
                    3 => {}
                    4 => {}
                    5 => {}
                    6 => {}
                    7 => {}
                    8 => {}
                    9 => {}
                    10 => {}
                    11 => {}
                    12 => {}
                    13 => {}
                    14 => {}
                    _ => {}
                }

                // println!("{}", column);
                // println!("{}", row);

                column += 1;
            }
            row += 1;
        }

        next_frame().await;
    }
}

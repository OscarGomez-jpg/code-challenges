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
    a as i32 + (b * 2.) as i32 + (c * 4.) as i32 + (d * 8.) as i32
}

fn apply_rule(a: Vec2, b: Vec2) {
    draw_line(a.x, a.y, b.x, b.y, 2., VIOLET);
}

#[macroquad::main("Marching Squares")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    request_new_screen_size(WIDTH, HEIGHT);

    let mut field = [[0.; COLUMNS as usize]; ROWS as usize];

    (0..(ROWS as usize)).for_each(|i| {
        for j in 0..(COLUMNS as usize) {
            if rng.gen_bool(1. / 2.) {
                field[i][j] = 1.;
            } else {
                field[i][j] = 0.;
            }
            // field[i][j] = rng.gen_range(0f32..1f32);
            // print!("{:?} ", field[i][j]);
        }
        // println!();
    });

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
                let x = column as f32 * STEP;
                let y = row as f32 * STEP;

                // if column < field[0].len() - 1 {
                //     draw_circle(
                //         column as f32 * STEP + STEP,
                //         row as f32 * STEP + HALF_STEP,
                //         3.,
                //         WHITE,
                //     );
                // }
                //
                // if row < field.len() - 1 {
                //     draw_circle(
                //         column as f32 * STEP + HALF_STEP,
                //         row as f32 * STEP + STEP,
                //         3.,
                //         WHITE,
                //     );
                // }

                let a = Vec2::new(x + STEP, y + HALF_STEP);
                let b = Vec2::new(x + HALF_STEP + STEP, y + STEP);
                let c = Vec2::new(x + STEP, y + HALF_STEP + STEP);
                let d = Vec2::new(x + HALF_STEP, y + STEP);

                // draw_circle(a.x, a.y, 3., GREEN);
                // draw_circle(b.x, b.y, 3., VIOLET);
                // draw_circle(c.x, c.y, 3., GRAY);
                // draw_circle(d.x, d.y, 3., BLUE);

                let state = get_state(
                    field[row][column],
                    field[row][column + 1],
                    field[row + 1][column],
                    field[row + 1][column + 1],
                );

                // println!("{state}");

                match state {
                    1 => apply_rule(d, a),
                    2 => apply_rule(a, b),
                    3 => apply_rule(d, b),
                    4 => apply_rule(d, c),
                    5 => apply_rule(a, c),
                    6 => {
                        apply_rule(d, c);
                        apply_rule(a, b)
                    }
                    7 => apply_rule(c, b),
                    8 => apply_rule(c, b),
                    9 => {
                        apply_rule(a, d);
                        apply_rule(b, c)
                    }
                    10 => apply_rule(a, c),
                    11 => apply_rule(d, c),
                    12 => apply_rule(d, b),
                    13 => apply_rule(a, b),
                    14 => apply_rule(a, d),
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

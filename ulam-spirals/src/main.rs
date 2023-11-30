use macroquad::{
    color::BLACK,
    miniquad::window::set_window_size,
    prelude::*,
    window::{clear_background, next_frame},
};

fn _plot_grid(sqr_width: &f32, s_large: &f32, on_x: bool) {
    let mut i = *sqr_width;
    while i < *s_large {
        if on_x {
            draw_line(0., i, *s_large, i, 1.1, WHITE);
        } else {
            draw_line(i, 0., i, *s_large, 1.1, WHITE);
        }
        i += sqr_width;
    }
}

fn check_prime(num: &i32) -> bool {
    if *num == 1 {
        return false;
    } else if *num == 2 {
        return true;
    } else if num % 2 == 0 {
        return false;
    }

    for i in (3..=(f64::from(*num)).sqrt() as i32).step_by(2) {
        if *num % i == 0 {
            return false;
        }
    }

    true
}

#[macroquad::main("Ulam spirals")]
async fn main() {
    clear_background(BLACK);
    let window_size = 600.;
    set_window_size(600, 600);
    let sqr_width = 20.;
    let sqr_height = 20.;

    //Font specifications
    let _font_size = 20.1;

    //Circles drawing attributes
    let radious = 10.;
    let thickness = 1.1;
    let circle_color = WHITE;
    //println!("{}", window_size);

    //Principal logic that adds points
    //moves in order up, left, down, right
    let moves: Vec<(f32, f32)> = vec![
        (0., -sqr_height),
        (-sqr_width, 0.),
        (0., sqr_height),
        (sqr_width, 0.),
    ];

    let mut circles: Vec<(i32, f32, f32)> = Vec::new();

    //First circle
    let mut pos_x = window_size / 2.;
    let mut pos_y = window_size / 2.;
    let mut act_num = 1;
    circles.push((act_num, pos_x, pos_y));

    //Setup
    let mut real_size = 0;
    let mut total_sqrs = 20;
    let mut act_dir = 2;

    while total_sqrs > 0 {
        act_dir += 1;
        act_dir = act_dir % moves.len();

        if act_dir == 3 {
            real_size += 1;
        } else if act_dir == 1 {
            real_size += 1;
            total_sqrs -= 1;
        }

        for _ in 0..real_size {
            act_num += 1;
            pos_x += moves[act_dir].0;
            pos_y += moves[act_dir].1;
            circles.push((act_num, pos_x, pos_y));
        }
    }

    //println!("{}", check_prime(&13));

    loop {
        //plot_grid(&sqr_width, &window_size, false);
        //plot_grid(&sqr_width, &window_size, true);
        let mut last = &circles[0];
        for circle in &circles {
            // draw_text(
            //     &circle.0.to_string(),
            //     circle.1 - font_size / 2.,
            //     circle.2 + radious / 2.,
            //     font_size,
            //     circle_color,
            // );

            draw_line(last.1, last.2, circle.1, circle.2, thickness, circle_color);

            //I can optimize this adding a boolean that indicates if is prime or not
            //But as this is just a short code challenge I am not going to give a deep
            //optimization to the code
            if check_prime(&circle.0) {
                draw_circle_lines(circle.1, circle.2, radious, thickness, circle_color);
            }

            last = circle;
        }

        next_frame().await;
    }
}

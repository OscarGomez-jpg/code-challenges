mod ball;
mod behavior;
mod enemy;
mod player;

use ::rand::Rng;
use ball::Ball;
use behavior::Updatable;
use macroquad::{miniquad::window::set_window_size, prelude::*};
use player::Player;

//Window attributes
const WINDOW_WIDTH: f32 = 1000.;
const WINDOW_HEIGHT: f32 = 700.;

//Balls attributes
const TOTAL_BALLS: usize = 50;
const BALLS_RADIUS: f32 = 5.;

//All initial radius
const INITIAL_RADIUS: f32 = 10.;

fn spawn_ball(balls: &mut Vec<Ball>) {
    let mut rng = ::rand::thread_rng();
    balls.push(Ball::new(Circle::new(
        rng.gen_range(BALLS_RADIUS..(WINDOW_WIDTH - BALLS_RADIUS)),
        rng.gen_range(BALLS_RADIUS..(WINDOW_HEIGHT - BALLS_RADIUS)),
        BALLS_RADIUS,
    )));
}

#[macroquad::main("Agario")]
async fn main() {
    set_window_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    let mut balls: Vec<Ball> = Vec::new();
    let mut player: Player = Player::new(Circle::new(
        WINDOW_WIDTH / 2.,
        WINDOW_HEIGHT / 2.,
        INITIAL_RADIUS,
    ));

    for _ in 0..TOTAL_BALLS {
        spawn_ball(&mut balls);
    }

    loop {
        clear_background(BLACK);

        // if balls.len() < TOTAL_BALLS {
        //     spawn_ball(&mut balls);
        // }
        //
        // for ball in &balls {
        //     draw_circle(ball.shape.x, ball.shape.y, ball.shape.r, WHITE);
        // }

        for ball in &player.balls {
            draw_circle(ball.shape.x, ball.shape.y, ball.shape.r, GREEN);
        }

        player.update();
        next_frame().await;
    }
}

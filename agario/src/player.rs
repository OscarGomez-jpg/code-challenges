use macroquad::prelude::*;

use crate::{
    ball::Ball,
    behavior::{Edible, Splittable, Updatable},
};

#[derive(Debug)]
pub struct Player {
    dir: Vec2,
    acc: f32,
    pub balls: Vec<Ball>,
}

impl Splittable for Player {
    fn split(&mut self) {
        let mut to_add: Vec<Ball> = Vec::new();

        for ball in &mut self.balls {
            ball.shape.r = ball.shape.r / 2.;
            let mut ball_to_add = ball.clone();
            ball_to_add.shape.x += self.dir.x * self.acc + ball.shape.x;
            ball_to_add.shape.y += self.dir.y * self.acc + ball.shape.y;
            to_add.push(ball.clone());
        }

        self.balls.append(&mut to_add);
    }
}

impl Edible for Player {
    fn eat(&mut self, other_ball_radius: f32) {}
}

impl Updatable for Player {
    fn update(&mut self) {
        self.special_input();
        self.dir = Player::move_around();

        for ball in &mut self.balls {
            ball.shape.x += self.dir.x * self.acc;
            ball.shape.y += self.dir.y * self.acc;
        }
    }
}

impl Player {
    pub fn new(shape: Circle) -> Self {
        Self {
            dir: vec2(0., 0.),
            acc: 5.,
            balls: vec![Ball::new(shape)],
        }
    }

    fn move_around() -> Vec2 {
        let mut new_dir = vec2(0., 0.);
        if is_key_down(KeyCode::A) {
            new_dir.x += -1.;
        }

        if is_key_down(KeyCode::D) {
            new_dir.x += 1.;
        }

        if is_key_down(KeyCode::W) {
            new_dir.y += -1.;
        }

        if is_key_down(KeyCode::S) {
            new_dir.y += 1.;
        }

        return new_dir;
    }

    fn special_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.split();
        }
    }
}

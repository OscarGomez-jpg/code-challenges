use macroquad::{
    color::RED,
    math::{Rect, Vec2},
    shapes::draw_rectangle,
    time::get_frame_time,
    window::{next_frame, screen_width},
};
use rand::{rng, Rng};

const CARS_VEL: f32 = 100.0;
const CAR_WIDTH: f32 = 50.0;
const PLAYER_WIDTH: f32 = 50.0;

struct Car {
    pos: Vec2,
    hitbox: Rect,
    dir: Vec2,
    vel: f32,
}

impl Car {
    fn new(pos: Vec2, hitbox: Rect) -> Self {
        Self {
            pos,
            hitbox,
            dir: Vec2::new(0.0, 0.0),
            vel: 0.0,
        }
    }

    fn move_self(&mut self, delta: f32) {
        self.pos += self.dir * delta * self.vel;

        if self.pos.x > screen_width() {
            self.pos.x = -self.hitbox.w + 1.;
        } else if self.pos.x < -self.hitbox.w {
            self.pos.x = screen_width();
        }
    }
}

fn create_cars_list(
    car_list: &mut Vec<Car>,
    total_cars: usize,
    left_space: f32,
    right_space: f32,
    y_align: f32,
    going_left: bool,
    cars_vel: f32,
) {
    let mut rng = rng();
    for i in 0..total_cars {
        let value = rng.random_range(left_space..right_space);
        let x_pos = i as f32 * value;
        let mut car = Car::new(
            Vec2::new(x_pos, y_align),
            Rect::new(x_pos, y_align, CAR_WIDTH, 50.0),
        );

        car.dir = if going_left {
            Vec2::new(-1.0, 0.0)
        } else {
            Vec2::new(1.0, 0.0)
        };

        car.vel = cars_vel;

        car_list.push(car);
    }
}

#[macroquad::main("Froggy")]
async fn main() {
    let mut car_list: Vec<Car> = Vec::new();

    create_cars_list(
        &mut car_list,
        9,
        PLAYER_WIDTH + 5.,
        CAR_WIDTH * 2.,
        100.0,
        true,
        CARS_VEL,
    );

    loop {
        for car in car_list.iter() {
            // draw_circle(car.pos.x, car.pos.y, 10., GREEN);
            draw_rectangle(car.pos.x, car.pos.y, car.hitbox.w, car.hitbox.h, RED);
        }

        for car in car_list.iter_mut() {
            // println!("{}", get_frame_time());
            car.move_self(get_frame_time());
        }

        next_frame().await;
    }
}

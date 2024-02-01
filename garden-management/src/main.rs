use std::{collections::HashMap, usize};

use macroquad::{
    color::{BLACK, GREEN, RED, WHITE},
    math::Circle,
    shapes::draw_circle,
    text::draw_text,
    window::{clear_background, next_frame},
};
use rand::Rng;

struct Plant {
    id: String,
    shape: Circle,
    life_time: usize,
}

impl Plant {
    fn new(id: String, shape: Circle, life_time: usize) -> Self {
        Self {
            id,
            shape,
            life_time,
        }
    }
}

#[macroquad::main("Garden management")]
async fn main() {
    let mut rng = ::rand::thread_rng();
    let mut plants: HashMap<String, Plant> = HashMap::new();
    let mut to_remove: Vec<String> = Vec::new();
    let mut is_day = true;
    let mut hour: f32 = 0.;

    for i in 0..15 {
        let plant = Plant::new(
            i.to_string(),
            Circle::new(
                rng.gen_range(15f32..575f32),
                rng.gen_range(15f32..575f32),
                10.,
            ),
            rng.gen_range(100..1000),
        );

        plants.insert(i.to_string(), plant);
    }

    loop {
        if is_day {
            clear_background(WHITE);
        } else {
            clear_background(BLACK);
        }

        for (_id, plant) in plants.iter() {
            draw_circle(plant.shape.x, plant.shape.y, plant.shape.r, GREEN);
            draw_text(
                &plant.life_time.to_string(),
                plant.shape.x - plant.shape.r,
                plant.shape.y - plant.shape.r - 5.,
                12.,
                RED,
            );
        }

        for (id, plant) in plants.iter_mut() {
            if is_day {
                plant.life_time += rng.gen_range(0..10);
            } else {
                if plant.life_time == 0 {
                    to_remove.push(id.to_string());
                } else if plant.life_time >= 30 {
                    plant.life_time = plant.life_time.wrapping_sub(rng.gen_range(10..30));
                } else if plant.life_time > 1 {
                    plant.life_time = plant
                        .life_time
                        .wrapping_sub(rng.gen_range(0..plant.life_time));
                } else {
                    plant.life_time = 0;
                }
            }
        }

        for removing in &to_remove {
            plants.remove(removing);
        }

        to_remove = Vec::new();

        hour += 0.016;

        if hour >= 12. {
            hour = 0.;
            is_day = !is_day;
        }

        next_frame().await;
    }
}

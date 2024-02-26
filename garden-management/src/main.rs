use std::{collections::HashMap, usize};

use macroquad::{
    color::{BLACK, GREEN, RED, WHITE},
    math::{Circle, Vec2},
    shapes::draw_circle,
    text::draw_text,
    window::{clear_background, next_frame, screen_height, screen_width},
};
use plant::Plant;
use quadtree::{Boundary, Point, Quadtree};
use rand::Rng;

pub mod plant;
pub mod quadtree;

fn duplicate(index: &mut i32, plant: &Plant) -> Plant {
    let mut rng = ::rand::thread_rng();
    *index += 1;
    Plant::new(
        index.to_string(),
        Circle::new(
            plant.shape.x + rng.gen_range(-plant.shape.r..plant.shape.r),
            plant.shape.y + rng.gen_range(-plant.shape.r..plant.shape.r),
            plant.shape.r,
        ),
        rng.gen_range((plant.life_time.wrapping_div(2))..1000),
    )
}

fn create_random_plants(total: i32) -> HashMap<String, Plant> {
    let mut index = 0;
    let mut rng = ::rand::thread_rng();
    let mut plants = HashMap::new();
    while index < total {
        let plant = Plant::new(
            index.to_string(),
            Circle::new(
                rng.gen_range(15f32..575f32),
                rng.gen_range(15f32..575f32),
                10.,
            ),
            rng.gen_range(100..1000),
        );

        plants.insert(index.to_string(), plant.clone());

        index += 1;
    }

    plants
}

fn add_plants_to_quadtree(plants: &HashMap<String, Plant>, qt: &mut Quadtree) {
    for (index, plant) in plants {
        let mut point = Point::new(plant.shape.x, plant.shape.y);
        point.id = index.to_string();
        qt.insert(point);
    }
}

#[macroquad::main("Garden management")]
async fn main() {
    let total_plants = 15;

    let mut qt = Quadtree::new(Boundary::new(0., 0., screen_width(), screen_height()), 4);
    let mut rng = ::rand::thread_rng();
    let mut to_add: Vec<String> = Vec::new();
    let mut to_remove: Vec<String> = Vec::new();
    let mut is_day = true;
    let mut hour: f32 = 0.;
    let mut index = 0;

    let mut boundaries: Vec<Boundary> = Vec::new();
    let boundary_step_h = screen_width() / 12.;
    let boundary_step_v = screen_height() / 12.;
    for i in 0..(boundary_step_h as usize) {
        for j in 0..(boundary_step_v as usize) {
            let check_boundary = Boundary::new(
                i as f32 * boundary_step_h,
                j as f32 * boundary_step_v,
                boundary_step_h,
                boundary_step_v,
            );

            boundaries.push(check_boundary);
        }
    }

    let mut plants: HashMap<String, Plant> = create_random_plants(total_plants);

    loop {
        if is_day {
            clear_background(WHITE);
        } else {
            clear_background(BLACK);
        }

        add_plants_to_quadtree(&plants, &mut qt);

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

            if plant.life_time >= 1500 {
                to_remove.push(id.to_string());
                to_add.push(id.to_string());
            }

            if is_day {
                for boundary in &boundaries {
                    let others = qt.query(*boundary);

                    for other in &others {
                        let point = Vec2::new(other.x, other.y);
                        if plant.shape.contains(&point) {
                            plant.near_plants += 1;
                            //Think an update method, based on certains modifications coming from
                            //an enum, then I should not have any problems related to mutability
                            // let other_plant = plants.get(&other.id).unwrap();
                            // plant.life_time +=
                            //     rng.gen_range(0..(other_plant.life_time.wrapping_div(8)));
                        }
                    }
                }
            }
        }

        // ====================================Painting=========================================

        for (_id, plant) in plants.iter() {
            draw_circle(plant.shape.x, plant.shape.y, plant.shape.r, GREEN);
            // println!("id: {}", plant.id);
            draw_text(
                &plant.life_time.to_string(),
                plant.shape.x - plant.shape.r,
                plant.shape.y - plant.shape.r - 5.,
                12.,
                RED,
            );
        }

        // ==============================Reinstating the quadtree===============================

        qt = Quadtree::new(Boundary::new(0., 0., screen_width(), screen_height()), 4);

        // ============================Queue for adding and deleting============================

        // let mut last_id = String::new();
        for adding in &to_add {
            // println!("Adding from plant: {}", adding);
            for _ in 0..2 {
                let new_plant = duplicate(&mut index, plants.get(adding).unwrap());
                // println!("new {:?}", new_plant);
                // last_id = new_plant.id.clone();
                plants.insert(new_plant.id.clone(), new_plant.clone());
                qt.insert(Point::new(new_plant.shape.x, new_plant.shape.y));
                // println!("Id added: {:?}", plants.get(adding).unwrap());
            }
        }

        to_add = Vec::new();

        for removing in &to_remove {
            plants.remove(removing);
            // println!("last id {}", last_id);
        }

        to_remove = Vec::new();

        // =====================================Daytime========================================

        hour += 0.016;

        if hour >= 12. {
            hour = 0.;
            is_day = !is_day;
        }

        // ====================================================================================

        next_frame().await;
    }
}

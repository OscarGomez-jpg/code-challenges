use std::collections::HashMap;

use ::rand::Rng;
use macroquad::{miniquad::window::set_window_size, prelude::*};
use quadtree::{Boundary, Point, Quadtree};

mod quadtree;

const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = 600.;

struct Particle {
    id: usize,
    shape: Circle,
    color: Color,
}

impl Particle {
    fn new(shape: Circle) -> Self {
        Self {
            id: 0,
            shape,
            color: WHITE,
        }
    }
}

fn distance(obj1: &Circle, obj2: &Circle) -> f32 {
    ((obj2.x - obj1.x).powi(2) + (obj2.y - obj1.y).powi(2)).sqrt()
}

#[macroquad::main("Testing the quadtree")]
async fn main() {
    set_window_size(
        (WINDOW_WIDTH as usize).try_into().unwrap(),
        (WINDOW_HEIGHT as usize).try_into().unwrap(),
    );

    let step = 25.;
    let map_boundary = Boundary::new(0., 0., WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut local_zone = Boundary::new(0., 0., step, step);

    let mut particles: Vec<Particle> = Vec::new();

    let mut rng = ::rand::thread_rng();
    let total_particles: usize = 1000;
    let mut index: usize = 0;

    while index < total_particles {
        let x = rng.gen_range(0f32..WINDOW_WIDTH);
        let y = rng.gen_range(0f32..WINDOW_HEIGHT);

        particles.push(Particle::new(Circle::new(x, y, 5.)));
        particles[index].id = index;
        index += 1;
    }

    let mut qt;
    loop {
        qt = Quadtree::new(map_boundary, 4);
        for value in &particles {
            draw_circle(value.shape.x, value.shape.y, value.shape.r, value.color);
            qt.insert(Point::new(value.shape.x, value.shape.y, value.id));
        }

        for value in &mut particles {
            value.shape.x += rng.gen_range(-1f32..1f32);
            value.shape.y += rng.gen_range(-1f32..1f32);
        }

        let mut others: Vec<Point>;

        for i in 0..particles.len() {
            local_zone.x = particles[i].shape.x - step;
            local_zone.y = particles[i].shape.y - step;

            others = qt.query(local_zone);

            for other in &others {
                if particles[i]
                    .shape
                    .overlaps(&Circle::new(other.x, other.y, particles[i].shape.r))
                {
                    particles[i].color = GRAY;
                } else {
                    particles[i].color = WHITE;
                }
            }
        }

        next_frame().await;
    }
}

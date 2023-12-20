use std::usize;

use ::rand::Rng;
use macroquad::prelude::*;

const TOTAL_PARTICLES: usize = 100;
const GRAVITATION: f32 = 6.67e-11;

struct Particle {
    shape: Circle,
    acc: f32,
    dir: Vec2,
    m: f32,
}

impl Particle {
    fn new(shape: Circle, m: f32) -> Self {
        Self {
            shape,
            acc: 0.,
            dir: vec2(0., 0.),
            m,
        }
    }
}

fn grav_attraction(mass1: &Particle, mass2: &Particle) -> Vec2 {
    let r =
        ((mass2.shape.x - mass1.shape.x).powi(2) + (mass2.shape.y - mass1.shape.y).powi(2)).sqrt();
    let x = GRAVITATION * (mass2.m * mass1.m * (mass2.shape.x - mass1.shape.x)) / r.powi(3);
    let y = GRAVITATION * (mass2.m * mass1.m * (mass2.shape.y - mass1.shape.y)) / r.powi(3);
    vec2(x, y)
}

#[macroquad::main("Attractors")]
async fn main() {
    let mut rng = ::rand::thread_rng();
    let mut particles: Vec<Particle> = Vec::new();

    let attractor: Particle = Particle::new(
        Circle::new(screen_width() / 2., screen_height() / 2., 50.),
        100000000000.,
    );

    particles.push(attractor);

    for _ in 0..TOTAL_PARTICLES {
        let new_circle = Circle::new(
            rng.gen_range(0f32..screen_width()),
            rng.gen_range(0f32..screen_height()),
            rng.gen_range(2f32..5f32),
        );

        let m = new_circle.r.powi(4);

        let mut new_particle = Particle::new(new_circle, m);

        new_particle.acc = 10.;

        particles.push(new_particle);
    }

    loop {
        clear_background(BLACK);

        for particle in &particles {
            draw_circle(particle.shape.x, particle.shape.y, particle.shape.r, WHITE);
        }

        for particle in &mut particles {
            particle.shape.x += particle.acc * particle.dir.x;
            particle.shape.y += particle.acc * particle.dir.y;
        }

        for i in 1..particles.len() {
            if i < particles.len() {
                particles[i].dir = grav_attraction(&particles[i], &particles[0]);
                if particles[i].shape.overlaps(&particles[0].shape) {
                    let deleted = particles.remove(i);
                    particles[0].shape.r += deleted.shape.r / 2.;
                    particles[0].m += deleted.m;
                }
            }
        }

        next_frame().await;
    }
}

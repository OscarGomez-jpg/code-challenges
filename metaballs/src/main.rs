use macroquad::{miniquad::window::set_window_size, prelude::*};

struct Particle {
    shape: Circle,
    dir: Vec2,
    acc: f32,
}

impl Particle {
    fn new(shape: Circle) -> Self {
        Self {
            shape,
            dir: Vec2::ZERO,
            acc: 350.,
        }
    }
}

fn _convert_pos_to_color(pos: Vec2) -> Vec3 {
    vec3(
        pos.x + screen_width() / 2.,
        pos.y + screen_height() / 2.,
        pos.x / pos.y,
    )
}

#[macroquad::main("Meat balls")]
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    let total = 1;
    for _ in 0..total {
        let new_circle = Circle::new(screen_width() / 2., screen_height() / 2., 20.);
        let new_particle = Particle::new(new_circle);
        particles.push(new_particle);
    }
    set_window_size(600, 600);

    loop {
        clear_background(BLACK);

        for particle in &particles {
            draw_circle(particle.shape.x, particle.shape.y, particle.shape.r, WHITE);
        }

        let (mouse_pos_x, mouse_pos_y) = mouse_position();
        for particle in &mut particles {
            particle.dir = vec2(
                particle.shape.x - mouse_pos_x,
                particle.shape.y - mouse_pos_y,
            )
            .normalize();
            particle.shape.x -= particle.dir.x * particle.acc * get_frame_time();
            particle.shape.y -= particle.dir.y * particle.acc * get_frame_time();
        }

        next_frame().await;
    }
}

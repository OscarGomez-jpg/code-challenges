use ::rand::Rng;
use macroquad::{miniquad::window::set_window_size, prelude::*};

mod quadtree;
use quadtree::{Boundary, Point, Quadtree};

const WINDOW_SIZE: f32 = 600.;

#[macroquad::main("Quadtree")]
async fn main() {
    //This one is more a translation than a build by myself
    //because it is hard to just think in how to properly manage
    //autoreferences in rust, so it was really a headache to
    //figure out everything by myself
    let mut rng = ::rand::thread_rng();
    set_window_size(
        (WINDOW_SIZE as usize).try_into().unwrap(),
        (WINDOW_SIZE as usize).try_into().unwrap(),
    );
    let mut particles: Vec<Vec2> = Vec::new();
    // let total_particles = 5;

    let mut quadtree: Quadtree = Quadtree::new(Boundary::new(0., 0., WINDOW_SIZE, WINDOW_SIZE), 4);

    // for _ in 0..total_particles {
    //     let x = rng.gen_range(0f32..=WINDOW_SIZE);
    //     let y = rng.gen_range(0f32..=WINDOW_SIZE);
    //
    //     particles.push(Vec2 { x, y });
    //     quadtree.insert(Point::new(x, y));
    // }

    // let mut qtree: Vec<(f32, f32, f32, f32)> = Vec::new();
    // quadtree.show(&mut qtree);

    // println!("points: {}", quadtree.points.len());

    loop {
        clear_background(WHITE);

        let mut vqt: Vec<(f32, f32, f32, f32)> = Vec::new();

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            particles.push(Vec2 { x, y });
            quadtree.insert(Point { x, y });
        }

        for particle in &particles {
            draw_circle(particle.x, particle.y, 2., BLACK);
        }

        quadtree.show(&mut vqt);

        for rect in &vqt {
            draw_rectangle_lines(rect.0, rect.1, rect.2, rect.3, 1.1, BLACK);
        }

        next_frame().await;
    }
}

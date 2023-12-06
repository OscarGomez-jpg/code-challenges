use std::{cell::RefCell, rc::Rc};

use macroquad::{miniquad::window::set_window_size, prelude::*};

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

//This is an edited comment
const WINDOW_SIZE: f32 = 600.;
#[derive(Debug)]
struct Particle {
    x: f32,
    y: f32,
}

#[macroquad::main("Quadtree")]
async fn main() {
    set_window_size(600, 600);
    let mut particles: Vec<Particle> = Vec::new();
    particles.push(Particle {
        x: WINDOW_SIZE / 2.,
        y: WINDOW_SIZE / 2.,
    });

    loop {
        clear_background(WHITE);

        for particle in &particles {
            draw_circle(particle.x, particle.y, 10., BLACK);
        }

        next_frame().await;
    }
}

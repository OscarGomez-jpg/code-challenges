use macroquad::prelude::*;

struct Walker {
    points: Vec<Vec2>,
}

impl Walker {
    fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}

#[macroquad::main("Difussion limited aggregation")]
async fn main() {
    println!("Hello, world!");
    loop {
        next_frame().await;
    }
}

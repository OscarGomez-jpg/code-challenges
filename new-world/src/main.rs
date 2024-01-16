use macroquad::prelude::*;

struct Specimen {
    shape: Circle,
    acc: f32,
    dir: Vec2,
    race: String,
}

impl Specimen {
    fn new(shape: Circle, acc: f32, dir: Vec2, race: String) -> Self {
        Self {
            shape,
            acc,
            dir,
            race,
        }
    }
}

#[macroquad::main("New World")]
async fn main() {
    //In this world there are going to be various species
    //all of them have their own pros and contras
    //But when they are hungry wether they are carnivorous or not
    //They will eat each other.
    let window_size = 600.;
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}

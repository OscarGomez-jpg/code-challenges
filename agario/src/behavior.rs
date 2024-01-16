pub trait Updatable {
    fn update(&mut self);
}

pub trait Splittable {
    fn split(&mut self);
}

pub trait Edible {
    fn eat(&mut self, other_ball_radius: f32);
}

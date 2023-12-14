use macroquad::math::{vec2, Circle, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub shape: Circle,
    pub acc: f32,
    pub vec: Vec2,
}

impl Ball {
    pub fn new(shape: Circle) -> Self {
        Self {
            shape,
            acc: 0.,
            vec: vec2(0., 0.),
        }
    }
}

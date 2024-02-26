use macroquad::math::Circle;

#[derive(Debug, Clone)]
pub struct Plant {
    pub id: String,
    pub shape: Circle,
    pub life_time: usize,
    pub near_plants: usize,
}

impl Plant {
    pub fn new(id: String, shape: Circle, life_time: usize) -> Self {
        Self {
            id,
            shape,
            life_time,
            near_plants: 0,
        }
    }
}

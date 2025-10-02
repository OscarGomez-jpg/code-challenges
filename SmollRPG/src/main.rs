use macroquad::prelude::*;

const SQUARE_SIZE: i16 = 20;

trait GameEntity {
    fn name(&self) -> &str;
}

struct Player {
    size: f32,
    hitbox: Circle,
}

impl GameEntity for Player {
    fn name(&self) -> &str {
        "Player"
    }
}

impl Player {
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Player {
            size: radius,
            hitbox: Circle::new(x, y, radius),
        }
    }

    fn draw(&self) {
        draw_text("@", self.hitbox.x, self.hitbox.y, self.size, RED);
    }
}

#[macroquad::main("Smoll RPG")]
async fn main() {
    let squares_horizontal = screen_width() / SQUARE_SIZE as f32;
    let squares_vertical = screen_height() / SQUARE_SIZE as f32;

    let board: Vec<Vec<Option<Box<dyn GameEntity>>>> = Vec::new()
        .into_iter()
        .chain((0..squares_vertical as i32).map(|_| {
            (0..squares_horizontal as i32)
                .map(|_| None)
                .collect::<Vec<_>>()
        }))
        .collect();

    let mut player = Player::new((SQUARE_SIZE * 5) as f32, (SQUARE_SIZE * 5) as f32, 20.);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::Left) {
            player.hitbox.x -= SQUARE_SIZE as f32;
        } else if is_key_pressed(KeyCode::Right) {
            player.hitbox.x += SQUARE_SIZE as f32;
        } else if is_key_pressed(KeyCode::Up) {
            player.hitbox.y -= SQUARE_SIZE as f32;
        } else if is_key_pressed(KeyCode::Down) {
            player.hitbox.y += SQUARE_SIZE as f32;
        }

        // Draw the player
        player.draw();

        draw_circle(player.hitbox.x, player.hitbox.y, 5., GREEN);

        for (i, row) in board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                draw_circle(
                    j as f32 * SQUARE_SIZE as f32,
                    i as f32 * SQUARE_SIZE as f32,
                    1.,
                    WHITE,
                );

                // if let Some(entity) = cell {
                //     draw_text(&entity.name(), (j * 50) as f32, (i * 50) as f32, 20., WHITE);
                // }
            }
        }

        next_frame().await;
    }
}

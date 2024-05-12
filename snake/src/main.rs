use ::rand::Rng;
use macroquad::prelude::*;

const FRAME_LIMIT: i32 = 10;
const STEP: f32 = 20.0;

struct Piece {
    is_head: bool,
    pos: Rect,
    color: Color,
}

impl Piece {
    fn new(is_head: bool, pos: Rect, color: Color) -> Self {
        Self {
            is_head,
            pos,
            color,
        }
    }
}

struct Food {
    pos: Rect,
    color: Color,
}

impl Food {
    fn new(pos: Rect, color: Color) -> Self {
        Self { pos, color }
    }
}

fn add_piece(snake: &mut Vec<Piece>, color: Color) {
    let n_piece = Piece::new(false, snake[snake.len() - 1].pos, color);
    snake.push(n_piece);
}

fn gen_food(color: Color) -> Food {
    let mut rng = ::rand::thread_rng();
    let new_x = (rng.gen_range(0f32..screen_width()) / STEP).floor() * STEP;
    let new_y = (rng.gen_range(0f32..screen_height()) / STEP).floor() * STEP - (STEP / 2.);

    Food::new(Rect::new(new_x, new_y, STEP, STEP), color)
}

#[macroquad::main("Snake")]
async fn main() {
    request_new_screen_size(600., 600.);
    let width = screen_width();
    let height = screen_height();

    let mut snake: Vec<Piece> = Vec::new();
    let mut last_pos: (f32, f32) = (width / STEP, height / STEP);

    // Order: Left, Right, Up, Down
    let dir = [(-1., 0.), (1., 0.), (0., -1.), (0., 1.)];
    let mut actual_dir = 1;
    let mut frame_counter: i32 = 0;
    let mut temp_pos: (f32, f32);
    let mut actual_food = gen_food(GREEN);
    let mut score = 0;

    for i in 0..3 {
        if i == 0 {
            let n_piece = Piece::new(true, Rect::new(last_pos.0, last_pos.1, STEP, STEP), RED);
            snake.push(n_piece);
        } else {
            add_piece(&mut snake, WHITE);
        }
        last_pos = (last_pos.0, last_pos.1 + STEP);
    }

    loop {
        for piece in &snake {
            draw_rectangle(piece.pos.x, piece.pos.y, STEP, STEP, piece.color);
        }

        draw_rectangle(
            actual_food.pos.x,
            actual_food.pos.y,
            STEP,
            STEP,
            actual_food.color,
        );

        let score_string = format!("Score: {}", score);

        draw_text(&score_string, 0., 10., 14., WHITE);

        if is_key_pressed(KeyCode::A) {
            actual_dir = 0;
        } else if is_key_pressed(KeyCode::D) {
            actual_dir = 1;
        } else if is_key_pressed(KeyCode::W) {
            actual_dir = 2;
        } else if is_key_pressed(KeyCode::S) {
            actual_dir = 3;
        }

        if frame_counter == FRAME_LIMIT - 1 {
            for i in 0..snake.len() {
                temp_pos = (snake[i].pos.x, snake[i].pos.y);

                if snake[i].is_head {
                    snake[i].pos.x += STEP * dir[actual_dir].0;
                    snake[i].pos.y += STEP * dir[actual_dir].1;

                    if snake[i].pos.intersect(actual_food.pos).is_some() {
                        add_piece(&mut snake, WHITE);
                        actual_food = gen_food(GREEN);
                        score += 1;
                    }

                    // println!("h_x: {}, h_y: {}", snake[i].pos.x, snake[i].pos.y);
                } else {
                    snake[i].pos.x = last_pos.0;
                    snake[i].pos.y = last_pos.1;
                }

                last_pos = temp_pos;

                // println!("i: {}, h_x: {}, h_y: {}", i, snake[i].pos.x, snake[i].pos.y);
            }
        }

        if frame_counter == FRAME_LIMIT {
            frame_counter = 0;
        }
        frame_counter += 1;

        // println!("{frame_counter}");

        next_frame().await;
    }
}

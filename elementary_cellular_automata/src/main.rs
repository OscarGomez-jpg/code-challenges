use ::rand::Rng;
use macroquad::prelude::*;
use maplit::hashmap;

const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 600.0;
const CELL_SIZE: f32 = 5.0;

fn rule_set(cells: &Vec<bool>) -> Vec<bool> {
    // println!("Number of old cells: {}", cells.len());
    let rules = hashmap! {
        "000" => false,
        "001" => true,
        "010" => true,
        "011" => true,
        "100" => true,
        "101" => true,
        "110" => true,
        "111" => false,
    };

    let mut old_state = String::new();
    let mut new_line = Vec::new();

    new_line.push(cells[0]);
    for i in 0..(cells.len() - 2) {
        for j in 0..3 {
            let to_add = if cells[i + j] { '1' } else { '0' };
            old_state.push(to_add);
        }

        new_line.push(*rules.get(old_state.as_str()).unwrap());

        old_state = String::new();
    }
    new_line.push(cells[cells.len() - 1]);

    new_line
}

#[macroquad::main("Elementary Cellular Automata")]
async fn main() {
    let mut rng = ::rand::thread_rng();
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut cells = Vec::new();
    for _ in 0..(SCREEN_WIDTH / CELL_SIZE) as usize {
        cells.push(rng.gen_bool(1.0 / 2.0));
    }

    let mut cells_matrix: Vec<Vec<bool>> = Vec::new();
    cells_matrix.push(cells);

    let mut frame = 0;

    loop {
        let mut y = 0.;
        (0..cells_matrix.len()).for_each(|i| {
            let mut x;
            for j in 0..cells_matrix[i].len() {
                x = j as f32 * CELL_SIZE;

                let color = if cells_matrix[i][j] { GRAY } else { WHITE };

                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
            }
            y += CELL_SIZE;
        });

        if frame == 1 && (cells_matrix.len() as f32) < (SCREEN_HEIGHT / CELL_SIZE) {
            // println!("CellM lines: {}", cells_matrix.len());
            let next = cells_matrix.len();
            let result = rule_set(&cells_matrix[next - 1]);
            cells_matrix.push(result);
            frame = 0;
        }

        frame += 1;

        next_frame().await
    }
}

use macroquad::{
    miniquad::window::set_window_size,
    prelude::WHITE,
    shapes::draw_rectangle,
    window::{clear_background, next_frame},
};

fn gen_maze(maze: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let dirs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    if !maze[y][x] {
        return;
    }

    maze[y][x] = true;

    for i in 0..dirs.len() {
        let new_x = x as i32 + dirs[i].0;
        let new_y = y as i32 + dirs[i].1;
        if new_x >= 0 && new_y >= 0 && new_x < maze[0].len() as i32 && new_y < maze.len() as i32 {
            gen_maze(maze, x as usize, y as usize);
        }
    }
}

#[macroquad::main("Maze generator")]
async fn main() {
    let window_size = 600.;
    let step = 20.;

    set_window_size(600, 600);

    let mut map: Vec<Vec<bool>> =
        vec![vec![false; (window_size / step) as usize]; (window_size / step) as usize];

    gen_maze(&mut map, 5, 5);

    loop {
        clear_background(WHITE);
        for row in 0..((600 as f32 / step) as usize) {
            for column in 0..((600 as f32 / step) as usize) {
                if map[row][column] {
                    draw_rectangle(column as f32 * step, row as f32 * step, step, step, WHITE);
                }
            }
        }
        next_frame().await;
    }
}

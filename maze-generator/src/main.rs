use macroquad::{miniquad::window::set_window_size, window::next_frame};

fn gen_maze(maze: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let dirs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    if !maze[y][x] {
        maze[y][x] = true;
    }
}

#[macroquad::main("Maze generator")]
async fn main() {
    let window_size = 600.;
    let step = 20.;

    set_window_size(600, 600);

    let mut map: Vec<Vec<bool>> =
        vec![vec![false; (window_size / step) as usize]; (window_size / step) as usize];

    loop {
        next_frame().await;
    }
}

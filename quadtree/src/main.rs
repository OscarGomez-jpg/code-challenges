use ::rand::Rng;
use macroquad::{miniquad::window::set_window_size, prelude::*};

mod quadtree;
use quadtree::{Boundary, Point, Quadtree};

const WINDOW_SIZE: f32 = 600.;

#[macroquad::main("Quadtree")]
async fn main() {
    //This is half a translation and a build by myself
    //because it is hard to think in how to properly manage
    //cyclic implementations in rust, so it was really a headache to
    //figure out everything by myself anyway, it also has part
    //of my design in it
    let mut rng = ::rand::thread_rng();
    set_window_size(
        (WINDOW_SIZE as usize).try_into().unwrap(),
        (WINDOW_SIZE as usize).try_into().unwrap(),
    );
    let mut particles: Vec<Vec2> = Vec::new();
    let total_particles = 500;

    let mut quadtree: Quadtree = Quadtree::new(Boundary::new(0., 0., WINDOW_SIZE, WINDOW_SIZE), 4);
    let mut range = Boundary::new(300., 300., 50., 50.);
    for _ in 0..total_particles {
        let x = rng.gen_range(0f32..=WINDOW_SIZE);
        let y = rng.gen_range(0f32..=WINDOW_SIZE);

        particles.push(Vec2 { x, y });
        quadtree.insert(Point::new(x, y));
    }

    // let found: Vec<Point> = quadtree.query(range);
    //
    // println!("points: {}", quadtree.points.len());
    //

    loop {
        clear_background(WHITE);

        //The vec of points with all the points inside an area in the tree

        //Mouse actions, just like positioning the range or create new points
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();

            range.x = x;
            range.y = y;
            // particles.push(vec2(x, y));
            // quadtree.insert(Point::new(x, y));
        }

        //Drawing of each particle
        for particle in &particles {
            draw_circle(particle.x, particle.y, 3., BLACK);
        }

        //Generates the Vec that prints the tree
        let vqt = quadtree.show();

        //Draws the tree itself
        for rect in &vqt {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.1, BLACK);
        }

        //Draw the points on the canvas
        if particles.len() > 0 {
            let found: Vec<Point> = quadtree.query(range);
            for point in &found {
                draw_circle(point.x, point.y, 3., GREEN);
            }
            // println!("total: {}", found.len));
        }

        //Draws a delimited area known as range
        draw_rectangle_lines(range.x, range.y, range.w, range.h, 3.1, GREEN);

        next_frame().await;
    }
}

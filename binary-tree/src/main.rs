use macroquad::prelude::*;

fn make_tree(points: &mut Vec<(Vec2, Vec2)>, hor: bool, orig: Vec2, deepnest: usize, len: f32) {
    if deepnest == 0 {
        return;
    }

    let spacing = 2.;

    let new_points = if hor {
        (
            vec2(orig.x - len / spacing, orig.y),
            vec2(orig.x + len / spacing, orig.y),
        )
    } else {
        (
            vec2(orig.x, orig.y - len / spacing),
            vec2(orig.x, orig.y + len / spacing),
        )
    };

    points.push(new_points);

    let distance = new_points.0.distance(new_points.1);
    let punishment = 3.;

    make_tree(
        points,
        !hor,
        new_points.0,
        deepnest - 1,
        distance - distance / punishment,
    );
    make_tree(
        points,
        !hor,
        new_points.1,
        deepnest - 1,
        distance - distance / punishment,
    );
}

#[macroquad::main("Binary tree")]
async fn main() {
    request_new_screen_size(600., 600.);
    let mut points = Vec::new();
    make_tree(&mut points, true, Vec2::new(300., 300.), 10, 250.);

    loop {
        for point in &points {
            draw_line(
                point.0.x,
                point.0.y,
                point.1.x,
                point.1.y,
                2.,
                Color {
                    r: (point.0.x + point.1.x) / 600.,
                    g: (point.0.y + point.1.y) / 600.,
                    b: 0.1,
                    a: 1.,
                },
            );
        }

        next_frame().await;
    }
}

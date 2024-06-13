use macroquad::{prelude::*, telemetry::disable};

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

    make_tree(points, !hor, new_points.0, deepnest - 1, distance);
    make_tree(points, !hor, new_points.1, deepnest - 1, distance);
}

#[macroquad::main("Binary tree")]
async fn main() {
    let mut points = Vec::new();
    make_tree(&mut points, true, Vec2::new(300., 300.), 10, 50.);

    // for point in &points {
    //     println!(
    //         "x1: {} x2: {} y1: {} y2: {}",
    //         point.0.x, point.1.x, point.0.y, point.1.y
    //     );
    // }

    loop {
        for point in &points {
            draw_line(
                point.0.x,
                point.0.y,
                point.1.x,
                point.1.y,
                2.,
                Color {
                    r: point.0.x / 600.,
                    g: point.1.x / 600.,
                    b: 0.1,
                    a: 1.,
                },
            );
        }

        next_frame().await;
    }
}

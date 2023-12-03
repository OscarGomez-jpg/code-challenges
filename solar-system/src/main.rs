use macroquad::prelude::*;

#[macroquad::main("Solar system")]
async fn main() {
    let separation = [0., 5., 10., 15., 20.];
    let velocities = [0., 0.01, -0.02, 0.025, 0.005];
    let mut actuals: Vec<f32> = vec![0., 0., 0., 0., 0.];
    let total = 365.;

    //I know that I can improve this implementation by adding their corresponding
    //structures to planets, so it can be easily scaled and proper managed, but for now
    //it will stay with the easy implementation
    loop {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_sphere(
            vec3(
                actuals[0].cos() * separation[0],
                0.,
                actuals[0].sin() * separation[0],
            ),
            2.,
            None,
            YELLOW,
        );
        draw_sphere(
            vec3(
                actuals[1].cos() * separation[1],
                0.,
                actuals[1].sin() * separation[1],
            ),
            0.75,
            None,
            BROWN,
        );
        draw_sphere(
            vec3(
                actuals[2].cos() * separation[2],
                0.,
                actuals[2].sin() * separation[2],
            ),
            0.95,
            None,
            GREEN,
        );
        draw_sphere(
            vec3(
                actuals[3].cos() * separation[3],
                0.,
                actuals[3].sin() * separation[3],
            ),
            1.,
            None,
            BLUE,
        );
        draw_sphere(
            vec3(
                actuals[4].cos() * separation[4],
                0.,
                actuals[4].sin() * separation[4],
            ),
            0.8,
            None,
            ORANGE,
        );

        for i in 0..separation.len() {
            actuals[i] += velocities[i];
            actuals[i] = actuals[i] % total;
        }

        next_frame().await
    }
}

use macroquad::{prelude::*, ui::root_ui};

struct Attractor {
    path: Vec<Vec3>,
    total_points: usize,
    dt: f32,
    sigma: f32,
    rho: f32,
    beta: f32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
}

impl Attractor {
    fn new() -> Self {
        Self {
            path: Vec::new(),
            total_points: 200,
            dt: 0.,
            sigma: 0.,
            rho: 0.,
            beta: 0.,
            pos_x: 0.,
            pos_y: 0.,
            pos_z: 0.,
        }
    }

    fn update(&mut self) {
        let dx = self.sigma * (self.pos_y - self.pos_x);
        let dy = self.pos_x * (self.rho - self.pos_z) - self.pos_y;
        let dz = self.pos_x * self.pos_y - self.beta * self.pos_z;
        self.pos_x += self.dt * dx;
        self.pos_y += self.dt * dy;
        self.pos_z += self.dt * dz;

        self.path.push(vec3(self.pos_x, self.pos_y, self.pos_z));
    }
}

struct Camera {
    position: Vec3,
    rotation_speed: f32,
    zoom_speed: f32,
}

impl Camera {
    fn new() -> Self {
        Self {
            position: vec3(30.0, 0.0, 0.0), // Set the initial position at a 45° angle
            rotation_speed: 0.75,
            zoom_speed: 0.25,
        }
    }

    fn update(&mut self) {
        // Update camera logic here
        // Rotate the camera based on the current view direction
        // (This logic can be adjusted based on your needs)
        let rotation_matrix = Mat4::from_rotation_y(get_frame_time() as f32 * self.rotation_speed);
        self.position = rotation_matrix.transform_point3(self.position);

        // Zoom in/out using the scroll wheel
        self.position.y += mouse_wheel().1 * self.zoom_speed;
    }
}

#[macroquad::main("Lorenz attractor")]
async fn main() {
    let mut camera = Camera::new();
    let camera_rotation_speed = 0.75;
    let mut is_camera_rotating = false;

    let mut attractor = Attractor::new();
    attractor.dt = 0.02;
    attractor.sigma = 10.;
    attractor.rho = 28.;
    attractor.beta = 2.7;
    attractor.pos_x = 2.;
    attractor.pos_y = 1.;
    attractor.pos_z = 1.;
    attractor.total_points = 200;

    let mut dt = 0.02;
    let mut sigma = 10.;
    let mut rho = 28.;
    let mut beta = 2.7;
    let mut pos_x = 2.;
    let mut pos_y = 1.;
    let mut pos_z = 1.;
    let mut total_points = String::from("200");

    loop {
        clear_background(BLACK);

        camera.update();

        set_camera(&Camera3D {
            position: camera.position.into(),
            up: vec3(0.0, 1.0, 0.0).into(),
            target: vec3(0.0, 0.0, 0.0).into(),
            ..Default::default()
        });

        if is_key_down(KeyCode::Q) {
            if is_camera_rotating {
                camera.rotation_speed = 0.;
            } else {
                camera.rotation_speed = camera_rotation_speed;
            }
            is_camera_rotating = !is_camera_rotating;
        }

        //draw_grid(20, 1., WHITE, WHITE);

        root_ui().slider(0, "dt", 0.01f32..1f32, &mut dt);
        root_ui().slider(1, "Sigma", -50f32..50f32, &mut sigma);
        root_ui().slider(2, "rho", -50f32..50f32, &mut rho);
        root_ui().slider(3, "beta", -50f32..50f32, &mut beta);
        root_ui().slider(4, "initial pos x", -10f32..10f32, &mut pos_x);
        root_ui().slider(5, "initial pos y", -10f32..10f32, &mut pos_y);
        root_ui().slider(6, "initial pos z", -10f32..10f32, &mut pos_z);
        root_ui().input_text(7, "Total points", &mut total_points);

        if root_ui().button(None, "Reset") {
            attractor.dt = dt;
            attractor.sigma = sigma;
            attractor.rho = rho;
            attractor.beta = beta;
            attractor.pos_x = pos_x;
            attractor.pos_y = pos_y;
            attractor.pos_z = pos_z;
            attractor.total_points = total_points.parse::<usize>().unwrap();

            attractor.path = Vec::new();
        }

        if attractor.path.len() < attractor.total_points {
            attractor.update();
        }

        let mut last = attractor.path[0];
        for node in &attractor.path {
            draw_line_3d(last, *node, WHITE);
            draw_sphere(*node, 0.2, None, WHITE);
            last = *node;
        }

        set_default_camera();

        next_frame().await
    }
}

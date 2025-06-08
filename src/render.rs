use kiss3d::{
    camera::ArcBall,
    nalgebra::{Point3, Vector3},
    window::{CanvasSetup, Window},
};

pub trait Render {
    fn draw_axes(&mut self);
    fn window_and_camera_set_up() -> (Window, ArcBall);
}

impl Render for Window {
    fn draw_axes(&mut self) {
        let origin = Point3::origin();

        self.draw_line(
            &origin,
            &Point3::new(1.0, 0.0, 0.0),
            &Point3::new(1.0, 0.0, 0.0),
        );

        // Draw Y axis in green (EXPIRY)
        self.draw_line(
            &origin,
            &Point3::new(0.0, 1.0, 0.0),
            &Point3::new(0.0, 1.0, 0.0),
        );

        // Draw Z axis in blue (IV)
        self.draw_line(
            &origin,
            &Point3::new(0.0, 0.0, 1.0),
            &Point3::new(0.0, 0.0, 1.0),
        );
    }

    fn window_and_camera_set_up() -> (Window, ArcBall) {
        let window = Window::new_with_setup(
            "Volatility Surface Mesh",
            800,
            600,
            CanvasSetup {
                samples: kiss3d::window::NumSamples::Eight,
                vsync: true,
            },
        );

        // Define starting camera position - we want to look directly at the centre of the plot from the front (in the x-z plane)
        let eye: Point3<f32> = Point3::new(0.0, -2.5, 0.0);
        let at: Point3<f32> = Point3::new(0.5, 0.0, 0.5);
        let up: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

        let mut camera = ArcBall::new(eye, at);
        camera.set_up_axis(up);

        (window, camera)
    }
}

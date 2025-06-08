use kiss3d::{nalgebra::Point3, window::Window};

pub trait Render {
    fn draw_axes(&mut self);
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
}

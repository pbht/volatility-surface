mod plot;
mod render;
mod types;
mod utils;

use anyhow::Result;
use kiss3d::camera::ArcBall;
use std::{cell::RefCell, rc::Rc};

use kiss3d::nalgebra::{Point3, Unit, Vector3};
use kiss3d::window::{CanvasSetup, Window};

use crate::plot::construct_mesh;
use crate::render::Render;
use crate::types::{DeribitDataPoint, DeribitWebSocketMessage};

fn main() -> Result<()> {
    let (mut window, mut camera) = Window::window_and_camera_set_up();

    // Test data
    let raw_options = serde_json::from_str::<DeribitWebSocketMessage>(include_str!(
        "../resources/test_data.json"
    ))?;

    //Business logic
    let deribit_points: Vec<DeribitDataPoint> = raw_options
        .params
        .data
        .into_iter()
        .filter_map(|data_point| data_point.into_full())
        .map(|option| option.into_data_point())
        .collect();

    let mesh = construct_mesh(deribit_points);
    let mut mesh_node = window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));

    mesh_node.recompute_normals();
    mesh_node.set_color(0.4, 0.7, 1.0);

    window.set_light(kiss3d::light::Light::StickToCamera);
    while window.render_with_camera(&mut camera) {
        window.draw_axes();

        // mesh_node.recompute_normals();
    }

    Ok(())
}

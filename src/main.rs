mod plot;
mod render;
mod types;
mod utils;

use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

use kiss3d::nalgebra::Vector3;
use kiss3d::window::{CanvasSetup, Window};

use crate::plot::construct_mesh;
use crate::render::Render;
use crate::types::{DeribitDataPoint, DeribitWebSocketMessage};

fn main() -> Result<()> {
    // Test data
    let raw_options = serde_json::from_str::<DeribitWebSocketMessage>(include_str!(
        "../resources/test_data.json"
    ))?;

    //Business logic
    let deribit_points: Vec<DeribitDataPoint> = raw_options
        .params
        .data
        .into_iter()
        .map(|point| point.into_full().unwrap().into_data_point())
        .collect();

    let mut window = Window::new_with_setup(
        "Volatility Surface Mesh",
        800,
        600,
        CanvasSetup {
            samples: kiss3d::window::NumSamples::Eight,
            vsync: true,
        },
    );

    let mesh = construct_mesh(deribit_points);
    let mut mesh_node = window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));

    mesh_node.recompute_normals();
    mesh_node.set_color(0.3, 1.0, 1.0);

    window.set_light(kiss3d::light::Light::StickToCamera);
    while window.render() {
        window.draw_axes();

        // mesh_node.recompute_normals();
    }

    Ok(())
}

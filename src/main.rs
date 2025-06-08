mod plot;
mod render;
mod types;
mod utils;
mod websocket;

use anyhow::Result;
use std::sync::mpsc;
use std::{cell::RefCell, rc::Rc};

use kiss3d::nalgebra::Vector3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use crate::plot::construct_mesh;
use crate::render::Render;
use crate::types::{DeribitDataPoint, RawDeribitOption};
use crate::websocket::listen_for_deribit_data;

#[tokio::main]
async fn main() -> Result<()> {
    let (mut window, mut camera) = Window::window_and_camera_set_up();

    let (tx, rx) = mpsc::channel::<Vec<RawDeribitOption>>();
    tokio::spawn(async move {
        if let Err(e) = listen_for_deribit_data(tx).await {
            eprintln!("Error: {}", e);
        }
    });

    let mut current_node: Option<SceneNode> = None;
    window.set_light(kiss3d::light::Light::StickToCamera);

    while window.render_with_camera(&mut camera) {
        window.draw_axes();

        while let Ok(raw_options) = rx.try_recv() {
            println!("Number of raw options: {}", raw_options.len());

            let deribit_points: Vec<DeribitDataPoint> = raw_options
                .into_iter()
                .filter_map(|data_point| data_point.into_full())
                .map(|option| option.into_data_point())
                .collect();

            let mesh = construct_mesh(deribit_points);

            if let Some(mut old_node) = current_node.take() {
                window.remove_node(&mut old_node);
            }

            let mut mesh_node =
                window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));

            mesh_node.recompute_normals();
            mesh_node.set_color(0.4, 0.7, 1.0);

            current_node = Some(mesh_node);
        }
    }

    Ok(())
}

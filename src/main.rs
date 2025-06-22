mod plot;
mod render;
mod types;
mod utils;
mod websocket;

use anyhow::Result;
use clap::Parser;
use kiss3d::{nalgebra::Vector3, scene::SceneNode, window::Window};
use std::{cell::RefCell, rc::Rc, sync::mpsc};

use crate::plot::State;
use crate::render::Render;
use crate::types::{Args, DeribitDataPoint, OptionSide, RawDeribitOption};
use crate::websocket::listen_for_deribit_data;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let (mut window, mut camera) = Window::window_and_camera_set_up();

    let (tx, rx) = mpsc::channel::<Vec<RawDeribitOption>>();
    tokio::spawn(async move {
        if let Err(e) = listen_for_deribit_data(tx).await {
            eprintln!("Error: {}", e);
        }
    });

    let mut current_call_node: Option<SceneNode> = None;
    let mut current_put_node: Option<SceneNode> = None;
    window.set_light(kiss3d::light::Light::StickToCamera);

    let mut call_state = State::new();
    let mut put_state = State::new();

    while window.render_with_camera(&mut camera) {
        window.draw_axes();

        while let Ok(raw_options) = rx.try_recv() {
            println!("Number of raw options: {}", raw_options.len());

            // Process calls
            if args.calls {
                let call_points: Vec<DeribitDataPoint> = raw_options
                    .clone()
                    .into_iter()
                    .filter_map(|data_point| data_point.into_full())
                    .filter(|data_point| data_point.side == OptionSide::Call)
                    .map(|option| option.into_data_point())
                    .collect();

                call_state.update_state(call_points);
                let call_mesh = call_state.construct_mesh();

                if let Some(mut old_call_node) = current_call_node.take() {
                    window.remove_node(&mut old_call_node);
                }

                let mut call_mesh_node = window.add_mesh(
                    Rc::new(RefCell::new(call_mesh)),
                    Vector3::new(1.0, 1.0, 1.0),
                );

                // Set call surface colour here
                call_mesh_node.recompute_normals();
                call_mesh_node.set_color(0.0, 0.9, 1.0);

                current_call_node = Some(call_mesh_node);
            }

            // Process puts
            if args.puts {
                let put_points: Vec<DeribitDataPoint> = raw_options
                    .clone()
                    .into_iter()
                    .filter_map(|data_point| data_point.into_full())
                    .filter(|data_point| data_point.side == OptionSide::Put)
                    .map(|option| option.into_data_point())
                    .collect();

                put_state.update_state(put_points);

                let put_mesh = put_state.construct_mesh();

                if let Some(mut old_put_node) = current_put_node.take() {
                    window.remove_node(&mut old_put_node);
                }

                let mut put_mesh_node =
                    window.add_mesh(Rc::new(RefCell::new(put_mesh)), Vector3::new(1.0, 1.0, 1.0));

                // Set put surface colour here
                put_mesh_node.recompute_normals();
                put_mesh_node.set_color(1.0, 0.2, 0.4);

                current_put_node = Some(put_mesh_node);
            }
        }
    }

    Ok(())
}

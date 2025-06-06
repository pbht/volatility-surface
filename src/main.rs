mod types;
mod utils;

use anyhow::Result;
use ordered_float::OrderedFloat;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use kiss3d::nalgebra::{Point3, Vector3};
use kiss3d::resource::Mesh;
use kiss3d::window::{CanvasSetup, Window};

use crate::types::{DeribitDataPoint, DeribitWebSocketMessage};

fn main() -> Result<()> {
    // Business logic

    let raw_options = serde_json::from_str::<DeribitWebSocketMessage>(include_str!(
        "../resources/test_data.json"
    ))?;

    let deribit_points: Vec<DeribitDataPoint> = raw_options
        .params
        .data
        .into_iter()
        .map(|point| point.into_full().unwrap().into_data_point())
        .collect();

    let max_x = deribit_points
        .iter()
        .map(|p| OrderedFloat(p.x as f32))
        .max()
        .unwrap()
        .0;
    let max_y = deribit_points
        .iter()
        .map(|p| OrderedFloat(p.y as f32))
        .max()
        .unwrap()
        .0;
    let max_z = deribit_points
        .iter()
        .map(|p| OrderedFloat(p.z))
        .max()
        .unwrap()
        .0;

    let mut point_map: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), f32> = HashMap::new();
    for point in &deribit_points {
        point_map.insert(
            (OrderedFloat(point.x as f32), OrderedFloat(point.y as f32)),
            point.z,
        );
    }

    let delaunay_points: Vec<delaunator::Point> = point_map
        .keys()
        .map(|(x, y)| delaunator::Point {
            x: x.0 as f64,
            y: y.0 as f64,
        })
        .collect();

    let triangulation = delaunator::triangulate(&delaunay_points);

    let positions: Vec<Point3<f32>> = point_map
        .iter()
        .map(|((x, y), z)| Point3::new(x.0 / max_x, y.0 / max_y, *z / max_z))
        .collect();

    let indices: Vec<Point3<u16>> = triangulation
        .triangles
        .chunks(3)
        .map(|tri| Point3::new(tri[0] as u16, tri[1] as u16, tri[2] as u16))
        .collect();

    let setup = CanvasSetup {
        samples: kiss3d::window::NumSamples::Eight,
        vsync: true,
        // ..Default::default()
    };
    let mut window = Window::new_with_setup("Volatility Surface Mesh", 800, 600, setup);
    window.set_light(kiss3d::light::Light::StickToCamera);

    fn iv_to_colour(z: f32) -> Vector3<f32> {
        let norm = ((z - 0.3) / (0.7 - 0.3)).clamp(0.0, 1.0);
        Vector3::new(norm, 0.0, 1.0 - norm)
    }

    let vertex_colours: Vec<Vector3<f32>> = point_map.values().map(|z| iv_to_colour(*z)).collect();

    let mesh = Mesh::new(
        positions.clone(),    // Vec<Point3<f32>>
        indices.clone(),      // Vec<Point3<u16>>
        Some(vertex_colours), // Vec<Vector3<f32>>
        None,
        true,
    );

    let mut mesh_node = window.add_mesh(Rc::new(RefCell::new(mesh)), Vector3::new(1.0, 1.0, 1.0));
    mesh_node.recompute_normals();
    // mesh_node.set_color(0.3, 1.0, 1.0);

    while window.render() {
        let origin = Point3::origin();

        // Draw X axis in red (STRIKE)
        window.draw_line(
            &origin,
            &Point3::new(1.0, 0.0, 0.0),
            &Point3::new(1.0, 0.0, 0.0),
        );

        // Draw Y axis in green (EXPIRY)
        window.draw_line(
            &origin,
            &Point3::new(0.0, 1.0, 0.0),
            &Point3::new(0.0, 1.0, 0.0),
        );

        // Draw Z axis in blue (IV)
        window.draw_line(
            &origin,
            &Point3::new(0.0, 0.0, 1.0),
            &Point3::new(0.0, 0.0, 1.0),
        );

        // for (i, j) in &edges {
        //     let a = positions[*i];
        //     let b = positions[*j];
        //     window.draw_line(&a, &b, &Point3::new(1.0, 1.0, 1.0));
        // }
        mesh_node.recompute_normals();
    }

    Ok(())
}

use crate::types::DeribitDataPoint;
use kiss3d::nalgebra::Point3;
use kiss3d::resource::Mesh;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

pub fn construct_mesh(points: Vec<DeribitDataPoint>) -> Mesh {
    let (max_x, max_y, max_z) = points
        .iter()
        .fold((0.0f32, 0.0f32, 0.0f32), |(mx, my, mz), p| {
            (mx.max(p.x as f32), my.max(p.y as f32), mz.max(p.z))
        });

    let mut point_map: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), f32> = HashMap::new();
    for point in &points {
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
        .map(|triangle| Point3::new(triangle[0] as u16, triangle[1] as u16, triangle[2] as u16))
        .collect();

    Mesh::new(positions, indices, None, None, true)
}

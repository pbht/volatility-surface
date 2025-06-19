use crate::types::DeribitDataPoint;
use kiss3d::nalgebra::Point3;
use kiss3d::resource::Mesh;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    points: HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), f32>,
}

impl State {
    pub fn new() -> Self {
        State {
            points: HashMap::new(),
        }
    }

    pub fn update_state(&mut self, points: Vec<DeribitDataPoint>) {
        points.iter().for_each(|px| {
            self.points
                .insert((OrderedFloat(px.x as f32), OrderedFloat(px.y as f32)), px.z);
        });
    }

    pub fn construct_mesh(&self) -> Mesh {
        let (max_x, max_y, max_z) =
            self.points
                .iter()
                .fold((0.0f32, 0.0f32, 0.0f32), |(mx, my, mz), ((x, y), z)| {
                    (mx.max(x.into_inner()), my.max(y.into_inner()), mz.max(*z))
                });

        let point_map = &self.points;

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
}

mod types;
mod utils;

use anyhow::Result;

// Testing
use types::RawDeribitOption;

use crate::types::DeribitDataPoint;

fn main() -> Result<()> {
    // Define test data
    let raw_options = vec![
        RawDeribitOption {
            iv: 0.1,
            instrument_name: "BTC-05JUN25-120000-C".to_string(),
        },
        RawDeribitOption {
            iv: 0.3,
            instrument_name: "BTC-06JUN25-130000-C".to_string(),
        },
        RawDeribitOption {
            iv: 0.7,
            instrument_name: "BTC-07JUN25-100000-C".to_string(),
        },
        RawDeribitOption {
            iv: 0.7,
            instrument_name: "BTC-07JUN25-180000-C".to_string(),
        },
    ];

    // Business logic

    let deribit_points: Vec<DeribitDataPoint> = raw_options
        .into_iter()
        .map(|point| point.into_full().unwrap().into_data_point())
        .collect();

    let delaunay_points: Vec<delaunator::Point> = deribit_points
        .iter()
        .map(|point| delaunator::Point {
            x: point.x,
            y: point.y,
        })
        .collect();

    let triangulation = delaunator::triangulate(&delaunay_points);

    assert!(
        triangulation.triangles.len() % 3 == 0,
        "Triangulation is not valid"
    );
    println!("Triangulation: {:?}", triangulation.triangles);

    Ok(())
}

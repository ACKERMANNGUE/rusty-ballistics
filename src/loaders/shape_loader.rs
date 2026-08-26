use bevy::prelude::Vec2;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::geometry::shape::Shape;
use crate::geometry::polygon::compute_polygon_centroid;

type RawShapeLibrary = HashMap<String, Vec<[f32; 2]>>;

pub fn load_shapes(path: impl AsRef<Path>) -> Result<HashMap<String, Shape>, Box<dyn Error>> {
    let path = path.as_ref();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let raw_shapes: RawShapeLibrary = serde_json::from_reader(reader)?;

    let shapes = raw_shapes
        .into_iter()
        .map(|(name, points)| {
            let points: Vec<Vec2> = points.into_iter().map(Vec2::from_array).collect();

            let centroid = compute_polygon_centroid(&points);
            // ensure that the shape is centered around the origin by subtracting the centroid from each point
            let centered_points: Vec<Vec2> = points
                .into_iter()
                .map(|point| point - centroid)
                .collect();

            let shape = Shape::new(centered_points);

            (name, shape)
        })
        .collect();

    Ok(shapes)
}

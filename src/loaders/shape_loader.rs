use bevy::prelude::Vec2;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::geometry::shape::Shape;

type RawShapeLibrary = HashMap<String, Vec<[f32; 2]>>;

pub fn load_shapes(path: impl AsRef<Path>) -> Result<HashMap<String, Shape>, Box<dyn Error>> {
    let path = path.as_ref();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let raw_shapes: RawShapeLibrary = serde_json::from_reader(reader)?;

    let shapes = raw_shapes
        .into_iter()
        .map(|(name, points)| {
            let points = points.into_iter().map(Vec2::from_array).collect();
            let shape = Shape::new(points);
            (name, shape)
        })
        .collect();

    Ok(shapes)
}

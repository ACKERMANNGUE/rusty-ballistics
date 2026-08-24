use bevy::prelude::*;
use std::collections::HashMap;
use std::path::Path;

use crate::loaders::shape_loader::load_shapes;
use rand::seq::IteratorRandom;

#[derive(Resource)]
pub struct ShapeLibrary {
    shapes: HashMap<String, Vec<Vec2>>,
}

impl ShapeLibrary {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let shapes = load_shapes(path).unwrap_or_else(|error| {
            panic!("Failed to load shapes from '{}': {}", path.display(), error)
        });

        for (name, points) in &shapes {
            println!("Loaded shape '{}': {:?}", name, points);
        }

        Self { shapes }
    }

    pub fn get(&self, name: &str) -> Option<&Vec<Vec2>> {
        self.shapes.get(name)
    }

    pub fn get_random_shape_name(&self) -> Option<String> {
        let mut rng = rand::rng();

        self.shapes.keys().choose(&mut rng).cloned()
    }
}

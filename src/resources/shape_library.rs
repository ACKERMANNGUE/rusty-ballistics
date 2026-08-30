use bevy::prelude::*;
use std::collections::HashMap;
use std::path::Path;

use crate::geometry::shape::Shape;
use crate::loaders::shape_loader::load_shapes;
use rand::seq::IteratorRandom;

#[derive(Resource)]
pub struct ShapeLibrary {
    shapes: HashMap<String, Shape>,
}

impl ShapeLibrary {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let shapes = load_shapes(path).unwrap_or_else(|error| {
            panic!("Failed to load shapes from '{}': {}", path.display(), error)
        });

        for (name, shape) in &shapes {
            println!("Loaded shape '{}': {}", name, shape.get_vertices().len());
            println!("Triangles: {:?}", shape.get_triangles());
            println!("Is convex: {}", shape.is_convex());
            println!("Inertia factor: {}", shape.get_inertia_factor());
            println!("-----------------------------");
        }

        Self { shapes }
    }

    pub fn get(&self, name: &str) -> Option<&Shape> {
        self.shapes.get(name)
    }

    pub fn get_random_shape_name(&self) -> Option<String> {
        let mut rng = rand::rng();

        self.shapes.keys().choose(&mut rng).cloned()
    }

    pub fn get_shape_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.shapes.keys().cloned().collect();
        names.sort();

        names
    }
}

use bevy::prelude::*;

use crate::geometry::polygon::{validate_polygon, normalize_polygon_winding, is_polygon_convex, triangulate_ear_clipping};

pub struct Shape {
    vertices: Vec<Vec2>,
    triangles: Vec<[usize; 3]>,
    is_convex: bool,
}

impl Shape {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        let mut vertices = vertices;

        if !validate_polygon(&vertices) {
            panic!("Invalid polygon vertices.");
        }

        normalize_polygon_winding(&mut vertices);
        let is_convex = is_polygon_convex(&vertices);
        let triangles = triangulate_ear_clipping(&vertices);

        Self {
            vertices,
            triangles,
            is_convex,
        }
    }
    
    pub fn get_vertices(&self) -> &Vec<Vec2> {
        &self.vertices
    }

    pub fn get_triangles(&self) -> &Vec<[usize; 3]> {
        &self.triangles
    }

    pub fn is_convex(&self) -> bool {
        self.is_convex
    }
}
use bevy::prelude::*;

use crate::geometry::polygon::{validate_polygon, normalize_polygon_winding, is_polygon_convex, triangulate_ear_clipping};

use crate::geometry::moment_of_inertia::compute_polygon_inertia_factor;

pub struct Shape {
    vertices: Vec<Vec2>,
    triangles: Vec<[usize; 3]>,
    is_convex: bool,
    inertia_factor: f32,
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
        let inertia_factor = compute_polygon_inertia_factor(&vertices);

        Self {
            vertices,
            triangles,
            is_convex,
            inertia_factor,
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

    pub fn get_inertia_factor(&self) -> f32 {
        self.inertia_factor
    }
}
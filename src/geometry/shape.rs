use bevy::prelude::*;

use crate::geometry::moment_of_inertia::compute_polygon_inertia_factor;
use crate::geometry::polygon::{
    compute_polygon_area, is_polygon_convex, normalize_polygon_winding, triangulate_ear_clipping,
    validate_polygon,
};
use crate::geometry::shape_triangle::{ShapeTriangle, build_shape_triangles};

pub struct Shape {
    vertices: Vec<Vec2>,
    triangles: Vec<ShapeTriangle>,
    is_convex: bool,
    inertia_factor: f32,
    area: f32,
}

impl Shape {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        let mut vertices = vertices;

        if !validate_polygon(&vertices) {
            panic!("Invalid polygon vertices.");
        }

        normalize_polygon_winding(&mut vertices);

        let is_convex = is_polygon_convex(&vertices);

        let triangle_indices = triangulate_ear_clipping(&vertices);
        let triangles = build_shape_triangles(triangle_indices, vertices.len());

        let inertia_factor = compute_polygon_inertia_factor(&vertices);
        let area = compute_polygon_area(&vertices);

        Self {
            vertices,
            triangles,
            is_convex,
            inertia_factor,
            area,
        }
    }

    pub fn get_vertices(&self) -> &[Vec2] {
        &self.vertices
    }

    pub fn get_triangles(&self) -> &[ShapeTriangle] {
        &self.triangles
    }

    pub fn get_area(&self) -> f32 {
        self.area
    }

    pub fn is_convex(&self) -> bool {
        self.is_convex
    }

    pub fn get_inertia_factor(&self) -> f32 {
        self.inertia_factor
    }
}

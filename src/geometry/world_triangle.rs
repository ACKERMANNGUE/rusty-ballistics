use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct WorldTriangle {
    vertices: [Vec2; 3],
    boundary_edges: [bool; 3],
}

impl WorldTriangle {
    pub fn new(vertices: [Vec2; 3], boundary_edges: [bool; 3]) -> Self {
        Self {
            vertices,
            boundary_edges,
        }
    }

    pub fn get_vertices(&self) -> &[Vec2; 3] {
        &self.vertices
    }

    pub fn get_boundary_edges(&self) -> &[bool; 3] {
        &self.boundary_edges
    }

    pub fn is_boundary_edge(&self, edge_index: usize) -> bool {
        self.boundary_edges[edge_index]
    }
}

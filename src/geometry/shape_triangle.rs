use crate::geometry::polygon::is_polygon_boundary_edge;

#[derive(Debug, Clone, Copy)]
pub struct ShapeTriangle {
    indices: [usize; 3],
    boundary_edges: [bool; 3],
}

impl ShapeTriangle {
    pub fn new(indices: [usize; 3], boundary_edges: [bool; 3]) -> Self {
        Self {
            indices,
            boundary_edges,
        }
    }

    pub fn get_indices(&self) -> &[usize; 3] {
        &self.indices
    }

    pub fn get_boundary_edges(&self) -> &[bool; 3] {
        &self.boundary_edges
    }
}

pub fn build_shape_triangles(
    triangle_indices: Vec<[usize; 3]>,
    vertex_count: usize
) -> Vec<ShapeTriangle> {
    triangle_indices
        .into_iter()
        .map(|indices| {
            let boundary_edges = [
                is_polygon_boundary_edge(indices[0], indices[1], vertex_count),
                is_polygon_boundary_edge(indices[1], indices[2], vertex_count),
                is_polygon_boundary_edge(indices[2], indices[0], vertex_count),
            ];

            ShapeTriangle::new(indices, boundary_edges)
        })
        .collect()
}

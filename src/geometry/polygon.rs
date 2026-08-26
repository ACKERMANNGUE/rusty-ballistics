use std::f32::EPSILON;

use bevy::prelude::*;

pub fn signed_area(vertices: &[Vec2]) -> f32 {
    let n = vertices.len();
    let mut area = 0.0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += vertices[i].x * vertices[j].y;
        area -= vertices[j].x * vertices[i].y;
    }

    area / 2.0
}

pub fn normalize_polygon_winding(vertices: &mut Vec<Vec2>) {
    if signed_area(vertices) < 0.0 {
        vertices.reverse();
    }
}

pub fn cross_2d(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

pub fn is_convex_vertex(prev: Vec2, curr: Vec2, next: Vec2) -> bool {
    let edge1 = curr - prev;
    let edge2 = next - curr;
    cross_2d(edge1, edge2) > 0.0
}

pub fn is_polygon_convex(vertices: &[Vec2]) -> bool {
    let mut is_convex = true;

    for i in 0..vertices.len() {
        let prev = vertices[(i + vertices.len() - 1) % vertices.len()];
        let curr = vertices[i];
        let next = vertices[(i + 1) % vertices.len()];

        if !is_convex_vertex(prev, curr, next) {
            // means that the angle is concave
            is_convex = false;
            break;
        }
    }

    is_convex
}

pub fn point_in_triangle(point: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let side_1 = cross_2d(b - a, point - a);
    let side_2 = cross_2d(c - b, point - b);
    let side_3 = cross_2d(a - c, point - c);

    side_1 >= -EPSILON && side_2 >= -EPSILON && side_3 >= -EPSILON
}

pub fn is_ear(vertices: &[Vec2], remaining_indices: &[usize], current_index: usize) -> bool {
    let prev_index =
        remaining_indices[(current_index + remaining_indices.len() - 1) % remaining_indices.len()];
    let curr_index = remaining_indices[current_index];
    let next_index = remaining_indices[(current_index + 1) % remaining_indices.len()];

    let a = vertices[prev_index];
    let b = vertices[curr_index];
    let c = vertices[next_index];

    if !is_convex_vertex(a, b, c) {
        return false;
    }

    for &i in remaining_indices {
        if i == prev_index || i == curr_index || i == next_index {
            continue;
        }
        if point_in_triangle(vertices[i], a, b, c) {
            return false;
        }
    }

    true
}

pub fn triangulate_ear_clipping(vertices: &[Vec2]) -> Vec<[usize; 3]> {
    let mut triangles = Vec::new();
    let mut remaining_indices: Vec<usize> = (0..vertices.len()).collect();

    while remaining_indices.len() > 3 {
        let mut ear_found = false;

        for i in 0..remaining_indices.len() {
            if is_ear(vertices, &remaining_indices, i) {
                let prev_index =
                    remaining_indices[(i + remaining_indices.len() - 1) % remaining_indices.len()];
                let curr_index = remaining_indices[i];
                let next_index = remaining_indices[(i + 1) % remaining_indices.len()];

                triangles.push([prev_index, curr_index, next_index]);
                remaining_indices.remove(i);
                ear_found = true;
                break;
            }
        }

        if !ear_found {
            panic!("No ear found. The polygon might be malformed.");
        }
    }

    triangles.push([remaining_indices[0], remaining_indices[1], remaining_indices[2]]);

    triangles
}

pub fn validate_polygon(vertices: &[Vec2]) -> bool {
    if vertices.len() < 3 {
        return false;
    }

    let area = signed_area(vertices);
    if area.abs() < EPSILON {
        println!("Polygon has zero area.");
        return false;
    }

    true
}

pub fn compute_polygon_centroid(vertices: &[Vec2]) -> Vec2 {
    if vertices.len() < 3 {
        return Vec2::ZERO;
    }

    let mut centroid = Vec2::ZERO;
    let mut signed_area = 0.0;

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();

        let a = vertices[i];
        let b = vertices[j];

        let cross = cross_2d(a, b);

        signed_area += cross;
        centroid += (a + b) * cross;
    }

    signed_area *= 0.5;

    if signed_area.abs() <= f32::EPSILON {
        return Vec2::ZERO;
    }

    centroid /= 6.0 * signed_area;

    centroid
}

pub fn compute_polygon_area(vertices: &[Vec2]) -> f32 {
    if vertices.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += cross_2d(vertices[i], vertices[j]);
    }

    area.abs() * 0.5
}
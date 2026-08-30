use bevy::prelude::Vec2;

use crate::config::EPSILON;
use crate::geometry::vector::cross_2d;

pub fn compute_polygon_inertia_factor(vertices: &[Vec2]) -> f32 {
    if vertices.len() < 3 {
        return 0.0;
    }

    let mut cross_sum = 0.0;
    let mut inertia_sum = 0.0;

    for i in 0..vertices.len() {
        let current = vertices[i];
        let next = vertices[(i + 1) % vertices.len()];

        let cross = cross_2d(current, next);

        let quadratic_term = current.length_squared() + current.dot(next) + next.length_squared();

        cross_sum += cross;
        inertia_sum += cross * quadratic_term;
    }

    if cross_sum.abs() <= EPSILON {
        return 0.0;
    }

    let inertia_factor = inertia_sum / (6.0 * cross_sum);

    inertia_factor.abs()
}

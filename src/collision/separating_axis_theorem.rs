use bevy::prelude::*;

pub fn check_polygon_collision(polygon1: &[Vec2], polygon2: &[Vec2]) -> bool {
    if polygon1.len() < 3 || polygon2.len() < 3 {
        return false;
    }

    let axes_a = get_polygon_axes(polygon1);
    let axes_b = get_polygon_axes(polygon2);

    for axis in axes_a.iter().chain(axes_b.iter()) {
        let (min_a, max_a) = project_polygon(*axis, polygon1);
        let (min_b, max_b) = project_polygon(*axis, polygon2);

        if max_a < min_b || max_b < min_a {
            return false;
        }
    }

    true
}

fn get_polygon_axes(polygon: &[Vec2]) -> Vec<Vec2> {
    let mut axes = Vec::with_capacity(polygon.len());

    for i in 0..polygon.len() {
        let current_point = polygon[i];
        let next_point = polygon[(i + 1) % polygon.len()]; // ensure we loop back to the first point

        let edge = next_point - current_point;

        if edge.length_squared() == 0.0 {
            continue;
        }

        let normal = Vec2::new(-edge.y, edge.x).normalize();
        axes.push(normal);
    }

    axes
}

fn project_polygon(axis: Vec2, polygon: &[Vec2]) -> (f32, f32) {
    let first_projection = axis.dot(polygon[0]);

    let mut min = first_projection;
    let mut max = first_projection;

    for point in polygon.iter().skip(1) {
        let projection = axis.dot(*point);

        if projection < min {
            min = projection;
        }

        if projection > max {
            max = projection;
        }
    }

    (min, max)
}

use bevy::prelude::*;
use crate::{ collision::collision_info::CollisionInfo, geometry::projection::project_polygon };

pub fn check_polygon_collision(polygon1: &[Vec2], polygon2: &[Vec2]) -> Option<CollisionInfo> {
    if polygon1.len() < 3 || polygon2.len() < 3 {
        return None;
    }

    let axes_a = get_polygon_axes(polygon1);
    let axes_b = get_polygon_axes(polygon2);

    let mut minimum_overlap = f32::MAX;
    let mut best_axis = Vec2::ZERO;

    for axis in axes_a.iter().chain(axes_b.iter()) {
        let (min_a, max_a) = project_polygon(*axis, polygon1);
        let (min_b, max_b) = project_polygon(*axis, polygon2);

        if max_a < min_b || max_b < min_a {
            return None;
        }

        let overlap = max_a.min(max_b) - min_a.max(min_b);

        if overlap <= 0.0 {
            return None;
        }

        if overlap < minimum_overlap {
            minimum_overlap = overlap;
            best_axis = *axis;
        }
    }

    let center_a = polygon1.iter().copied().sum::<Vec2>() / (polygon1.len() as f32);
    let center_b = polygon2.iter().copied().sum::<Vec2>() / (polygon2.len() as f32);
    let direction_a_to_b = center_b - center_a;

    if direction_a_to_b.dot(best_axis) < 0.0 {
        best_axis = -best_axis;
    }

    let support_point_a = get_support_point(polygon1, best_axis);
    let support_point_b = get_support_point(polygon2, -best_axis);

    let contact_point = (support_point_a + support_point_b) / 2.0;

    Some(CollisionInfo::new(best_axis, minimum_overlap, contact_point))
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

pub fn check_triangles_collision(
    triangles_a: &[[Vec2; 3]],
    triangles_b: &[[Vec2; 3]]
) -> Option<CollisionInfo> {
    let mut best_collision: Option<CollisionInfo> = None;

    for triangle_a in triangles_a {
        for triangle_b in triangles_b {
            let Some(collision_info) = check_polygon_collision(triangle_a, triangle_b) else {
                continue;
            };

            let should_replace = match &best_collision {
                Some(current_collision) => {
                    collision_info.get_penetration_depth() <
                        current_collision.get_penetration_depth()
                }
                None => true,
            };

            if should_replace {
                best_collision = Some(collision_info);
            }
        }
    }

    best_collision
}

fn get_support_point(polygon: &[Vec2], direction: Vec2) -> Vec2 {
    const SUPPORT_EPSILON: f32 = 0.0001;

    let max_projection = polygon
        .iter()
        .map(|point| point.dot(direction))
        .fold(f32::NEG_INFINITY, f32::max);

    let mut support_sum = Vec2::ZERO;
    let mut support_count = 0;

    for &point in polygon {
        let projection = point.dot(direction);

        if (max_projection - projection).abs() <= SUPPORT_EPSILON {
            support_sum += point;
            support_count += 1;
        }
    }

    support_sum / (support_count as f32)
}

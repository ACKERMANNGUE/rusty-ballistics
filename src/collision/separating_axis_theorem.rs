use bevy::prelude::*;

use crate::{
    collision::collision_info::CollisionInfo,
    config::EPSILON,
    geometry::{ polygon::compute_polygon_centroid, projection::project_polygon },
};

use crate::collision::contact_manifold::{ build_contact_manifold, ContactManifold };

#[derive(Clone, Copy, Debug)]
pub(crate) enum ReferencePolygon {
    Polygon1,
    Polygon2,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SATResult {
    pub(crate) normal: Vec2,
    pub(crate) penetration_depth: f32,
    pub(crate) reference_polygon: ReferencePolygon,
    pub(crate) reference_edge_index: usize,
}

#[derive(Clone, Copy, Debug)]
struct CollisionAxis {
    normal: Vec2,
}

pub fn check_polygon_collision(polygon1: &[Vec2], polygon2: &[Vec2]) -> Option<CollisionInfo> {
    let sat_result = compute_sat_result(polygon1, polygon2)?;

    let SATResult {
        normal,
        penetration_depth,
        reference_polygon: _reference_polygon,
        reference_edge_index: _reference_edge_index,
    } = sat_result;

    let support_point_a = get_support_point(polygon1, normal);
    let support_point_b = get_support_point(polygon2, -normal);

    let contact_point = (support_point_a + support_point_b) * 0.5;

    Some(CollisionInfo::new(normal, penetration_depth, contact_point))
}

pub(crate) fn compute_sat_result(polygon1: &[Vec2], polygon2: &[Vec2]) -> Option<SATResult> {
    if polygon1.len() < 3 || polygon2.len() < 3 {
        return None;
    }

    let axes_1 = get_polygon_axes(polygon1);
    let axes_2 = get_polygon_axes(polygon2);

    let mut minimum_overlap = f32::MAX;

    let mut best_axis = Vec2::ZERO;
    let mut reference_polygon = ReferencePolygon::Polygon1;

    for axis in &axes_1 {
        let (min_1, max_1) = project_polygon(axis.normal, polygon1);
        let (min_2, max_2) = project_polygon(axis.normal, polygon2);

        if max_1 < min_2 || max_2 < min_1 {
            return None;
        }

        let overlap = max_1.min(max_2) - min_1.max(min_2);

        if overlap <= 0.0 {
            return None;
        }

        if overlap < minimum_overlap {
            minimum_overlap = overlap;
            best_axis = axis.normal;
            reference_polygon = ReferencePolygon::Polygon1;
        }
    }

    for axis in &axes_2 {
        let (min_1, max_1) = project_polygon(axis.normal, polygon1);
        let (min_2, max_2) = project_polygon(axis.normal, polygon2);

        if max_1 < min_2 || max_2 < min_1 {
            return None;
        }

        let overlap = max_1.min(max_2) - min_1.max(min_2);

        if overlap <= 0.0 {
            return None;
        }

        if overlap < minimum_overlap {
            minimum_overlap = overlap;
            best_axis = axis.normal;
            reference_polygon = ReferencePolygon::Polygon2;
        }
    }

    if best_axis == Vec2::ZERO {
        return None;
    }

    let center_1 = compute_polygon_centroid(polygon1);
    let center_2 = compute_polygon_centroid(polygon2);

    let direction_1_to_2 = center_2 - center_1;

    if direction_1_to_2.dot(best_axis) < 0.0 {
        best_axis = -best_axis;
    }

    let reference_edge_index = match reference_polygon {
        ReferencePolygon::Polygon1 => find_reference_edge_index(polygon1, best_axis)?,
        ReferencePolygon::Polygon2 => find_reference_edge_index(polygon2, best_axis)?,
    };

    Some(SATResult {
        normal: best_axis,
        penetration_depth: minimum_overlap,
        reference_polygon,
        reference_edge_index,
    })
}

fn get_polygon_axes(polygon: &[Vec2]) -> Vec<CollisionAxis> {
    let mut axes = Vec::with_capacity(polygon.len());
    let polygon_center = compute_polygon_centroid(polygon);

    for i in 0..polygon.len() {
        let current_point = polygon[i];
        let next_point = polygon[(i + 1) % polygon.len()];

        let edge_center = (current_point + next_point) * 0.5;
        let mut normal = Vec2::new(
            -(next_point.y - current_point.y),
            next_point.x - current_point.x
        ).normalize();
        let direction_to_center = polygon_center - edge_center;
        if normal.dot(direction_to_center) < 0.0 {
            normal = -normal;
        }

        let edge = next_point - current_point;
        if edge.length_squared() <= EPSILON {
            continue;
        }

        let normal = Vec2::new(-edge.y, edge.x).normalize();
        axes.push(CollisionAxis {
            normal,
        });
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

pub fn check_triangles_manifold(
    triangles_a: &[[Vec2; 3]],
    triangles_b: &[[Vec2; 3]]
) -> Option<ContactManifold> {
    let mut best_manifold: Option<ContactManifold> = None;

    for triangle_a in triangles_a {
        for triangle_b in triangles_b {
            let Some(manifold) = check_polygon_manifold(triangle_a, triangle_b) else {
                continue;
            };

            let should_replace = match &best_manifold {
                Some(current_manifold) => {
                    manifold.get_penetration_depth() < current_manifold.get_penetration_depth()
                }
                None => true,
            };

            if should_replace {
                best_manifold = Some(manifold);
            }
        }
    }

    best_manifold
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

fn find_reference_edge_index(polygon: &[Vec2], reference_normal: Vec2) -> Option<usize> {
    if polygon.len() < 2 {
        return None;
    }

    let polygon_center = compute_polygon_centroid(polygon);
    let mut best_dot = f32::NEG_INFINITY;
    let mut best_edge_index = None;

    for i in 0..polygon.len() {
        let current = polygon[i];
        let next = polygon[(i + 1) % polygon.len()];

        let edge = next - current;

        if edge.length_squared() <= f32::EPSILON {
            continue;
        }

        let edge_center = (current + next) * 0.5;
        let direction_to_edge = edge_center - polygon_center;
        let mut outward_normal = Vec2::new(-edge.y, edge.x).normalize();

        if outward_normal.dot(direction_to_edge) < 0.0 {
            outward_normal = -outward_normal;
        }

        let dot = outward_normal.dot(reference_normal);

        if dot > best_dot {
            best_dot = dot;
            best_edge_index = Some(i);
        }
    }

    best_edge_index
}

pub fn check_polygon_manifold(polygon1: &[Vec2], polygon2: &[Vec2]) -> Option<ContactManifold> {
    let sat_result = compute_sat_result(polygon1, polygon2)?;

    match sat_result.reference_polygon {
        ReferencePolygon::Polygon1 => {
            build_contact_manifold(
                polygon1,
                polygon2,
                sat_result.normal,
                sat_result.normal,
                sat_result.penetration_depth,
                sat_result.reference_edge_index
            )
        }

        ReferencePolygon::Polygon2 => {
            build_contact_manifold(
                polygon2,
                polygon1,
                -sat_result.normal,
                sat_result.normal,
                sat_result.penetration_depth,
                sat_result.reference_edge_index
            )
        }
    }
}

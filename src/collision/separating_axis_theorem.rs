use bevy::prelude::*;

use crate::{
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

        let Some(penetration) = compute_interval_penetration(min_1, max_1, min_2, max_2) else {
            return None;
        };

        if penetration < minimum_overlap {
            minimum_overlap = penetration;
            best_axis = axis.normal;
            reference_polygon = ReferencePolygon::Polygon1;
        }
    }

    for axis in &axes_2 {
        let (min_1, max_1) = project_polygon(axis.normal, polygon1);
        let (min_2, max_2) = project_polygon(axis.normal, polygon2);

        let Some(penetration) = compute_interval_penetration(min_1, max_1, min_2, max_2) else {
            return None;
        };

        if penetration < minimum_overlap {
            minimum_overlap = penetration;
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
        ReferencePolygon::Polygon2 => find_reference_edge_index(polygon2, -best_axis)?,
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

    for i in 0..polygon.len() {
        let current_point = polygon[i];
        let next_point = polygon[(i + 1) % polygon.len()];

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

fn compute_interval_penetration(min_1: f32, max_1: f32, min_2: f32, max_2: f32) -> Option<f32> {
    let penetration_1_to_2 = max_1 - min_2;
    let penetration_2_to_1 = max_2 - min_1;

    if penetration_1_to_2 <= 0.0 || penetration_2_to_1 <= 0.0 {
        return None;
    }

    Some(penetration_1_to_2.min(penetration_2_to_1))
}

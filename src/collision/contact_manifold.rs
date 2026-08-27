use bevy::prelude::Vec2;

use crate::{config::EPSILON, geometry::polygon::compute_polygon_centroid};

#[derive(Debug, Clone)]
pub struct ContactManifold {
    normal: Vec2,
    penetration_depth: f32,
    contacts: Vec<Vec2>,
}

impl ContactManifold {
    pub fn new(normal: Vec2, penetration_depth: f32, contacts: Vec<Vec2>) -> Self {
        Self {
            normal,
            penetration_depth,
            contacts,
        }
    }

    pub fn get_normal(&self) -> Vec2 {
        self.normal
    }

    pub fn get_penetration_depth(&self) -> f32 {
        self.penetration_depth
    }

    pub fn get_contacts(&self) -> &[Vec2] {
        &self.contacts
    }
}

pub(crate) fn find_incident_edge(polygon: &[Vec2], reference_normal: Vec2) -> Option<[Vec2; 2]> {
    if polygon.len() < 2 {
        return None;
    }

    let polygon_center = compute_polygon_centroid(polygon);

    let mut best_dot = f32::INFINITY;
    let mut best_edge: Option<[Vec2; 2]> = None;

    for i in 0..polygon.len() {
        let current = polygon[i];
        let next = polygon[(i + 1) % polygon.len()];

        let edge = next - current;

        if edge.length_squared() <= f32::EPSILON {
            continue;
        }

        let edge_center = (current + next) * 0.5;
        let direction_to_edge = edge_center - polygon_center;

        let mut edge_normal = Vec2::new(-edge.y, edge.x).normalize();

        // ensures that the edge normal is pointing towards the polygon center
        if edge_normal.dot(direction_to_edge) < 0.0 {
            edge_normal = -edge_normal;
        }

        let dot = edge_normal.dot(reference_normal);

        // the incident edge is the one whose normal is most opposite to the reference normal
        // so incident_normal dot product reference_normal = -1
        if dot < best_dot {
            best_dot = dot;
            best_edge = Some([current, next]);
        }
    }

    best_edge
}

fn clip_segment_to_line(points: [Vec2; 2], normal: Vec2, offset: f32) -> Vec<Vec2> {
    let mut clipped_points = Vec::new();

    let distances = [points[0].dot(normal) - offset, points[1].dot(normal) - offset];

    if distances[0] <= 0.0 {
        clipped_points.push(points[0]);
    }
    if distances[1] <= 0.0 {
        clipped_points.push(points[1]);
    }

    if distances[0] * distances[1] < 0.0 {
        let interpolation = distances[0] / (distances[0] - distances[1]);
        let intersection_point = points[0] + interpolation * (points[1] - points[0]);
        clipped_points.push(intersection_point);
    }

    clipped_points
}

pub(crate) fn build_contact_manifold(
    reference_polygon: &[Vec2],
    incident_polygon: &[Vec2],
    reference_normal: Vec2,
    collision_normal: Vec2,
    penetration_depth: f32,
    reference_edge_index: usize
) -> Option<ContactManifold> {
    if reference_polygon.len() < 2 || incident_polygon.len() < 2 {

        return None;
    }

    let reference_start = reference_polygon[reference_edge_index];
    let reference_end = reference_polygon[(reference_edge_index + 1) % reference_polygon.len()];
    let reference_edge = reference_end - reference_start;

    if reference_edge.length_squared() <= EPSILON {
        return None;
    }

    let reference_tangent = reference_edge.normalize();

    let incident_edge = find_incident_edge(incident_polygon, reference_normal)?;

    // first side plane normal is opposite to the reference tangent
    let first_clip_normal = -reference_tangent;
    let first_clip_offset = first_clip_normal.dot(reference_start);
    let first_clipped = clip_segment_to_line(incident_edge, first_clip_normal, first_clip_offset);

    if first_clipped.is_empty() {
        return None;
    }

    // second side plane normal is the reference tangent
    let second_clip_normal = reference_tangent;
    let second_clip_offset = second_clip_normal.dot(reference_end);
    let second_clipped = if first_clipped.len() == 2 {
        clip_segment_to_line(
            [first_clipped[0], first_clipped[1]],
            second_clip_normal,
            second_clip_offset
        )
    } else {
        let point = first_clipped[0];
        let distance = point.dot(second_clip_normal) - second_clip_offset;

        if distance <= EPSILON {
            vec![point]
        } else {
            // if the point is outside the second clipping plane, we discard it
            Vec::new()
        }
    };

    if second_clipped.is_empty() {
        return None;
    }

    let mut contacts = Vec::with_capacity(2);

    for point in second_clipped {
        let separation = reference_normal.dot(point - reference_start);

        if separation <= EPSILON {
            // this is done to reduce jittering when the contact point is very close to the reference plane
             
            let contact = point - reference_normal * separation * 0.5;

            let already_exists = contacts
                .iter()
                .any(|existing: &Vec2| {
                    existing.distance_squared(contact) <= EPSILON * EPSILON
                });

            if !already_exists {
                contacts.push(contact);
            }
        }
    }

    if contacts.is_empty() {
        return None;
    }

    Some(ContactManifold::new(collision_normal, penetration_depth, contacts))
}

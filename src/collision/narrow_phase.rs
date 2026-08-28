use crate::{
    collision::{
        contact_manifold::ContactManifold,
        separating_axis_theorem::{ check_polygon_manifold, check_triangles_manifolds },
    },
    geometry::bullet_shape::{ get_bullet_world_shape, get_bullet_world_triangles },
    models::bullet::Bullet,
    resources::shape_library::ShapeLibrary,
};

use bevy::prelude::Vec2;

struct ManifoldGroup {
    normal: Vec2,
    penetration_depth: f32,
    contacts: Vec<Vec2>,
}

const NORMAL_MERGE_DOT_THRESHOLD: f32 = 0.999; // cos(2.5 degrees) : considering two normals to be "similar enough" to merge their contact manifolds
const CONTACT_MERGE_DISTANCE: f32 = 0.01; // maximum distance between two contact points to consider them as "the same" for merging purposes
const CONTACT_DUPLICATE_EPSILON: f32 = 0.001; // maximum distance between two contact points to consider them as "the same" for reducing purposes

pub fn detect_collision_manifolds(
    bullet_a: &Bullet,
    bullet_b: &Bullet,
    shape_library: &ShapeLibrary
) -> Vec<ContactManifold> {
    let (Some(shape_a), Some(shape_b)) = (
        shape_library.get(bullet_a.get_shape()),
        shape_library.get(bullet_b.get_shape()),
    ) else {
        return Vec::new();
    };

    if shape_a.is_convex() && shape_b.is_convex() {
        let (Some(polygon_a), Some(polygon_b)) = (
            get_bullet_world_shape(bullet_a, shape_library),
            get_bullet_world_shape(bullet_b, shape_library),
        ) else {
            return Vec::new();
        };

        return check_polygon_manifold(&polygon_a, &polygon_b).into_iter().collect();
    }

    let (Some(triangles_a), Some(triangles_b)) = (
        get_bullet_world_triangles(bullet_a, shape_library),
        get_bullet_world_triangles(bullet_b, shape_library),
    ) else {
        return Vec::new();
    };

    let candidate_manifolds = check_triangles_manifolds(&triangles_a, &triangles_b);

    merge_contact_manifolds(candidate_manifolds)
}

fn manifolds_are_mergeable(group: &ManifoldGroup, manifold: &ContactManifold) -> bool {
    let normal_alignment = group.normal.dot(manifold.get_normal());

    if normal_alignment < NORMAL_MERGE_DOT_THRESHOLD {
        return false;
    }

    let maximum_distance_squared = CONTACT_MERGE_DISTANCE * CONTACT_MERGE_DISTANCE;

    group.contacts.iter().any(|group_contact| {
        manifold
            .get_contacts()
            .iter()
            .any(|manifold_contact| {
                group_contact.distance_squared(*manifold_contact) <= maximum_distance_squared
            })
    })
}

fn add_unique_contact(contacts: &mut Vec<Vec2>, contact: Vec2) {
    let already_exists = contacts
        .iter()
        .any(|existing| {
            existing.distance_squared(contact) <=
                CONTACT_DUPLICATE_EPSILON * CONTACT_DUPLICATE_EPSILON
        });

    if !already_exists {
        contacts.push(contact);
    }
}

fn reduce_contacts(contacts: Vec<Vec2>, normal: Vec2) -> Vec<Vec2> {
    if contacts.len() <= 2 {
        return contacts;
    }

    let tangent = Vec2::new(-normal.y, normal.x);

    let mut minimum_contact = contacts[0];
    let mut maximum_contact = contacts[0];
    let mut minimum_projection = contacts[0].dot(tangent);
    let mut maximum_projection = minimum_projection;

    // skipping the first contact since we already initialized the min/max with it
    for &contact in contacts.iter().skip(1) {
        let projection = contact.dot(tangent);

        if projection < minimum_projection {
            minimum_projection = projection;
            minimum_contact = contact;
        }

        if projection > maximum_projection {
            maximum_projection = projection;
            maximum_contact = contact;
        }
    }

    // If the minimum and maximum contacts are very close, we only need one.
    if
        minimum_contact.distance_squared(maximum_contact) <=
        CONTACT_DUPLICATE_EPSILON * CONTACT_DUPLICATE_EPSILON
    {
        vec![minimum_contact]
    } else {
        vec![minimum_contact, maximum_contact]
    }
}

fn merge_contact_manifolds(manifolds: Vec<ContactManifold>) -> Vec<ContactManifold> {
    let mut groups: Vec<ManifoldGroup> = Vec::new();

    for manifold in manifolds {
        let matching_group = groups
            .iter()
            .position(|group| { manifolds_are_mergeable(group, &manifold) });

        if let Some(group_index) = matching_group {
            let group = &mut groups[group_index];
            group.penetration_depth = group.penetration_depth.max(manifold.get_penetration_depth());

            for &contact in manifold.get_contacts() {
                add_unique_contact(&mut group.contacts, contact);
            }
        } else {
            groups.push(ManifoldGroup {
                normal: manifold.get_normal(),
                penetration_depth: manifold.get_penetration_depth(),
                contacts: manifold.get_contacts().to_vec(),
            });
        }
    }

    groups
        .into_iter()
        .map(|group| {
            let contacts = reduce_contacts(group.contacts, group.normal);
            ContactManifold::new(group.normal, group.penetration_depth, contacts)
        })
        .collect()
}

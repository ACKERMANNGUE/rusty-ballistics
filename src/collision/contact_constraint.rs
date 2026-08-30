use bevy::prelude::Vec2;

use crate::{
    collision::contact_manifold::ContactManifold,
    config::{ EPSILON, RESTITUTION_VELOCITY_THRESHOLD },
    geometry::polygon::cross_2d,
    models::bullet::Bullet,
};

pub struct ContactConstraint {
    pub point: Vec2,
    pub normal: Vec2,
    pub tangent: Vec2,
    pub penetration_depth: f32,
    pub r1: Vec2,
    pub r2: Vec2,
    pub normal_mass: f32,
    pub tangent_mass: f32,
    pub accumulated_normal_impulse: f32,
    pub accumulated_tangent_impulse: f32,
    pub restitution_velocity: f32,
}

impl ContactConstraint {
    pub fn new(
        point: Vec2,
        normal: Vec2,
        penetration_depth: f32,
        r1: Vec2,
        r2: Vec2,
        normal_mass: f32,
        tangent_mass: f32,
        restitution_velocity: f32
    ) -> Self {
        let tangent = Vec2::new(-normal.y, normal.x);

        Self {
            point,
            normal,
            tangent,
            penetration_depth,
            r1,
            r2,
            normal_mass,
            tangent_mass,
            accumulated_normal_impulse: 0.0,
            accumulated_tangent_impulse: 0.0,
            restitution_velocity,
        }
    }
}

pub fn build_contact_constraints(
    bullet1: &Bullet,
    bullet2: &Bullet,
    manifolds: &[ContactManifold]
) -> Vec<ContactConstraint> {
    let mut contact_constraints = Vec::new();

    let inverse_mass1 = 1.0 / bullet1.get_mass();
    let inverse_mass2 = 1.0 / bullet2.get_mass();

    let sum_inverse_mass = inverse_mass1 + inverse_mass2;

    let inverse_inertia1 = 1.0 / bullet1.get_moment_of_inertia();
    let inverse_inertia2 = 1.0 / bullet2.get_moment_of_inertia();

    let position1 = bullet1.get_position();
    let position2 = bullet2.get_position();

    let restitution = bullet1.get_restitution().min(bullet2.get_restitution());

    for manifold in manifolds {
        let normal = manifold.get_normal();
        let tangent = Vec2::new(-normal.y, normal.x);

        for contact_point in manifold.get_contacts() {
            let r1 = contact_point - position1;
            let r2 = contact_point - position2;

            let r1_cross_normal = cross_2d(r1, normal);
            let r2_cross_normal = cross_2d(r2, normal);

            let normal_denominator =
                sum_inverse_mass +
                r1_cross_normal.powi(2) * inverse_inertia1 +
                r2_cross_normal.powi(2) * inverse_inertia2;

            let normal_mass = if normal_denominator > EPSILON {
                1.0 / normal_denominator
            } else {
                0.0
            };

            let r1_cross_tangent = cross_2d(r1, tangent);
            let r2_cross_tangent = cross_2d(r2, tangent);

            let tangent_denominator =
                sum_inverse_mass +
                r1_cross_tangent.powi(2) * inverse_inertia1 +
                r2_cross_tangent.powi(2) * inverse_inertia2;

            let tangent_mass = if tangent_denominator > EPSILON {
                1.0 / tangent_denominator
            } else {
                0.0
            };

            let contact_velocity1 = compute_contact_velocity(
                *bullet1.get_velocity(),
                bullet1.get_angular_velocity(),
                r1
            );
            let contact_velocity2 = compute_contact_velocity(
                *bullet2.get_velocity(),
                bullet2.get_angular_velocity(),
                r2
            );

            let relative_velocity = contact_velocity2 - contact_velocity1;
            let velocity_along_normal = relative_velocity.dot(normal);
            let restitution_velocity = if velocity_along_normal < -RESTITUTION_VELOCITY_THRESHOLD {
                -restitution * velocity_along_normal
            } else {
                0.0
            };

            let contact_constraint = ContactConstraint::new(
                *contact_point,
                normal,
                manifold.get_penetration_depth(),
                r1,
                r2,
                normal_mass,
                tangent_mass,
                restitution_velocity
            );
            contact_constraints.push(contact_constraint);
        }
    }

    contact_constraints
}

fn compute_contact_velocity(velocity: Vec2, angular_velocity: f32, r: Vec2) -> Vec2 {
    let rotational_velocity = angular_velocity_cross_radius(angular_velocity, r);
    velocity + rotational_velocity
}

fn angular_velocity_cross_radius(angular_velocity: f32, r: Vec2) -> Vec2 {
    Vec2::new(-angular_velocity * r.y, angular_velocity * r.x)
}

use bevy::prelude::Vec2;

use crate::{
    collision::{
        contact_manifold::ContactManifold,
        rigid_body_math::{compute_contact_velocity, compute_effective_mass, inverse_or_zero},
    },
    config::RESTITUTION_VELOCITY_THRESHOLD,
    geometry::vector::perpendicular,
    models::bullet::Bullet,
};

pub struct ContactConstraint {
    pub(crate) normal: Vec2,
    pub(crate) tangent: Vec2,
    pub(crate) r1: Vec2,
    pub(crate) r2: Vec2,
    pub(crate) normal_mass: f32,
    pub(crate) tangent_mass: f32,
    pub(crate) accumulated_normal_impulse: f32,
    pub(crate) accumulated_tangent_impulse: f32,
    pub(crate) restitution_velocity: f32,
}

impl ContactConstraint {
    pub fn new(
        normal: Vec2,
        r1: Vec2,
        r2: Vec2,
        normal_mass: f32,
        tangent_mass: f32,
        restitution_velocity: f32,
    ) -> Self {
        let tangent = perpendicular(normal);

        debug_assert!(
            (normal.length_squared() - 1.0).abs() <= 0.001,
            "Contact constraint normal must be normalized."
        );

        Self {
            normal,
            tangent,
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
    manifolds: &[ContactManifold],
) -> Vec<ContactConstraint> {
    let mut contact_constraints = Vec::new();

    let inverse_mass1 = inverse_or_zero(bullet1.get_mass());
    let inverse_mass2 = inverse_or_zero(bullet2.get_mass());

    let inverse_inertia1 = inverse_or_zero(bullet1.get_moment_of_inertia());
    let inverse_inertia2 = inverse_or_zero(bullet2.get_moment_of_inertia());

    let position1 = bullet1.get_position();
    let position2 = bullet2.get_position();

    let restitution = bullet1.get_restitution().min(bullet2.get_restitution());

    for manifold in manifolds {
        let normal = manifold.get_normal();
        let tangent = perpendicular(normal);

        for contact_point in manifold.get_contacts() {
            let r1 = contact_point - position1;
            let r2 = contact_point - position2;

            let normal_mass = compute_effective_mass(
                inverse_mass1,
                inverse_mass2,
                inverse_inertia1,
                inverse_inertia2,
                r1,
                r2,
                normal,
            );

            let tangent_mass = compute_effective_mass(
                inverse_mass1,
                inverse_mass2,
                inverse_inertia1,
                inverse_inertia2,
                r1,
                r2,
                tangent,
            );

            let contact_velocity1 = compute_contact_velocity(
                *bullet1.get_velocity(),
                bullet1.get_angular_velocity(),
                r1,
            );

            let contact_velocity2 = compute_contact_velocity(
                *bullet2.get_velocity(),
                bullet2.get_angular_velocity(),
                r2,
            );

            let relative_velocity = contact_velocity2 - contact_velocity1;
            let velocity_along_normal = relative_velocity.dot(normal);
            let restitution_velocity = if velocity_along_normal < -RESTITUTION_VELOCITY_THRESHOLD {
                -restitution * velocity_along_normal
            } else {
                0.0
            };

            contact_constraints.push(ContactConstraint::new(
                normal,
                r1,
                r2,
                normal_mass,
                tangent_mass,
                restitution_velocity,
            ));
        }
    }

    contact_constraints
}

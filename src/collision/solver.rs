use crate::{
    collision::{
        contact_constraint::{ContactConstraint, build_contact_constraints},
        contact_manifold::ContactManifold,
        rigid_body_math::{combine_friction, compute_contact_velocity, inverse_or_zero},
    },
    config::{EPSILON, SOLVER_ITERATIONS},
    models::bullet::Bullet,
};

const PENETRATION_SLOP: f32 = 0.01;
const CORRECTION_PERCENTAGE: f32 = 0.8;

pub fn solve_collision_manifolds(
    bullet1: &mut Bullet,
    bullet2: &mut Bullet,
    manifolds: &[ContactManifold],
) {
    if manifolds.is_empty() {
        return;
    }

    let mut constraints = build_contact_constraints(bullet1, bullet2, manifolds);

    if !constraints.is_empty() {
        solve_velocity_constraints(bullet1, bullet2, &mut constraints);
    }

    let inverse_mass1 = inverse_or_zero(bullet1.get_mass());
    let inverse_mass2 = inverse_or_zero(bullet2.get_mass());
    let inverse_mass_sum = inverse_mass1 + inverse_mass2;

    for manifold in manifolds {
        correct_penetration(
            bullet1,
            bullet2,
            manifold,
            inverse_mass1,
            inverse_mass2,
            inverse_mass_sum,
        );
    }
}

fn solve_velocity_constraints(
    bullet1: &mut Bullet,
    bullet2: &mut Bullet,
    constraints: &mut [ContactConstraint],
) {
    let static_friction =
        combine_friction(bullet1.get_static_friction(), bullet2.get_static_friction());

    let dynamic_friction = combine_friction(
        bullet1.get_dynamic_friction(),
        bullet2.get_dynamic_friction(),
    );

    for _ in 0..SOLVER_ITERATIONS {
        for contact_constraint in constraints.iter_mut() {
            solve_normal_constraint(bullet1, bullet2, contact_constraint);

            solve_friction_constraint(
                bullet1,
                bullet2,
                contact_constraint,
                static_friction,
                dynamic_friction,
            );
        }
    }
}

fn solve_normal_constraint(
    bullet1: &mut Bullet,
    bullet2: &mut Bullet,
    contact_constraint: &mut ContactConstraint,
) {
    let inverse_mass1 = inverse_or_zero(bullet1.get_mass());
    let inverse_mass2 = inverse_or_zero(bullet2.get_mass());

    let inverse_inertia1 = inverse_or_zero(bullet1.get_moment_of_inertia());
    let inverse_inertia2 = inverse_or_zero(bullet2.get_moment_of_inertia());

    let velocity1 = bullet1.get_velocity();
    let velocity2 = bullet2.get_velocity();

    let angular_velocity1 = bullet1.get_angular_velocity();
    let angular_velocity2 = bullet2.get_angular_velocity();

    let contact_velocity1 =
        compute_contact_velocity(velocity1, angular_velocity1, contact_constraint.r1);
    let contact_velocity2 =
        compute_contact_velocity(velocity2, angular_velocity2, contact_constraint.r2);

    let relative_velocity = contact_velocity2 - contact_velocity1;
    let normal_velocity = relative_velocity.dot(contact_constraint.normal);

    let impulse_delta = (contact_constraint.restitution_velocity - normal_velocity)
        * contact_constraint.normal_mass;

    let old_accumulated_impulse = contact_constraint.accumulated_normal_impulse;
    let new_accumulated_impulse = (old_accumulated_impulse + impulse_delta).max(0.0);
    contact_constraint.accumulated_normal_impulse = new_accumulated_impulse;

    let applied_impulse_magnitude = new_accumulated_impulse - old_accumulated_impulse;

    if applied_impulse_magnitude.abs() <= EPSILON {
        return;
    }

    let impulse = contact_constraint.normal * applied_impulse_magnitude;

    let new_velocity1 = velocity1 - impulse * inverse_mass1;
    let new_velocity2 = velocity2 + impulse * inverse_mass2;

    let new_angular_velocity1 =
        angular_velocity1 - contact_constraint.r1.perp_dot(impulse) * inverse_inertia1;
    let new_angular_velocity2 =
        angular_velocity2 + contact_constraint.r2.perp_dot(impulse) * inverse_inertia2;

    bullet1.set_velocity(new_velocity1);
    bullet2.set_velocity(new_velocity2);

    bullet1.set_angular_velocity(new_angular_velocity1);
    bullet2.set_angular_velocity(new_angular_velocity2);
}

fn solve_friction_constraint(
    bullet1: &mut Bullet,
    bullet2: &mut Bullet,
    contact_constraint: &mut ContactConstraint,
    static_friction: f32,
    dynamic_friction: f32,
) {
    let inverse_mass1 = inverse_or_zero(bullet1.get_mass());
    let inverse_mass2 = inverse_or_zero(bullet2.get_mass());

    let inverse_inertia1 = inverse_or_zero(bullet1.get_moment_of_inertia());
    let inverse_inertia2 = inverse_or_zero(bullet2.get_moment_of_inertia());

    let velocity1 = bullet1.get_velocity();
    let velocity2 = bullet2.get_velocity();

    let angular_velocity1 = bullet1.get_angular_velocity();
    let angular_velocity2 = bullet2.get_angular_velocity();

    let contact_velocity1 =
        compute_contact_velocity(velocity1, angular_velocity1, contact_constraint.r1);

    let contact_velocity2 =
        compute_contact_velocity(velocity2, angular_velocity2, contact_constraint.r2);

    let relative_velocity = contact_velocity2 - contact_velocity1;
    let tangent_velocity = relative_velocity.dot(contact_constraint.tangent);
    let friction_delta = -tangent_velocity * contact_constraint.tangent_mass;
    let old_impulse = contact_constraint.accumulated_tangent_impulse;

    let candidate_impulse = old_impulse + friction_delta;
    let maximum_static_friction = contact_constraint.accumulated_normal_impulse * static_friction;

    let new_impulse = if candidate_impulse.abs() <= maximum_static_friction {
        candidate_impulse
    } else {
        let maximum_dynamic_friction =
            contact_constraint.accumulated_normal_impulse * dynamic_friction;

        candidate_impulse.clamp(-maximum_dynamic_friction, maximum_dynamic_friction)
    };

    contact_constraint.accumulated_tangent_impulse = new_impulse;

    let applied_delta = new_impulse - old_impulse;
    if applied_delta.abs() <= EPSILON {
        return;
    }

    let friction_impulse = contact_constraint.tangent * applied_delta;

    let new_velocity1 = velocity1 - friction_impulse * inverse_mass1;
    let new_velocity2 = velocity2 + friction_impulse * inverse_mass2;

    let new_angular_velocity1 =
        angular_velocity1 - contact_constraint.r1.perp_dot(friction_impulse) * inverse_inertia1;
    let new_angular_velocity2 =
        angular_velocity2 + contact_constraint.r2.perp_dot(friction_impulse) * inverse_inertia2;

    bullet1.set_velocity(new_velocity1);
    bullet2.set_velocity(new_velocity2);

    bullet1.set_angular_velocity(new_angular_velocity1);
    bullet2.set_angular_velocity(new_angular_velocity2);
}

fn correct_penetration(
    bullet1: &mut Bullet,
    bullet2: &mut Bullet,
    manifold: &ContactManifold,
    inverse_mass1: f32,
    inverse_mass2: f32,
    inverse_mass_sum: f32,
) {
    if inverse_mass_sum <= EPSILON {
        return;
    }

    let penetration_depth = manifold.get_penetration_depth();

    let correction_magnitude = ((penetration_depth - PENETRATION_SLOP).max(0.0) / inverse_mass_sum)
        * CORRECTION_PERCENTAGE;

    if correction_magnitude <= 0.0 {
        return;
    }

    let correction = manifold.get_normal() * correction_magnitude;

    let position1 = bullet1.get_position();
    let position2 = bullet2.get_position();

    bullet1.set_position(position1 - correction * inverse_mass1);
    bullet2.set_position(position2 + correction * inverse_mass2);
}

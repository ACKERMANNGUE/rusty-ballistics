use bevy::prelude::Vec2;

use crate::{config::EPSILON, geometry::vector::cross_2d};

pub fn inverse_or_zero(value: f32) -> f32 {
    if value.abs() <= EPSILON {
        0.0
    } else {
        1.0 / value
    }
}

pub fn angular_velocity_cross_radius(angular_velocity: f32, radius: Vec2) -> Vec2 {
    Vec2::new(-angular_velocity * radius.y, angular_velocity * radius.x)
}

pub fn compute_contact_velocity(velocity: Vec2, angular_velocity: f32, lever_arm: Vec2) -> Vec2 {
    velocity + angular_velocity_cross_radius(angular_velocity, lever_arm)
}

pub fn compute_effective_mass(
    inverse_mass1: f32,
    inverse_mass2: f32,
    inverse_inertia1: f32,
    inverse_inertia2: f32,
    r1: Vec2,
    r2: Vec2,
    axis: Vec2,
) -> f32 {
    let r1_cross_axis = cross_2d(r1, axis);
    let r2_cross_axis = cross_2d(r2, axis);

    let denominator = inverse_mass1
        + inverse_mass2
        + r1_cross_axis.powi(2) * inverse_inertia1
        + r2_cross_axis.powi(2) * inverse_inertia2;

    inverse_or_zero(denominator)
}

pub fn combine_friction(friction1: f32, friction2: f32) -> f32 {
    (friction1 * friction2).sqrt()
}

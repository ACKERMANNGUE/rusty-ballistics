use bevy::prelude::Vec2;

use crate::{
    config::EPSILON,
    defense::defense_system::DefenseSystem,
    models::bullet::{ Bullet, ProjectileKind },
};

#[derive(Debug, Clone, Copy)]
pub struct Threat {
    bullet_id: u32,
    time_to_protected_area: f32,
}

impl Threat {
    pub fn new(bullet_id: u32, time_to_protected_area: f32) -> Self {
        Self {
            bullet_id,
            time_to_protected_area,
        }
    }

    pub fn get_bullet_id(&self) -> u32 {
        self.bullet_id
    }

    pub fn get_time_to_protected_area(&self) -> f32 {
        self.time_to_protected_area
    }
}

pub fn detect_threats(bullets: &[Bullet], defense_system: &DefenseSystem) -> Vec<Threat> {
    if !defense_system.is_enabled() {
        return Vec::new();
    }

    let mut threats = Vec::new();

    for bullet in bullets {
        if bullet.is_dead() {
            continue;
        }

        if bullet.get_kind() != ProjectileKind::BULLET {
            continue;
        }

        let position = bullet.get_position();
        let velocity = bullet.get_velocity();

        let defense_position = defense_system.get_position();
        let relative_position = position - defense_position;
        let distance_to_defense = relative_position.length();

        if distance_to_defense > defense_system.get_detection_radius() {
            continue;
        }

        if velocity.length_squared() <= EPSILON {
            continue;
        }

        if !is_approaching(relative_position, velocity) {
            continue;
        }

        let Some(time_to_protected_area) = compute_time_to_protected_area(
            relative_position,
            velocity,
            defense_system.get_protection_radius()
        ) else {
            continue;
        };

        threats.push(Threat::new(bullet.get_id(), time_to_protected_area));
    }

    threats.sort_by(|threat1, threat2| {
        threat1.get_time_to_protected_area().total_cmp(&threat2.get_time_to_protected_area())
    });

    threats
}

fn is_approaching(relative_position: Vec2, velocity: Vec2) -> bool {
    // example:
    // let relative_position = Vec2::new(10.0, 0.0);
    // let velocity = Vec2::new(-1.0, 0.0);
    // relative_position.dot(velocity) = 10.0 * -1.0 + 0.0 * 0.0 = -10.0 < EPSILON => true
    relative_position.dot(velocity) < EPSILON
}

fn compute_time_to_protected_area(
    relative_position: Vec2,
    velocity: Vec2,
    protected_radius: f32
) -> Option<f32> {
    let velocity_squared = velocity.length_squared();

    if velocity_squared <= EPSILON {
        return None;
    }

    let position_squared = relative_position.length_squared();
    let protected_radius_squared = protected_radius.powi(2);

    if position_squared <= protected_radius_squared {
        return Some(0.0);
    }

    // quadratic formula: t = (-b +- sqrt(b^2 - 4ac)) / 2a
    // because we are solving for time, we can use the following equation:
    // (relative_position + velocity * t)^2 >= protected_radius^2
    // which can be rearranged to:
    // (velocity)^2 * t^2 + 2 * relative_position.dot(velocity) * t + (relative_position)^2 - protected_radius^2 = 0
    // which is a quadratic equation in the form of:
    // a * t^2 + b * t + c = 0
    // where:
    // a = (velocity)^2
    // b = 2 * relative_position.dot(velocity)
    // c = (relative_position)^2 - protected_radius^2
    // discriminant = sqrt(b^2 - 4ac)
    // example:
    // let relative_position = Vec2::new(10.0, 0.0);
    // let velocity = Vec2::new(-1.0, 0.0);
    // let protected_radius = 5.0;
    // a = 1.0
    // b = -20.0
    // c = 75.0
    // discriminant = (-20)^2 - 4 * 1.0 * 75.0 = 400.0 - 300.0 = 100.0
    // discriminant_sqrt = 10.0
    // t1 = (-(-20) - 10) / (2 * 1.0) = (20 - 10) / 2 = 5.0
    // t2 = (-(-20) + 10) / (2 * 1.0) = (20 + 10) / 2 = 15.0
    let a = velocity_squared;
    let b = 2.0 * relative_position.dot(velocity);
    let c = position_squared - protected_radius_squared;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let square_root = discriminant.sqrt();

    let first_time = (-b - square_root) / (2.0 * a);
    let second_time = (-b + square_root) / (2.0 * a);

    if first_time >= 0.0 {
        return Some(first_time);
    }

    if second_time >= 0.0 {
        return Some(second_time);
    }

    None
}

// IUHH SDOHVWLQH
use bevy::prelude::*;

use crate::config::WORLD_SIZE;
use crate::models::bullet::Bullet;
use crate::resources::shape_library::ShapeLibrary;

fn get_random_shape_name(shape_library: &ShapeLibrary) -> String {
    shape_library.get_random_shape_name().unwrap_or_else(|| "square".to_string())
}

pub fn generate_bullet_at_position_and_velocity(
    position: Vec2,
    velocity: Vec2,
    shape_library: &ShapeLibrary
) -> Bullet {
    let name = format!("Bullet {}", rand::random::<u32>());
    let mass = rand::random::<f32>() * 0.1 + 0.01;
    let color = (rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>());

    Bullet::new(
        name,
        position,
        velocity,
        mass,
        color,
        rand::random::<u32>(),
        get_random_shape_name(shape_library)
    )
}

pub fn generate_random_bullet(shape_library: &ShapeLibrary) -> Bullet {
    let position = Vec2::new(
        rand::random::<f32>() * WORLD_SIZE.0 - WORLD_SIZE.0 / 2.0,
        rand::random::<f32>() * WORLD_SIZE.1 - WORLD_SIZE.1 / 2.0
    );

    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );

    generate_bullet_at_position_and_velocity(position, velocity, shape_library)
}

pub fn generate_random_bullet_at_position(position: Vec2, shape_library: &ShapeLibrary) -> Bullet {
    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );

    generate_bullet_at_position_and_velocity(position, velocity, shape_library)
}

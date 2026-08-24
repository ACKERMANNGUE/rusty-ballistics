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
    shape_name: &str
) -> Bullet {
    let mass = rand::random::<f32>() * 100.0 + 1.0;

    let color = (rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>());

    Bullet::new(position, velocity, mass, color, rand::random::<u32>(), shape_name.to_string())
}

pub fn generate_random_bullet(shape_name: &str) -> Bullet {
    let position = Vec2::new(
        rand::random::<f32>() * WORLD_SIZE.0 - WORLD_SIZE.0 / 2.0,
        rand::random::<f32>() * WORLD_SIZE.1 - WORLD_SIZE.1 / 2.0
    );

    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );

    generate_bullet_at_position_and_velocity(position, velocity, shape_name)
}

pub fn generate_random_bullet_at_position(position: Vec2, shape_name: &str) -> Bullet {
    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );

    generate_bullet_at_position_and_velocity(position, velocity, shape_name)
}

pub fn generate_random_bullet_with_random_shape(shape_library: &ShapeLibrary) -> Bullet {
    let shape_name = get_random_shape_name(shape_library);

    generate_random_bullet(&shape_name)
}

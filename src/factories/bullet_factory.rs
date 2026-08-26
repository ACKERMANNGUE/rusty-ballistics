use bevy::prelude::*;

use crate::config::WORLD_SIZE;
use crate::models::bullet::Bullet;
use crate::resources::bullet_spawn_settings::BulletSpawnSettings;
use crate::resources::shape_library::ShapeLibrary;

pub fn get_random_shape_name(shape_library: &ShapeLibrary) -> String {
    shape_library.get_random_shape_name().unwrap_or_else(|| "square".to_string())
}

pub fn generate_bullet_at_position_and_velocity(
    position: Vec2,
    velocity: Vec2,
    shape_name: &str,
    spawn_settings: &BulletSpawnSettings
) -> Bullet {
    let mass = spawn_settings.get_mass();
    let restitution = spawn_settings.get_restitution();
    let static_friction = spawn_settings.get_static_friction();
    let dynamic_friction = spawn_settings.get_dynamic_friction();

    let color = (rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>());
    Bullet::new(position, velocity, mass, color, rand::random::<u32>(), shape_name.to_string(), restitution, static_friction, dynamic_friction)
}

pub fn generate_random_bullet(
    shape_name: &str,
    spawn_settings: &BulletSpawnSettings
) -> Bullet {
    let position = Vec2::new(
        rand::random::<f32>() * WORLD_SIZE.0 - WORLD_SIZE.0 / 2.0,
        rand::random::<f32>() * WORLD_SIZE.1 - WORLD_SIZE.1 / 2.0
    );

    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );

    generate_bullet_at_position_and_velocity(position, velocity, shape_name, spawn_settings)
}

pub fn generate_random_bullet_at_position(
    position: Vec2,
    shape_name: &str,
    spawn_settings: &BulletSpawnSettings
) -> Bullet {
    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );

    generate_bullet_at_position_and_velocity(position, velocity, shape_name, spawn_settings)
}

use bevy::prelude::*;

use crate::config::{ MAX_BULLET_VELOCITY, WORLD_SIZE };
use crate::models::bullet::Bullet;
use crate::resources::bullet_spawn_settings::BulletSpawnSettings;
use crate::resources::shape_library::ShapeLibrary;

use crate::geometry::mass_properties::compute_mass_properties;

pub fn get_random_shape_name(shape_library: &ShapeLibrary) -> &str {
    shape_library.get_random_shape_name().unwrap_or("square")
}

pub fn generate_bullet_at_position_and_velocity_with_rotation(
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    shape_name: &str,
    spawn_settings: &BulletSpawnSettings,
    shape_library: &ShapeLibrary
) -> Bullet {
    let size = spawn_settings.get_size();
    let density = spawn_settings.get_density();

    let color = (rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>());

    let shape = shape_library
        .get(shape_name)
        .unwrap_or_else(|| {
            panic!("Cannot create bullet: shape '{}' does not exist.", shape_name)
        });

    let mass_properties = compute_mass_properties(shape, size, density);
    let mass = mass_properties.get_mass();
    let moment_of_inertia = mass_properties.get_moment_of_inertia();

    Bullet::new(
        position,
        velocity,
        rotation,
        mass,
        size,
        moment_of_inertia,
        color,
        rand::random::<u32>(),
        shape_name.to_string(),
        spawn_settings.get_restitution(),
        spawn_settings.get_static_friction(),
        spawn_settings.get_dynamic_friction()
    )
}

pub fn generate_random_bullet(
    shape_name: &str,
    spawn_settings: &BulletSpawnSettings,
    shape_library: &ShapeLibrary
) -> Bullet {
    let position = Vec2::new(
        rand::random::<f32>() * WORLD_SIZE.0 - WORLD_SIZE.0 / 2.0,
        rand::random::<f32>() * WORLD_SIZE.1 - WORLD_SIZE.1 / 2.0
    );

    generate_bullet_at_position_and_velocity_with_rotation(
        position,
        generate_random_velocity(),
        generate_random_rotation(),
        shape_name,
        spawn_settings,
        shape_library
    )
}

pub fn generate_random_bullet_at_position(
    position: Vec2,
    shape_name: &str,
    spawn_settings: &BulletSpawnSettings,
    shape_library: &ShapeLibrary
) -> Bullet {
    generate_bullet_at_position_and_velocity_with_rotation(
        position,
        generate_random_velocity(),
        generate_random_rotation(),
        shape_name,
        spawn_settings,
        shape_library
    )
}

fn generate_random_velocity() -> Vec2 {
    Vec2::new(
        rand::random::<f32>() * MAX_BULLET_VELOCITY * 2.0 - MAX_BULLET_VELOCITY,
        rand::random::<f32>() * MAX_BULLET_VELOCITY * 2.0 - MAX_BULLET_VELOCITY
    )
}

fn generate_random_rotation() -> f32 {
    rand::random::<f32>() * std::f32::consts::TAU
}

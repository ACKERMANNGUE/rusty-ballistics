use bevy::prelude::*;

use crate::{
    config::{
        DEFENSE_INTERCEPTOR_DENSITY,
        DEFENSE_INTERCEPTOR_DYNAMIC_FRICTION,
        DEFENSE_INTERCEPTOR_RESTITUTION,
        DEFENSE_INTERCEPTOR_SHAPE,
        DEFENSE_INTERCEPTOR_SIZE,
        DEFENSE_INTERCEPTOR_STATIC_FRICTION,
        MAX_BULLET_VELOCITY,
        WORLD_SIZE,
    },
    geometry::mass_properties::compute_mass_properties,
    models::bullet::{ Bullet, ProjectileKind },
    resources::{ bullet_spawn_settings::BulletSpawnSettings, shape_library::ShapeLibrary },
};

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
    generate_projectile(
        position,
        velocity,
        rotation,
        shape_name,
        spawn_settings.get_size(),
        spawn_settings.get_density(),
        spawn_settings.get_restitution(),
        spawn_settings.get_static_friction(),
        spawn_settings.get_dynamic_friction(),
        (rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()),
        ProjectileKind::BULLET,
        shape_library
    )
}

pub fn generate_interceptor(
    position: Vec2,
    velocity: Vec2,
    shape_library: &ShapeLibrary
) -> Bullet {
    generate_projectile(
        position,
        velocity,
        velocity.to_angle(),
        DEFENSE_INTERCEPTOR_SHAPE,
        DEFENSE_INTERCEPTOR_SIZE,
        DEFENSE_INTERCEPTOR_DENSITY,
        DEFENSE_INTERCEPTOR_RESTITUTION,
        DEFENSE_INTERCEPTOR_STATIC_FRICTION,
        DEFENSE_INTERCEPTOR_DYNAMIC_FRICTION,
        (0.2, 0.7, 1.0),
        ProjectileKind::INTERCEPTOR,
        shape_library
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

fn generate_projectile(
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    shape_name: &str,
    size: f32,
    density: f32,
    restitution: f32,
    static_friction: f32,
    dynamic_friction: f32,
    color: (f32, f32, f32),
    kind: ProjectileKind,
    shape_library: &ShapeLibrary
) -> Bullet {
    let shape = shape_library
        .get(shape_name)
        .unwrap_or_else(|| {
            panic!("Cannot create projectile: shape '{}' does not exist.", shape_name)
        });

    let mass_properties = compute_mass_properties(shape, size, density);

    Bullet::new(
        position,
        velocity,
        rotation,
        mass_properties.get_mass(),
        size,
        mass_properties.get_moment_of_inertia(),
        color,
        rand::random::<u32>(),
        shape_name.to_string(),
        restitution,
        static_friction,
        dynamic_friction,
        kind
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

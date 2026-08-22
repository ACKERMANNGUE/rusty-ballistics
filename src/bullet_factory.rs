use bevy::prelude::*;
use glam::Vec2;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;

use crate::config::{
    TRAIL_MAX_POINTS,
    WORLD_SIZE,
};

use crate::models::bullet::Bullet;

pub fn generate_random_bullet() -> Bullet {
    let name = format!("Bullet {}", rand::random::<u32>());

    let position = Vec2::new(
        rand::random::<f32>() * WORLD_SIZE.0 - WORLD_SIZE.0 / 2.0,
        rand::random::<f32>() * WORLD_SIZE.1 - WORLD_SIZE.1 / 2.0,
    );

    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0,
    );

    let mass = rand::random::<f32>() * 0.1 + 0.01;

    let color = (
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    );

    Bullet::new(
        name,
        position,
        velocity,
        mass,
        color,
    )
}

pub fn generate_random_bullet_at_position(position: Vec2) -> Bullet {
    let name = format!("Bullet {}", rand::random::<u32>());

    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0,
    );

    let mass = rand::random::<f32>() * 0.1 + 0.01;

    let color = (
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    );

    Bullet::new(
        name,
        position,
        velocity,
        mass,
        color,
    )
}

pub fn spawn_bullet_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    bullet: &Bullet,
    index: usize,
) {
    let radius = bullet.get_radius();

    let color = Color::srgb(
        bullet.get_color().0,
        bullet.get_color().1,
        bullet.get_color().2,
    );

    commands.spawn((
        BulletEntity { index },
        BulletTrail::new(TRAIL_MAX_POINTS),

        Mesh2d(
            meshes
                .add(Circle::new(radius))
                .into()
        ),

        MeshMaterial2d(
            materials.add(color)
        ),

        Transform::from_xyz(
            bullet.get_position().x,
            bullet.get_position().y,
            0.0,
        ),
    ));
}
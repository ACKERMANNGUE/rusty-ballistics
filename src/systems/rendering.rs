use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::models::world::SimulationWorld;

pub fn sync_bullet_transforms(
    world: Res<SimulationWorld>,
    mut query: Query<(
        &BulletEntity,
        &mut Transform,
    )>,
) {
    let bullets = world.get_bullets_read();

    for (bullet_entity, mut transform) in &mut query {
        let bullet =
            &bullets[bullet_entity.index];

        let position =
            bullet.get_position();

        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

pub fn draw_world_bounds(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
) {
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::new(
            world.get_size().0,
            world.get_size().1,
        ),
        Color::WHITE,
    );
}

pub fn draw_bullet_trails(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    query: Query<(
        &BulletEntity,
        &BulletTrail,
    )>,
) {
    let bullets = world.get_bullets_read();

    for (bullet_entity, trail) in &query {
        if trail.points.len() < 2 {
            continue;
        }

        let bullet =
            &bullets[bullet_entity.index];

        let color =
            bullet.get_color();

        gizmos.linestrip_2d(
            trail.points.iter().copied(),
            Color::srgb(
                color.0,
                color.1,
                color.2,
            ),
        );
    }
}
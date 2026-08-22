use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::models::world::SimulationWorld;

pub fn sync_bullet_transforms(
    world: Res<SimulationWorld>,
    mut commands: Commands,
    mut query: Query<(Entity, &BulletEntity, &mut Transform)>
) {
    let bullets = world.get_bullets_read();

    for (entity, bullet_entity, mut transform) in &mut query {
        let Some(bullet) = find_bullet_by_id(bullets, bullet_entity.get_id()) else {
            commands.entity(entity).despawn();
            continue;
        };

        let position = bullet.get_position();

        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

pub fn find_bullet_by_id<'a>(
    bullets: &'a [crate::models::bullet::Bullet],
    id: u32
) -> Option<&'a crate::models::bullet::Bullet> {
    bullets.iter().find(|bullet| bullet.get_id() == id)
}

pub fn draw_world_bounds(mut gizmos: Gizmos, world: Res<SimulationWorld>) {
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::new(world.get_size().0, world.get_size().1),
        Color::WHITE
    );
}

pub fn draw_bullet_trails(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    mut commands: Commands,
    query: Query<(Entity, &BulletEntity, &BulletTrail)>
) {
    let bullets = world.get_bullets_read();

    for (entity, bullet_entity, trail) in &query {
        if trail.points.len() < 2 {
            continue;
        }

        let Some(bullet) = find_bullet_by_id(bullets, bullet_entity.get_id()) else {
            commands.entity(entity).despawn();
            continue;
        };

        let color = bullet.get_color();

        gizmos.linestrip_2d(trail.points.iter().copied(), Color::srgb(color.0, color.1, color.2));
    }
}

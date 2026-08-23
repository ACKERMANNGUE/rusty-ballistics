use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::models::world::SimulationWorld;

use crate::rendering::bullet_renderer::find_bullet_by_id;

pub fn update_simulation(mut world: ResMut<SimulationWorld>) {
    world.update();
}

pub fn record_bullet_trails(
    world: Res<SimulationWorld>,
    mut query: Query<(&BulletEntity, &mut BulletTrail)>,
) {
    let bullets = world.get_bullets_read();

    for (bullet_entity, mut trail) in &mut query {
        let Some(bullet) = find_bullet_by_id(bullets, bullet_entity.get_id()) else {
            continue;
        };

        let position = bullet.get_position();

        trail.push(Vec2::new(position.x, position.y));
    }
}

pub fn despawn_orphan_bullet_entities(
    world: Res<SimulationWorld>,
    mut commands: Commands,
    query: Query<(Entity, &BulletEntity)>,
) {
    let bullets = world.get_bullets_read();

    for (entity, bullet_entity) in &query {
        if find_bullet_by_id(bullets, bullet_entity.get_id()).is_none() {
            commands.entity(entity).despawn();
        }
    }
}

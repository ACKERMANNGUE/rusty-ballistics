use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::models::world::SimulationWorld;

use crate::resources::shape_library::ShapeLibrary;

pub fn update_simulation(mut world: ResMut<SimulationWorld>, shape_library: Res<ShapeLibrary>) {
    world.update(&shape_library);
}

pub fn record_bullet_trails(
    world: Res<SimulationWorld>,
    mut query: Query<(&BulletEntity, &mut BulletTrail)>,
) {
    for (bullet_entity, mut trail) in &mut query {
        let Some(bullet) = world.get_bullet_by_id(bullet_entity.get_id()) else {
            continue;
        };

        trail.push(bullet.get_position());
    }
}

pub fn despawn_orphan_bullet_entities(
    world: Res<SimulationWorld>,
    mut commands: Commands,
    query: Query<(Entity, &BulletEntity)>,
) {
    for (entity, bullet_entity) in &query {
        if world.get_bullet_by_id(bullet_entity.get_id()).is_none() {
            commands.entity(entity).despawn();
        }
    }
}

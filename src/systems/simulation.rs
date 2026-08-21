use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::models::world::SimulationWorld;

pub fn update_simulation(
    mut world: ResMut<SimulationWorld>,
) {
    world.update();
}

pub fn record_bullet_trails(
    world: Res<SimulationWorld>,
    mut query: Query<(
        &BulletEntity,
        &mut BulletTrail,
    )>,
) {
    let bullets = world.get_bullets_read();

    for (bullet_entity, mut trail) in &mut query {
        let bullet =
            &bullets[bullet_entity.index];

        let position =
            bullet.get_position();

        trail.push(
            bevy::prelude::Vec2::new(
                position.x,
                position.y,
            )
        );
    }
}
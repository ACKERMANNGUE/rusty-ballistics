use bevy::prelude::*;

use crate::bullet_factory::{
    generate_random_bullet,
    spawn_bullet_entity,
};

use crate::components::bullet_entity::BulletEntity;

use crate::config::BULLET_COUNT;

use crate::models::world::SimulationWorld;

pub fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

pub fn regenerate_bullets(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut world: ResMut<SimulationWorld>,
    bullet_entities: Query<
        Entity,
        With<BulletEntity>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<ColorMaterial>
    >,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    for entity in &bullet_entities {
        commands.entity(entity).despawn();
    }

    world.get_bullets().clear();

    for _ in 0..BULLET_COUNT {
        world.add_bullet(
            generate_random_bullet()
        );
    }

    for (index, bullet) in world
        .get_bullets_read()
        .iter()
        .enumerate()
    {
        spawn_bullet_entity(
            &mut commands,
            &mut meshes,
            &mut materials,
            bullet,
            index,
        );
    }
}
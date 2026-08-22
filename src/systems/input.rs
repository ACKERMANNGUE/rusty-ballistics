use bevy::prelude::*;

use crate::bullet_factory::{
    generate_random_bullet,
    generate_random_bullet_at_position,
    spawn_bullet_entity,
};

use crate::components::bullet_entity::BulletEntity;

use crate::config::BULLET_COUNT;

use crate::models::world::SimulationWorld;

pub fn toggle_pause(keyboard: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
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
    bullet_entities: Query<Entity, With<BulletEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    for entity in &bullet_entities {
        commands.entity(entity).despawn();
    }

    world.get_bullets().clear();

    for _ in 0..BULLET_COUNT {
        world.add_bullet(generate_random_bullet());
    }

    for bullet in world.get_bullets_read().iter() {
        spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, bullet);
    }
}

pub fn create_new_bullet(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut world: ResMut<SimulationWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if !keyboard.just_pressed(KeyCode::KeyF) {
        return;
    }

    let bullet = generate_random_bullet();

    spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, &bullet);

    world.add_bullet(bullet);
}

pub fn clear_bullets(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut world: ResMut<SimulationWorld>,
    bullet_entities: Query<Entity, With<BulletEntity>>
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    for entity in &bullet_entities {
        commands.entity(entity).despawn();
    }

    world.get_bullets().clear();
}

pub fn toggle_wind(keyboard: Res<ButtonInput<KeyCode>>, mut world: ResMut<SimulationWorld>) {
    if !keyboard.just_pressed(KeyCode::KeyW) {
        return;
    }

    let mut wind = world.get_physics_mut().get_wind_mut();

    if wind.is_active() {
        wind.set_active(false);
    } else {
        wind.set_active(true);
    }
}

pub fn spawn_bullet_at_mouse_position(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut commands: Commands,
    mut world: ResMut<SimulationWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    // convert Bevy's Vec2 to the Vec2 type used by the simulation
    let position = glam::Vec2::new(world_position.x, world_position.y);

    let bullet = generate_random_bullet_at_position(position);
    spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, &bullet);

    world.add_bullet(bullet);
}

pub fn spawn_bullets_at_mouse_position(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut commands: Commands,
    mut world: ResMut<SimulationWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    // convert Bevy's Vec2 to the Vec2 type used by the simulation
    let position = glam::Vec2::new(world_position.x, world_position.y);

    for _ in 0..25 {
        let bullet = generate_random_bullet_at_position(position);
        spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, &bullet);

        world.add_bullet(bullet);
    }
}

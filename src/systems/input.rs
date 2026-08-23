use bevy::{ gizmos, prelude::* };

use crate::factories::bullet_factory::{
    generate_bullet_at_position_and_velocity,
    generate_random_bullet,
    generate_random_bullet_at_position,
};

use crate::components::bullet_entity::BulletEntity;

use crate::config::BULLET_COUNT;

use crate::models::world::SimulationWorld;
use crate::rendering::bullet_renderer::spawn_bullet_entity;
use crate::resources::shape_library::ShapeLibrary;

use bevy::window::PrimaryWindow;

use crate::systems::bullet_launcher::BulletLauncher;

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
    mut materials: ResMut<Assets<ColorMaterial>>,
    shape_library: Res<ShapeLibrary>
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    for entity in &bullet_entities {
        commands.entity(entity).despawn();
    }

    world.get_bullets().clear();

    for _ in 0..BULLET_COUNT {
        world.add_bullet(generate_random_bullet(&shape_library));
    }

    for bullet in world.get_bullets_read().iter() {
        spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, &shape_library, bullet);
    }
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

    let wind = world.get_physics_mut().get_wind_mut();

    if wind.is_active() {
        wind.set_active(false);
    } else {
        wind.set_active(true);
    }
}

pub fn spawn_bullets_at_mouse_position(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut commands: Commands,
    mut world: ResMut<SimulationWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    shape_library: Res<ShapeLibrary>
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

    let position = Vec2::new(world_position.x, world_position.y);

    for _ in 0..25 {
        let bullet = generate_random_bullet_at_position(position, &shape_library);
        spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, &shape_library, &bullet);

        world.add_bullet(bullet);
    }
}

pub fn bullet_launcher_input_system(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut launcher: ResMut<BulletLauncher>,
    mut world: ResMut<SimulationWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    shape_library: Res<ShapeLibrary>
) {
    let (camera, camera_transform) = *camera;

    let mouse_world_position = window
        .cursor_position()
        .and_then(|cursor_position| {
            camera.viewport_to_world_2d(camera_transform, cursor_position).ok()
        });

    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = mouse_world_position {
            launcher.set_drag_start(position);
            launcher.set_drag_end(position);
        }
    }

    if mouse_buttons.pressed(MouseButton::Left) && launcher.is_dragging() {
        if let Some(position) = mouse_world_position {
            launcher.set_drag_end(position);
        }

        draw_drag_line(
            &mut gizmos,
            launcher.get_drag_start(),
            launcher.get_drag_end(),
            launcher.get_max_drag_length()
        );
    }

    if mouse_buttons.just_released(MouseButton::Left) && launcher.is_dragging() {
        let spawn_position = launcher.get_drag_start();
        if let Some(velocity) = launcher.release_drag() {
            let bullet = generate_bullet_at_position_and_velocity(
                spawn_position,
                velocity * -1.0,
                &shape_library
            );
            spawn_bullet_entity(
                &mut commands,
                &mut meshes,
                &mut materials,
                &shape_library,
                &bullet
            );
            world.add_bullet(bullet);
        }
    }
}

fn draw_drag_line(gizmos: &mut Gizmos, start: Vec2, end: Vec2, max_drag_length: f32) {
    let drag_vector = end - start;
    let drag_length = drag_vector.length();
    let red_intensity = (drag_length / max_drag_length).clamp(0.0, 1.0);
    let green_intensity = (1.0 - drag_length / max_drag_length).clamp(0.0, 1.0);
    let blue_intensity = 0.5;
    gizmos.line_2d(start, end, Color::srgb(red_intensity, green_intensity, blue_intensity));
}

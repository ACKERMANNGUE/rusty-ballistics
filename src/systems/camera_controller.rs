use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

use crate::config::WORLD_SIZE;

const CAMERA_MOVE_SPEED: f32 = 500.0;

const CAMERA_MIN_ZOOM: f32 = 0.2;
const CAMERA_MAX_ZOOM: f32 = 5.0;
const CAMERA_ZOOM_SPEED: f32 = 0.15;

pub fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Single<(&mut Transform, &Projection), With<Camera2d>>,
) {
    let (transform, projection) = &mut *camera;

    let mut direction = Vec2::ZERO;

    // Up
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    // Down
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    // Left
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    // Right
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    direction = direction.normalize();

    let zoom = match projection {
        Projection::Orthographic(orthographic) => orthographic.scale,
        _ => 1.0,
    };

    let movement = direction * CAMERA_MOVE_SPEED * zoom * time.delta_secs();

    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}

pub fn camera_zoom(
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut projection: Single<&mut Projection, With<Camera2d>>,
) {
    let Projection::Orthographic(orthographic) = &mut **projection else {
        return;
    };

    if mouse_scroll.delta.y == 0.0 {
        return;
    }

    let zoom_factor = 1.0 - mouse_scroll.delta.y * CAMERA_ZOOM_SPEED;

    orthographic.scale = (orthographic.scale * zoom_factor).clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);
}

pub fn clamp_camera_to_world(mut camera: Single<&mut Transform, With<Camera2d>>) {
    let half_world_width = WORLD_SIZE.0 / 2.0;
    let half_world_height = WORLD_SIZE.1 / 2.0;

    camera.translation.x = camera
        .translation
        .x
        .clamp(-half_world_width, half_world_width);

    camera.translation.y = camera
        .translation
        .y
        .clamp(-half_world_height, half_world_height);
}

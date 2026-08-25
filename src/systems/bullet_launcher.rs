use bevy::prelude::*;

use crate::config::MAX_BULLET_VELOCITY;

#[derive(Resource)]
pub struct BulletLauncher {
    is_dragging: bool,
    drag_start: Vec2,
    drag_end: Vec2,
    velocity_scale: f32,
    max_velocity: f32,
}

impl BulletLauncher {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            drag_start: Vec2::ZERO,
            drag_end: Vec2::ZERO,
            velocity_scale: 2.0,
            max_velocity: MAX_BULLET_VELOCITY,
        }
    }

    pub fn set_drag_start(&mut self, position: Vec2) {
        self.drag_start = position;
        self.drag_end = position;
        self.is_dragging = true;
    }

    pub fn set_drag_end(&mut self, position: Vec2) {
        self.drag_end = position;
    }

    pub fn release_drag(&mut self) -> Option<Vec2> {
        if !self.is_dragging {
            return None;
        }

        let drag_vector = self.drag_end - self.drag_start;
        let velocity = drag_vector * self.velocity_scale;
        let clamped_velocity = velocity.clamp_length_max(self.max_velocity);
        self.is_dragging = false;

        Some(clamped_velocity)
    }

    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    pub fn get_drag_start(&self) -> Vec2 {
        self.drag_start
    }

    pub fn get_drag_end(&self) -> Vec2 {
        self.drag_end
    }

    pub fn get_max_drag_length(&self) -> f32 {
        self.max_velocity / self.velocity_scale
    }

    pub fn cancel_drag(&mut self) {
        self.is_dragging = false;
        self.drag_start = Vec2::ZERO;
        self.drag_end = Vec2::ZERO;
    }
}

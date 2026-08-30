use bevy::prelude::*;

use crate::config::EPSILON;

#[derive(Resource)]
pub struct DefenseSystem {
    position: Vec2,
    protection_radius: f32,
    detection_radius: f32,
    interceptor_speed: f32,
    launch_cooldown: f32,
    cooldown_remaining: f32,
    maximum_active_interceptors: usize,
    enabled: bool,
}

impl DefenseSystem {
    pub fn new(
        position: Vec2,
        protection_radius: f32,
        detection_radius: f32,
        interceptor_speed: f32,
        launch_cooldown: f32,
        maximum_active_interceptors: usize
    ) -> Self {
        Self {
            position,
            protection_radius,
            detection_radius,
            interceptor_speed,
            launch_cooldown,
            cooldown_remaining: 0.0,
            maximum_active_interceptors,
            enabled: true,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_protection_radius(&self) -> f32 {
        self.protection_radius
    }

    pub fn get_detection_radius(&self) -> f32 {
        self.detection_radius
    }

    pub fn get_interceptor_speed(&self) -> f32 {
        self.interceptor_speed
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn update_cooldown(&mut self, delta_time: f32) {
        self.cooldown_remaining = (self.cooldown_remaining - delta_time).max(0.0);
    }

    pub fn can_launch_interceptor(&self, active_interceptor_count: usize) -> bool {
        self.enabled &&
            self.cooldown_remaining <= EPSILON &&
            active_interceptor_count < self.maximum_active_interceptors
    }

    pub fn start_launch_cooldown(&mut self) {
        self.cooldown_remaining = self.launch_cooldown;
    }
}

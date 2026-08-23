use bevy::prelude::Vec2;

use crate::config::{ TURBULENCE_MAX_X, TURBULENCE_MAX_Y, TURBULENCE_DELTA_MAX };

pub struct Wind {
    direction: Vec2,
    speed: f32,
    turbulence: Vec2,
    active: bool,
}

impl Wind {
    pub fn new(direction: Vec2, speed: f32, active: bool) -> Self {
        Self {
            direction,
            speed,
            turbulence: Vec2::ZERO,
            active,
        }
    }

    pub fn get_direction(&self) -> &Vec2 {
        &self.direction
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn update_turbulence(&mut self) {
        let delta_x = (rand::random::<f32>() * 2.0 - 1.0) * TURBULENCE_DELTA_MAX;
        let delta_y = (rand::random::<f32>() * 2.0 - 1.0) * TURBULENCE_DELTA_MAX;

        self.turbulence.x += delta_x;
        self.turbulence.y += delta_y;

        self.turbulence.x = self.turbulence.x.clamp(-TURBULENCE_MAX_X, TURBULENCE_MAX_X);
        self.turbulence.y = self.turbulence.y.clamp(-TURBULENCE_MAX_Y, TURBULENCE_MAX_Y);
    }

    pub fn get_turbulence(&self) -> &Vec2 {
        &self.turbulence
    }

    pub fn get_direction_degrees(&self) -> f32 {
        self.direction.y.atan2(self.direction.x).to_degrees()
    }

    pub fn get_turbulence_direction_degrees(&self) -> f32 {
        self.turbulence.y.atan2(self.turbulence.x).to_degrees()
    }
}

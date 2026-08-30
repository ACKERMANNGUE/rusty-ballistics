use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Vec2,
    max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn get_min(&self) -> Vec2 {
        self.min
    }

    pub fn get_max(&self) -> Vec2 {
        self.max
    }

    pub fn is_outside_bounds(&self, half_width: f32, half_height: f32) -> bool {
        self.min.x < -half_width
            || self.max.x > half_width
            || self.min.y < -half_height
            || self.max.y > half_height
    }
}

use bevy::prelude::*;

pub struct CollisionInfo {
    normal: Vec2,
    penetration_depth: f32,
    contact_point: Vec2,
}

impl CollisionInfo {
    pub fn new(normal: Vec2, penetration_depth: f32, contact_point: Vec2) -> Self {
        Self {
            normal,
            penetration_depth,
            contact_point,
        }
    }

    pub fn get_normal(&self) -> Vec2 {
        self.normal
    }

    pub fn get_penetration_depth(&self) -> f32 {
        self.penetration_depth
    }

    pub fn get_contact_point(&self) -> Vec2 {
        self.contact_point
    }
}

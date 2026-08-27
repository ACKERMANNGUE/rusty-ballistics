use bevy::prelude::*;

pub struct ContactManifold {
    normal: Vec2,
    penetration_depth: f32,
    contact_points: Vec<Vec2>,
}

impl ContactManifold {
    pub fn new(normal: Vec2, penetration_depth: f32, contact_points: Vec<Vec2>) -> Self {
        Self {
            normal,
            penetration_depth,
            contact_points,
        }
    }

    pub fn get_normal(&self) -> &Vec2 {
        &self.normal
    }

    pub fn get_penetration_depth(&self) -> f32 {
        self.penetration_depth
    }

    pub fn get_contact_points(&self) -> &Vec<Vec2> {
        &self.contact_points
    }
}
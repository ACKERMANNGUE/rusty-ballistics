use bevy::prelude::Vec2;

pub fn cross_2d(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

pub fn perpendicular(vector: Vec2) -> Vec2 {
    Vec2::new(-vector.y, vector.x)
}

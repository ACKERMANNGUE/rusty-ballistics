use bevy::prelude::Vec2;

use crate::models::bullet::Bullet;
use crate::resources::shape_library::ShapeLibrary;

pub fn get_bullet_world_shape(bullet: &Bullet, shape_library: &ShapeLibrary) -> Option<Vec<Vec2>> {
    let shape_name = bullet.get_shape();

    let Some(shape_points) = shape_library.get(shape_name) else {
        println!("Warning: Shape '{}' not found in shape library.", shape_name);
        return None;
    };

    let world_points = shape_points
        .iter()
        .map(|point| *bullet.get_position() + *point * bullet.get_size())
        .collect();

    Some(world_points)
}

use bevy::prelude::Vec2;

use crate::models::bullet::Bullet;
use crate::resources::shape_library::ShapeLibrary;

pub fn get_bullet_world_shape(bullet: &Bullet, shape_library: &ShapeLibrary) -> Option<Vec<Vec2>> {
    let shape_name = bullet.get_shape();
    let Some(shape) = shape_library.get(shape_name) else {
        println!("Warning: Shape '{}' not found in shape library.", shape_name);
        return None;
    };

    let vertices = shape.get_vertices();

    let world_points = vertices
        .iter()
        .map(|point| *bullet.get_position() + *point * bullet.get_size())
        .collect();

    Some(world_points)
}

pub fn get_bullet_world_triangles(bullet: &Bullet, shape_library: &ShapeLibrary) -> Option<Vec<[Vec2; 3]>> {
    let shape_name = bullet.get_shape();
    let Some(shape) = shape_library.get(shape_name) else {
        println!("Warning: Shape '{}' not found in shape library.", shape_name);
        return None;
    };

    let vertices = shape.get_vertices();
    let triangles = shape.get_triangles();

    let world_triangles: Vec<[Vec2; 3]> = triangles
        .iter()
        .map(|triangle| {
            [
                *bullet.get_position() + vertices[triangle[0]] * bullet.get_size(),
                *bullet.get_position() + vertices[triangle[1]] * bullet.get_size(),
                *bullet.get_position() + vertices[triangle[2]] * bullet.get_size(),
            ]
        })
        .collect();

    Some(world_triangles)
}
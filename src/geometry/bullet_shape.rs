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
        .map(|point| transform_bullet_vertex(*point, bullet))
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
                transform_bullet_vertex(vertices[triangle[0]], bullet),
                transform_bullet_vertex(vertices[triangle[1]], bullet),
                transform_bullet_vertex(vertices[triangle[2]], bullet),
            ]
        })
        .collect();

    Some(world_triangles)
}

pub fn transform_bullet_vertex(local_vertex: Vec2, bullet: &Bullet) -> Vec2 {
    let position = bullet.get_position();
    let size = bullet.get_size();
    let rotation = bullet.get_rotation();

    let scaled_vertex = local_vertex * size;

    let rotated_vertex = Vec2::new(
        scaled_vertex.x * rotation.cos() - scaled_vertex.y * rotation.sin(),
        scaled_vertex.x * rotation.sin() + scaled_vertex.y * rotation.cos(),
    );

    rotated_vertex + position
}
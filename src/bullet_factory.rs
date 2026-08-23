use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;

use crate::config::{ TRAIL_MAX_POINTS, WORLD_SIZE };

use crate::models::bullet::Bullet;
use crate::resources::shape_library::ShapeLibrary;

fn get_random_shape_name(shape_library: &ShapeLibrary) -> String {
    shape_library
        .get_random_shape_name()
        .unwrap_or_else(|| "square".to_string())
}

pub fn generate_bullet_at_position_and_velocity(position: Vec2, velocity: Vec2, shape_library: &ShapeLibrary) -> Bullet {
    let name = format!("Bullet {}", rand::random::<u32>());
    let mass = rand::random::<f32>() * 0.1 + 0.01;
    let color = (rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>());

    Bullet::new(name, position, velocity, mass, color, rand::random::<u32>(), get_random_shape_name(&shape_library))
}

pub fn generate_random_bullet(shape_library: &ShapeLibrary) -> Bullet {
    let position = Vec2::new(
        rand::random::<f32>() * WORLD_SIZE.0 - WORLD_SIZE.0 / 2.0,
        rand::random::<f32>() * WORLD_SIZE.1 - WORLD_SIZE.1 / 2.0
    );

    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );
    generate_bullet_at_position_and_velocity(position, velocity, shape_library)
}

pub fn generate_random_bullet_at_position(position: Vec2, shape_library: &ShapeLibrary) -> Bullet {
    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0
    );
    generate_bullet_at_position_and_velocity(position, velocity, shape_library)
}

pub fn spawn_bullet_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    shape_library: &ShapeLibrary,
    bullet: &Bullet
) {
    let color = Color::srgb(bullet.get_color().0, bullet.get_color().1, bullet.get_color().2);

    let mesh = match create_bullet_shape(bullet, shape_library) {
        Some(mesh) => mesh,
        None => {
            println!("Failed to create shape '{}'. Using circle as fallback.", bullet.get_shape());

            Mesh::from(Circle::new(bullet.get_size()))
        }
    };

    commands.spawn((
        BulletEntity::new(bullet.get_id()),
        BulletTrail::new(TRAIL_MAX_POINTS),
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(bullet.get_position().x, bullet.get_position().y, 0.0),
    ));
}

fn create_bullet_shape(bullet: &Bullet, shape_library: &ShapeLibrary) -> Option<Mesh> {
    let shape_name = bullet.get_shape();

    let points = shape_library.get(shape_name)?;

    if points.len() < 3 {
        println!("Shape '{}' must contain at least 3 points.", shape_name);

        return None;
    }

    let scaled_points: Vec<Vec2> = points
        .iter()
        .map(|point| *point * bullet.get_size())
        .collect();

    let polygon = match ConvexPolygon::new(scaled_points) {
        Ok(polygon) => polygon,
        Err(error) => {
            println!("Shape '{}' is not a valid convex polygon: {:?}", shape_name, error);
            return None;
        }
    };

    Some(Mesh::from(polygon))
}


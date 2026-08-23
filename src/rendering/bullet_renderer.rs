use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::config::TRAIL_MAX_POINTS;
use crate::models::bullet::Bullet;
use crate::models::world::SimulationWorld;
use crate::resources::shape_library::ShapeLibrary;

pub fn spawn_bullet_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    shape_library: &ShapeLibrary,
    bullet: &Bullet
) {
    let color = Color::srgb(bullet.get_color().0, bullet.get_color().1, bullet.get_color().2);

    let mesh = match create_bullet_mesh(bullet, shape_library) {
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

fn create_bullet_mesh(bullet: &Bullet, shape_library: &ShapeLibrary) -> Option<Mesh> {
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

pub(crate) fn find_bullet_by_id(bullets: &[Bullet], id: u32) -> Option<&Bullet> {
    bullets.iter().find(|bullet| bullet.get_id() == id)
}

pub fn sync_bullet_transforms(
    world: Res<SimulationWorld>,
    mut commands: Commands,
    mut query: Query<(Entity, &BulletEntity, &mut Transform)>
) {
    let bullets = world.get_bullets_read();

    for (entity, bullet_entity, mut transform) in &mut query {
        let Some(bullet) = find_bullet_by_id(bullets, bullet_entity.get_id()) else {
            commands.entity(entity).despawn();
            continue;
        };

        let position = bullet.get_position();
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
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
    bullet: &Bullet,
) {
    let color = Color::srgb(
        bullet.get_color().0,
        bullet.get_color().1,
        bullet.get_color().2,
    );

    let mesh = match create_bullet_mesh(bullet, shape_library) {
        Some(mesh) => mesh,
        None => {
            println!(
                "Failed to create shape '{}'. Using circle as fallback.",
                bullet.get_shape()
            );

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
    let shape = shape_library.get(bullet.get_shape())?;

    let vertices = shape.get_vertices();
    let triangles = shape.get_triangles();

    let mut positions = Vec::with_capacity(triangles.len() * 3);

    for triangle in triangles {
        let triangle_indices = triangle.get_indices();

        let local_a = vertices[triangle_indices[0]] * bullet.get_size();
        let local_b = vertices[triangle_indices[1]] * bullet.get_size();
        let local_c = vertices[triangle_indices[2]] * bullet.get_size();

        positions.push([local_a.x, local_a.y, 0.0]);
        positions.push([local_b.x, local_b.y, 0.0]);
        positions.push([local_c.x, local_c.y, 0.0]);
    }

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    Some(mesh)
}

pub fn sync_bullet_transforms(
    world: Res<SimulationWorld>,
    mut commands: Commands,
    mut query: Query<(Entity, &BulletEntity, &mut Transform)>,
) {
    for (entity, bullet_entity, mut transform) in &mut query {
        let Some(bullet) = world.get_bullet_by_id(bullet_entity.get_id()) else {
            commands.entity(entity).despawn();
            continue;
        };

        let position = bullet.get_position();

        transform.translation.x = position.x;
        transform.translation.y = position.y;
        transform.rotation = Quat::from_rotation_z(bullet.get_rotation());
    }
}

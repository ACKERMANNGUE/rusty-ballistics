use bevy::prelude::*;

use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::models::bullet::Bullet;
use crate::models::world::SimulationWorld;
use crate::rendering::bullet_renderer::find_bullet_by_id;
use crate::resources::shape_library::ShapeLibrary;

pub fn draw_world_bounds(mut gizmos: Gizmos, world: Res<SimulationWorld>) {
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::new(world.get_size().0, world.get_size().1),
        Color::WHITE
    );
}

pub fn draw_bullet_trails(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    mut commands: Commands,
    query: Query<(Entity, &BulletEntity, &BulletTrail)>
) {
    let bullets = world.get_bullets_read();

    for (entity, bullet_entity, trail) in &query {
        if trail.points.len() < 2 {
            continue;
        }

        let Some(bullet) = find_bullet_by_id(bullets, bullet_entity.get_id()) else {
            commands.entity(entity).despawn();
            continue;
        };

        let color = bullet.get_color();
        gizmos.linestrip_2d(trail.points.iter().copied(), Color::srgb(color.0, color.1, color.2));
    }
}

pub fn display_bullet_hitbox(
    world: Res<SimulationWorld>,
    mut gizmos: Gizmos,
    query: Query<&BulletEntity>,
    shape_library: Res<ShapeLibrary>
) {
    let bullets = world.get_bullets_read();

    for bullet_entity in &query {
        let bullet = find_bullet_by_id(bullets, bullet_entity.get_id());
        if let Some(bullet) = bullet {
            draw_bullet_hitbox(bullet, &mut gizmos, &shape_library);
        }
    }
}

fn draw_bullet_hitbox(bullet: &Bullet, gizmos: &mut Gizmos, shape_library: &ShapeLibrary) {
    let Some(shape_points) = shape_library.get(bullet.get_shape()) else {
        println!("Shape '{}' not found in shape library.", bullet.get_shape());
        return;
    };

    let world_points = shape_points
        .iter()
        .chain(shape_points.first())
        .map(|point| *bullet.get_position() + *point * bullet.get_size());

    gizmos.linestrip_2d(world_points, Color::WHITE);
}

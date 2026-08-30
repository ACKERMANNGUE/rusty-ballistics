use bevy::prelude::*;

use crate::collision::narrow_phase::detect_collision_manifolds;
use crate::components::bullet_entity::BulletEntity;
use crate::components::bullet_trail::BulletTrail;
use crate::geometry::bullet_shape::{get_bullet_world_triangles, transform_bullet_vertex};
use crate::models::bullet::Bullet;
use crate::models::world::SimulationWorld;
use crate::resources::shape_library::ShapeLibrary;

pub fn draw_world_bounds(mut gizmos: Gizmos, world: Res<SimulationWorld>) {
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        Vec2::new(world.get_size().0, world.get_size().1),
        Color::srgb(0.8, 0.8, 0.8),
    );
}

pub fn draw_bullet_trails(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    mut commands: Commands,
    query: Query<(Entity, &BulletEntity, &BulletTrail)>,
) {
    for (entity, bullet_entity, trail) in &query {
        if trail.points.len() < 2 {
            continue;
        }

        let Some(bullet) = world.get_bullet_by_id(bullet_entity.get_id()) else {
            commands.entity(entity).despawn();
            continue;
        };

        let color = bullet.get_color();

        gizmos.linestrip_2d(
            trail.points.iter().copied(),
            Color::srgb(color.0, color.1, color.2),
        );
    }
}

pub fn display_bullet_hitbox(
    world: Res<SimulationWorld>,
    mut gizmos: Gizmos,
    query: Query<&BulletEntity>,
    shape_library: Res<ShapeLibrary>,
) {
    for bullet_entity in &query {
        let Some(bullet) = world.get_bullet_by_id(bullet_entity.get_id()) else {
            continue;
        };

        draw_bullet_hitbox(bullet, &mut gizmos, &shape_library);
    }
}

fn draw_bullet_hitbox(bullet: &Bullet, gizmos: &mut Gizmos, shape_library: &ShapeLibrary) {
    let Some(shape_points) = shape_library.get(bullet.get_shape()) else {
        println!("Shape '{}' not found in shape library.", bullet.get_shape());
        return;
    };

    let vertices = shape_points.get_vertices();

    let world_points = vertices
        .iter()
        .chain(vertices.first())
        .map(|point| transform_bullet_vertex(*point, bullet));

    gizmos.linestrip_2d(world_points, Color::srgb(0.8, 0.8, 0.8));
}

pub fn draw_wind_vector(mut gizmos: Gizmos, world: Res<SimulationWorld>) {
    let world_size = world.get_size();
    let wind = world.get_physics().get_wind();
    let arrow_size: f32 = 500.0;

    if wind.is_active() {
        let half_width = world_size.0 / 2.0;
        let half_height = world_size.1 / 2.0;
        let step = arrow_size.round() as i32;

        let start_x = (-half_width).ceil() as i32;
        let end_x = half_width.ceil() as i32;
        let start_y = (-half_height).ceil() as i32;
        let end_y = half_height.ceil() as i32;

        for x in (start_x..=end_x).step_by(step as usize) {
            for y in (start_y..=end_y).step_by(step as usize) {
                let start = Vec2::new(x as f32, y as f32);
                let end = start + wind.get_direction() * wind.get_turbulence() * arrow_size;
                gizmos.arrow_2d(start, end, Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

pub fn draw_bullet_triangulation(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    query: Query<&BulletEntity>,
    shape_library: Res<ShapeLibrary>,
) {
    for bullet_entity in &query {
        let Some(bullet) = world.get_bullet_by_id(bullet_entity.get_id()) else {
            continue;
        };

        draw_bullet_triangulation_for_bullet(bullet, &mut gizmos, &shape_library);
    }
}

fn draw_bullet_triangulation_for_bullet(
    bullet: &Bullet,
    gizmos: &mut Gizmos,
    shape_library: &ShapeLibrary,
) {
    let Some(triangles) = get_bullet_world_triangles(bullet, shape_library) else {
        println!(
            "Warning: Could not get triangles for bullet with shape '{}'.",
            bullet.get_shape()
        );

        return;
    };

    for triangle in triangles {
        let vertices = triangle.get_vertices();
        gizmos.line_2d(vertices[0], vertices[1], Color::srgb(0.5, 0.5, 0.5));
        gizmos.line_2d(vertices[1], vertices[2], Color::srgb(0.5, 0.5, 0.5));
        gizmos.line_2d(vertices[2], vertices[0], Color::srgb(0.5, 0.5, 0.5));
    }
}

pub fn draw_contact_manifolds(
    world: Res<SimulationWorld>,
    shape_library: Res<ShapeLibrary>,
    mut gizmos: Gizmos,
) {
    const CONTACT_RADIUS: f32 = 3.0;
    const NORMAL_LENGTH: f32 = 50.0;

    let bullets = world.get_bullets();

    for first_index in 0..bullets.len() {
        for second_index in first_index + 1..bullets.len() {
            let bullet1 = &bullets[first_index];
            let bullet2 = &bullets[second_index];

            let manifolds = detect_collision_manifolds(bullet1, bullet2, &shape_library);

            for manifold in &manifolds {
                let normal = manifold.get_normal();

                for &contact in manifold.get_contacts() {
                    gizmos.circle_2d(contact, CONTACT_RADIUS, Color::srgb(0.0, 1.0, 0.0));
                    gizmos.line_2d(
                        contact,
                        contact + normal * NORMAL_LENGTH,
                        Color::srgb(1.0, 0.0, 0.0),
                    );
                }
            }
        }
    }
}

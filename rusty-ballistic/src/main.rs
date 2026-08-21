mod models;

use glam::Vec2;
use models::bullet::Bullet;
use models::world::SimulationWorld;
use models::physics::Physics;

const GRAVITY: f32 = 9.81;
const AIR_RESISTANCE: f32 = 0.1;
const WORLD_SIZE: (f32, f32) = (1920.0, 1080.0);
const HZ: f32 = 144.0;
const DELTA_TIME: f32 = 1.0 / HZ;

use bevy::prelude::*;
mod components;
use components::bullet_entity::BulletEntity;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(HZ as f64))
        .add_systems(
            Startup,
            (
                setup,
                resize_window,
            ),
        )
        .add_systems(FixedUpdate, update_simulation)
        .add_systems(
            Update,
            (
                sync_bullet_transforms,
                draw_world_bounds,
            ),
        )
        .run();
}

fn resize_window(
    mut window: Single<&mut Window>,
) {
    window.resolution.set(
        WORLD_SIZE.0,
        WORLD_SIZE.1,
    );
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let physics = Physics::new(DELTA_TIME, AIR_RESISTANCE, GRAVITY);
    let mut world = SimulationWorld::new(WORLD_SIZE, physics);

    let bullet_1 = Bullet::new(
        String::from("B1"),
        Vec2::new(-30.0, 200.0),
        Vec2::new(100.0, -50.0),
        0.02,
        (1.0, 0.0, 0.0)
    );

    let bullet_2 = Bullet::new(
        String::from("B2"),
        Vec2::new(120.0, -200.0),
        Vec2::new(-150.0, 275.0),
        0.09,
        (0.0, 1.0, 0.0)
    );

    let bullet_3 = Bullet::new(
        String::from("B3"),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        0.1,
        (0.0, 0.0, 1.0)
    );

    world.add_bullet(bullet_1);
    world.add_bullet(bullet_2);
    world.add_bullet(bullet_3);

    commands.spawn(Camera2d);

    for (index, bullet) in world.get_bullets().iter().enumerate() {
        let radius = bullet.get_radius();
        let color = Color::srgb(bullet.get_color().0, bullet.get_color().1, bullet.get_color().2);
        commands.spawn((
            BulletEntity { index }, 
            Mesh2d(meshes.add(Circle::new(radius)).into()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(bullet.get_position().x, bullet.get_position().y, 0.0),
        ));
    }

    commands.insert_resource(world);
}

fn update_simulation(
    mut world: ResMut<SimulationWorld>,
) {
    world.update();
}

fn sync_bullet_transforms(
    world: Res<SimulationWorld>,
    mut query: Query<(&BulletEntity, &mut Transform)>,
) {
    let bullets = world.get_bullets_read();
    for (bullet_entity, mut transform) in &mut query {
        let bullet = &bullets[bullet_entity.index];
        let position = bullet.get_position();

        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

fn draw_world_bounds(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
) {
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        bevy::prelude::Vec2::new(
            world.get_size().0,
            world.get_size().1,
        ),
        Color::WHITE,
    );
}
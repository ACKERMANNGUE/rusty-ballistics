mod models;

use glam::Vec2;
use models::bullet::Bullet;
use models::world::SimulationWorld;
use models::physics::Physics;

const GRAVITY: f32 = 9.81;
const AIR_RESISTANCE: f32 = 0.1;
const WORLD_SIZE: (f32, f32) = (800.0, 600.0);
const HZ: f32 = 60.0;
const DELTA_TIME: f32 = 1.0 / HZ;

use bevy::prelude::*;

const BASE_BULLET_RADIUS: f32 = 10.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
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
        (1.0, 0.0, 0.0), 
    );

    let bullet_2 = Bullet::new(
        String::from("B2"),
        Vec2::new(120.0, -300.0),
        Vec2::new(-15.0, 275.0),
        0.009,
        (0.0, 1.0, 0.0), 
    );

    let bullet_3 = Bullet::new(
        String::from("B3"),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        0.1,
        (0.0, 0.0, 1.0), 
    );

    world.add_bullet(bullet_1);
    world.add_bullet(bullet_2);
    world.add_bullet(bullet_3);

    commands.spawn(Camera2d);

    for bullet in world.get_bullets() {
        let radius = (BASE_BULLET_RADIUS * bullet.get_mass() * 10.0) + BASE_BULLET_RADIUS;
        let color = Color::srgb(bullet.get_color().0, bullet.get_color().1, bullet.get_color().2);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(radius)).into()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(bullet.get_position().x, bullet.get_position().y, 0.0),
        ));
    }
}
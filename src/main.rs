mod models;

use glam::Vec2;
use models::bullet::Bullet;
use models::physics::Physics;
use models::world::SimulationWorld;

const GRAVITY: f32 = 9.81;
const AIR_RESISTANCE: f32 = 0.1;
const WORLD_SIZE: (f32, f32) = (1920.0, 1080.0);
const HZ: f32 = 144.0;
const DELTA_TIME: f32 = 1.0 / HZ;

use bevy::prelude::*;
mod components;
use components::bullet_entity::BulletEntity;
use components::bullet_trail::BulletTrail;
const TRAIL_MAX_POINTS: usize = 300;

use bevy::diagnostic::{
    DiagnosticsStore,
    FrameTimeDiagnosticsPlugin,
};

const UI_FONT_SIZE: f32 = 16.0;

const BULLET_COUNT: usize = 50;


use bevy::window::PresentMode;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(HZ as f64))
        .add_systems(
            Startup,
            (
                setup,
                resize_window,
                setup_ui,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                update_simulation,
                record_bullet_trails,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                toggle_pause,
                regenerate_bullets,
                sync_bullet_transforms,
                draw_bullet_trails,
                draw_world_bounds,
                update_ui,
            ),
        )
        .run();
}

fn resize_window(mut window: Single<&mut Window>) {
    window.resolution.set(WORLD_SIZE.0, WORLD_SIZE.1);
    window.present_mode = PresentMode::AutoNoVsync;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let physics = Physics::new(DELTA_TIME, AIR_RESISTANCE, GRAVITY);
    let mut world = SimulationWorld::new(WORLD_SIZE, physics);

    for _ in 0..BULLET_COUNT {
        world.add_bullet(generate_random_bullet());
    }

    commands.spawn(Camera2d);

    for (index, bullet) in world.get_bullets_read().iter().enumerate() {
        spawn_bullet_entity(
            &mut commands,
            &mut meshes,
            &mut materials,
            bullet,
            index,
        );
    }

    commands.insert_resource(world);
}

fn generate_random_bullet() -> Bullet {
    let name = format!("Bullet {}", rand::random::<u32>());
    let position = Vec2::new(
        rand::random::<f32>() * WORLD_SIZE.0 - WORLD_SIZE.0 / 2.0,
        rand::random::<f32>() * WORLD_SIZE.1 - WORLD_SIZE.1 / 2.0,
    );
    let velocity = Vec2::new(
        rand::random::<f32>() * 200.0 - 100.0,
        rand::random::<f32>() * 200.0 - 100.0,
    );
    let mass = rand::random::<f32>() * 0.1 + 0.01;
    let color = (
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    );

    Bullet::new(name, position, velocity, mass, color)
}

fn update_simulation(mut world: ResMut<SimulationWorld>) {
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

fn draw_world_bounds(mut gizmos: Gizmos, world: Res<SimulationWorld>) {
    gizmos.rect_2d(
        Isometry2d::IDENTITY,
        bevy::prelude::Vec2::new(world.get_size().0, world.get_size().1),
        Color::WHITE,
    );
}

fn record_bullet_trails(
    world: Res<SimulationWorld>,
    mut query: Query<(&BulletEntity, &mut BulletTrail)>,
) {
    let bullets = world.get_bullets_read();

    for (bullet_entity, mut trail) in &mut query {
        let bullet = &bullets[bullet_entity.index];
        let position = bullet.get_position();

        trail.push(bevy::prelude::Vec2::new(position.x, position.y));
    }
}

fn draw_bullet_trails(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    query: Query<(&BulletEntity, &BulletTrail)>,
) {
    let bullets = world.get_bullets_read();

    for (bullet_entity, trail) in &query {
        if trail.points.len() < 2 {
            continue;
        }

        let bullet = &bullets[bullet_entity.index];
        let color = bullet.get_color();

        gizmos.linestrip_2d(
            trail.points.iter().copied(),
            Color::srgb(color.0, color.1, color.2),
        );
    }
}

#[derive(Component)]
struct SimulationUiText;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                right: Val::Px(12.0),
                width: Val::Px(420.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.02, 0.03, 0.90)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading simulation information..."),
                TextFont::from_font_size(UI_FONT_SIZE),
                TextColor(Color::WHITE),
                SimulationUiText,
            ));
        });
}

fn update_ui(
    world: Res<SimulationWorld>,
    diagnostics: Res<DiagnosticsStore>,
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<&mut Text, With<SimulationUiText>>,
) {
    let physics = world.get_physics();
    let bullets = world.get_bullets_read();

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .unwrap_or(0.0);

    let simulation_time = fixed_time.elapsed_secs_f64();

    let delta_time = physics.get_delta_time();
    let physics_hz = 1.0 / delta_time;

    let mut content = format!(
        "\
SIMULATION
Time: {:.3} s
FPS: {:.1}
Fixed rate: {:.1} Hz

WORLD
Size: {:.0} x {:.0}
Bullets: {}

PHYSICS
Gravity: {:.3} m/s^2
Air resistance: {:.3}
Delta time: {:.6} s
Physics rate: {:.1} Hz
",
        simulation_time,
        fps,
        physics_hz,
        world.get_size().0,
        world.get_size().1,
        bullets.len(),
        physics.get_gravity(),
        physics.get_air_resistance(),
        delta_time,
        physics_hz,
    );

    for mut text in &mut query {
        text.0 = content.clone();
    }
}

fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}
fn spawn_bullet_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    bullet: &Bullet,
    index: usize,
) {
    let radius = bullet.get_radius();

    let color = Color::srgb(
        bullet.get_color().0,
        bullet.get_color().1,
        bullet.get_color().2,
    );

    commands.spawn((
        BulletEntity { index },
        BulletTrail::new(TRAIL_MAX_POINTS),
        Mesh2d(meshes.add(Circle::new(radius)).into()),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            bullet.get_position().x,
            bullet.get_position().y,
            0.0,
        ),
    ));
}

fn regenerate_bullets(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut world: ResMut<SimulationWorld>,
    bullet_entities: Query<Entity, With<BulletEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    for entity in &bullet_entities {
        commands.entity(entity).despawn();
    }

    world.get_bullets().clear();

    for _ in 0..BULLET_COUNT {
        world.add_bullet(generate_random_bullet());
    }

    for (index, bullet) in world.get_bullets_read().iter().enumerate() {
        spawn_bullet_entity(
            &mut commands,
            &mut meshes,
            &mut materials,
            bullet,
            index,
        );
    }
}
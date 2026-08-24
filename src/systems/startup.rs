use bevy::prelude::*;
use bevy::window::PresentMode;

use crate::config::{ AIR_RESISTANCE, BULLET_COUNT, DELTA_TIME, GRAVITY, WORLD_SIZE };
use crate::factories::bullet_factory::generate_random_bullet;

use crate::models::physics::Physics;
use crate::models::wind::Wind;
use crate::models::world::SimulationWorld;
use crate::rendering::bullet_renderer::spawn_bullet_entity;
use crate::resources::shape_library::ShapeLibrary;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    shape_library: Res<ShapeLibrary>
) {
    let physics = Physics::new(
        DELTA_TIME,
        AIR_RESISTANCE,
        GRAVITY,
        Wind::new(Vec2::new(0.5, 1.0), 5.5, true)
    );

    let mut world = SimulationWorld::new(WORLD_SIZE, physics);

    for _ in 0..BULLET_COUNT {
        world.add_bullet(generate_random_bullet(&shape_library));
    }

    commands.spawn(Camera2d);

    for bullet in world.get_bullets_read().iter() {
        spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, &shape_library, bullet);
    }

    commands.insert_resource(world);
}

pub fn resize_window(mut window: Single<&mut Window>) {
    window.resolution.set(WORLD_SIZE.0, WORLD_SIZE.1);
    window.present_mode = PresentMode::AutoNoVsync;
}

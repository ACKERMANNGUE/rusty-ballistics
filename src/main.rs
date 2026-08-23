mod bullet_factory;
mod components;
mod config;
mod loaders;
mod models;
mod systems;
mod resources;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use std::path::PathBuf;

use config::HZ;

use systems::input::{
    regenerate_bullets,
    toggle_pause,
    clear_bullets,
    toggle_wind,
    spawn_bullets_at_mouse_position,
    bullet_launcher_input_system,
};

use systems::rendering::{ draw_bullet_trails, draw_world_bounds, sync_bullet_transforms, display_bullet_hitbox };

use systems::simulation::{
    despawn_orphan_bullet_entities,
    record_bullet_trails,
    update_simulation,
};

use systems::startup::{ resize_window, setup };

use systems::ui::{ setup_ui, update_ui };

use systems::bullet_launcher::BulletLauncher;
use resources::shape_library::ShapeLibrary;

fn shapes_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("assets")
        .join("shapes")
        .join("bullets.json")
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
        .insert_resource(Time::<Fixed>::from_hz(HZ as f64))
        .insert_resource(BulletLauncher::new())
        .insert_resource(ShapeLibrary::load(shapes_path()))
        .add_systems(Startup, (setup, resize_window, setup_ui))
        .add_systems(
            FixedUpdate,
            (update_simulation, despawn_orphan_bullet_entities, record_bullet_trails).chain()
        )
        .add_systems(Update, (
            toggle_pause,
            regenerate_bullets,
            sync_bullet_transforms,
            draw_bullet_trails,
            draw_world_bounds,
            update_ui,
            clear_bullets,
            toggle_wind,
            spawn_bullets_at_mouse_position,
            bullet_launcher_input_system,
        ))
        .add_systems(Update, display_bullet_hitbox)
        .run();
}

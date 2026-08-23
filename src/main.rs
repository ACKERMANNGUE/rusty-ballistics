mod bullet_factory;
mod components;
mod config;
mod models;
mod systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use config::HZ;

use systems::input::{
    regenerate_bullets,
    toggle_pause,
    clear_bullets,
    toggle_wind,
    // spawn_bullet_at_mouse_position,
    spawn_bullets_at_mouse_position,
    bullet_launcher_input_system
};

use systems::rendering::{ draw_bullet_trails, draw_world_bounds, sync_bullet_transforms };

use systems::simulation::{
    despawn_orphan_bullet_entities,
    record_bullet_trails,
    update_simulation,
};

use systems::startup::{ resize_window, setup };

use systems::ui::{ setup_ui, update_ui };

use systems::bullet_launcher::BulletLauncher;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
        .insert_resource(Time::<Fixed>::from_hz(HZ as f64))
        .insert_resource(BulletLauncher::new())
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
            // spawn_bullet_at_mouse_position,
            spawn_bullets_at_mouse_position,
            bullet_launcher_input_system 
        ))
        .run();
}

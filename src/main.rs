mod collision;
mod components;
mod config;
mod factories;
mod geometry;
mod loaders;
mod models;
mod rendering;
mod resources;
mod systems;

use std::path::PathBuf;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::{ EguiPlugin, EguiPrimaryContextPass };

use config::HZ;

use rendering::bullet_renderer::sync_bullet_transforms;

use rendering::debug_renderer::{
    display_bullet_hitbox,
    draw_bullet_trails,
    draw_bullet_triangulation,
    draw_wind_vector,
    draw_world_bounds,
    draw_contact_manifolds,
};

use resources::selected_shape::SelectedShape;
use resources::shape_library::ShapeLibrary;
use resources::bullet_spawn_settings::BulletSpawnSettings;

use systems::bullet_launcher::BulletLauncher;

use systems::camera_controller::{ camera_movement, camera_zoom, clamp_camera_to_world };

use systems::input::{
    bullet_launcher_input_system,
    clear_bullets,
    regenerate_bullets,
    spawn_bullets_at_mouse_position,
    toggle_pause,
    toggle_wind,
};

use systems::simulation::{
    despawn_orphan_bullet_entities,
    record_bullet_trails,
    update_simulation,
};

use systems::startup::{ resize_window, setup };

use systems::ui::simulation_ui;

use bevy::ecs::schedule::common_conditions::not;
use bevy_egui::input::egui_wants_any_pointer_input;

use crate::systems::input::cancel_bullet_launcher_on_egui;

fn shapes_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("assets")
        .join("shapes")
        .join("bullets.json")
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(SelectedShape::new())
        .insert_resource(Time::<Fixed>::from_hz(HZ as f64))
        .insert_resource(BulletLauncher::new())
        .insert_resource(ShapeLibrary::load(shapes_path()))
        .insert_resource(BulletSpawnSettings::new())
        .add_systems(Startup, (setup, resize_window))
        .add_systems(EguiPrimaryContextPass, simulation_ui)
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
            clear_bullets,
            toggle_wind,
            spawn_bullets_at_mouse_position.run_if(not(egui_wants_any_pointer_input)),
            bullet_launcher_input_system.run_if(not(egui_wants_any_pointer_input)),
            cancel_bullet_launcher_on_egui.run_if(egui_wants_any_pointer_input),
        ))
        .add_systems(Update, (
            display_bullet_hitbox,
            draw_wind_vector,
            draw_bullet_triangulation,
            draw_contact_manifolds,
        ))
        .add_systems(
            Update,
            (
                camera_movement,
                camera_zoom.run_if(not(egui_wants_any_pointer_input)),
                clamp_camera_to_world,
            ).chain()
        )
        .run();
}

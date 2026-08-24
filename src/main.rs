mod components;
mod collision;
mod config;
mod factories;
mod geometry;
mod loaders;
mod models;
mod rendering;
mod resources;
mod systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use std::path::PathBuf;

use config::HZ;

use rendering::bullet_renderer::{ sync_bullet_transforms };
use rendering::debug_renderer::{
    display_bullet_hitbox,
    draw_bullet_trails,
    draw_world_bounds,
    draw_wind_vector,
    draw_bullet_triangulation,
};

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

use systems::ui::{ setup_ui, update_ui, shape_selector_ui };

use resources::shape_library::ShapeLibrary;
use systems::bullet_launcher::BulletLauncher;

use crate::systems::camera_controller::{ camera_movement, camera_zoom, clamp_camera_to_world };

use bevy_egui::{ EguiPlugin, EguiPrimaryContextPass };
use crate::resources::selected_shape::SelectedShape;

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

        .add_systems(Startup, (setup, resize_window, setup_ui))

        .add_systems(EguiPrimaryContextPass, shape_selector_ui)

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

        .add_systems(Update, (display_bullet_hitbox, draw_wind_vector, draw_bullet_triangulation))

        .add_systems(Update, (camera_movement, camera_zoom, clamp_camera_to_world).chain())

        .run();
}

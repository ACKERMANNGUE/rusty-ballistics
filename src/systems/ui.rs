use bevy::diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin };
use bevy::prelude::*;
use bevy_egui::{ egui, EguiContexts };

use crate::{
    geometry::mass_properties::compute_mass_properties,
    models::world::SimulationWorld,
    resources::{
        bullet_spawn_settings::BulletSpawnSettings,
        selected_shape::SelectedShape,
        shape_library::ShapeLibrary,
    },
};

const GRID_COLUMN_SPACING: f32 = 20.0;
const GRID_ROW_SPACING: f32 = 6.0;
const SETTINGS_GRID_ROW_SPACING: f32 = 8.0;

const CONTROLS: &[(&str, &str)] = &[
    ("Left mouse and drag", "Launch bullet"),
    ("Right mouse click", "Spawn firework of bullets"),
    ("Space", "Pause / Resume"),
    ("R", "Regenerate"),
    ("C", "Clear"),
    ("Y", "Toggle wind"),
    ("Mouse wheel", "Zoom camera"),
    ("WASD or Arrow keys", "Move camera"),
    ("H", "Toggle debug visuals"),    
];

#[derive(Debug, Clone, Copy)]
struct SimulationUiSnapshot {
    fps: f64,
    simulation_time: f64,
    world_size: (f32, f32),
    bullet_count: usize,
    is_paused: bool,
    gravity: f32,
    air_resistance: f32,
    angular_damping: f32,
    delta_time: f32,
    physics_hz: f32,
    wind_active: bool,
    wind_speed: f32,
    wind_direction: f32,
    wind_turbulence_direction: f32,
}

pub fn simulation_ui(
    mut contexts: EguiContexts,
    mut world: ResMut<SimulationWorld>,
    diagnostics: Res<DiagnosticsStore>,
    fixed_time: Res<Time<Fixed>>,
    virtual_time: Res<Time<Virtual>>,
    shape_library: Res<ShapeLibrary>,
    mut selected_shape: ResMut<SelectedShape>,
    mut spawn_settings: ResMut<BulletSpawnSettings>
) -> Result {
    let context = contexts.ctx_mut()?;

    let snapshot = collect_ui_snapshot(&world, &diagnostics, &fixed_time, &virtual_time);

    egui::Window
        ::new("Rusty Ballistic")
        .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
        .default_width(330.0)
        .resizable(false)
        .show(context, |ui| {
            show_simulation_section(ui, &snapshot);
            show_world_section(ui, &snapshot);
            show_physics_section(ui, &mut world, &snapshot);
            show_wind_section(ui, &snapshot);
            show_bullet_section(ui, &shape_library, &mut selected_shape);
            show_bullet_spawn_settings_section(
                ui,
                &shape_library,
                &selected_shape,
                &mut spawn_settings
            );
            show_controls_section(ui);
        });

    Ok(())
}

fn collect_ui_snapshot(
    world: &SimulationWorld,
    diagnostics: &DiagnosticsStore,
    fixed_time: &Time<Fixed>,
    virtual_time: &Time<Virtual>
) -> SimulationUiSnapshot {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .unwrap_or(0.0);

    let simulation_time = fixed_time.elapsed_secs_f64();
    let world_size = world.get_size();
    let bullet_count = world.get_bullets().len();
    let physics = world.get_physics();
    let wind = physics.get_wind();
    let gravity = physics.get_gravity();
    let air_resistance = physics.get_air_resistance();
    let angular_damping = physics.get_angular_damping();
    let delta_time = physics.get_delta_time();
    let physics_hz = if delta_time > 0.0 { 1.0 / delta_time } else { 0.0 };
    let wind_active = wind.is_active();
    let wind_speed = wind.get_speed();
    let wind_direction = wind.get_direction_degrees();
    let wind_turbulence_direction = wind.get_turbulence_direction_degrees();

    SimulationUiSnapshot {
        fps,
        simulation_time,
        world_size,
        bullet_count,
        is_paused: virtual_time.is_paused(),
        gravity,
        air_resistance,
        angular_damping,
        delta_time,
        physics_hz,
        wind_active,
        wind_speed,
        wind_direction,
        wind_turbulence_direction,
    }
}

fn show_simulation_section(ui: &mut egui::Ui, snapshot: &SimulationUiSnapshot) {
    ui.heading("Simulation");
    ui.add_space(4.0);

    show_two_column_grid(ui, "simulation_info_grid", GRID_ROW_SPACING, |ui| {
        ui.label("Status");
        ui.label(if snapshot.is_paused { "Paused" } else { "Running" });
        ui.end_row();
        ui.label("Time");
        ui.label(format!("{:.3} s", snapshot.simulation_time));
        ui.end_row();
        ui.label("FPS");
        ui.label(format!("{:.1}", snapshot.fps));
        ui.end_row();
        ui.label("Physics rate");
        ui.label(format!("{:.1} Hz", snapshot.physics_hz));
        ui.end_row();
    });
}

fn show_world_section(ui: &mut egui::Ui, snapshot: &SimulationUiSnapshot) {
    show_section_title(ui, "World");
    show_two_column_grid(ui, "world_info_grid", GRID_ROW_SPACING, |ui| {
        ui.label("Width");
        ui.label(format!("{:.0}", snapshot.world_size.0));
        ui.end_row();
        ui.label("Height");
        ui.label(format!("{:.0}", snapshot.world_size.1));
        ui.end_row();
        ui.label("Bullets");
        ui.label(snapshot.bullet_count.to_string());
        ui.end_row();
    });
}

fn show_physics_section(
    ui: &mut egui::Ui,
    world: &mut SimulationWorld,
    snapshot: &SimulationUiSnapshot
) {
    show_section_title(ui, "Physics");
    show_two_column_grid(ui, "physics_info_grid", GRID_ROW_SPACING, |ui| {
        ui.label("Gravity");
        ui.label(format!("{:.3} m/s^2", snapshot.gravity));
        ui.end_row();
        ui.label("Air resistance");
        ui.label(format!("{:.3}", snapshot.air_resistance));
        ui.end_row();

        let mut angular_damping_value = snapshot.angular_damping;

        ui.label("Angular damping");
        if
            ui
                .add(egui::Slider::new(&mut angular_damping_value, 0.0..=2.0).suffix(" s⁻¹"))
                .changed()
        {
            world.get_physics_mut().set_angular_damping(angular_damping_value);
        }

        ui.end_row();
        ui.label("Delta time");
        ui.label(format!("{:.6} s", snapshot.delta_time));
        ui.end_row();
        ui.label("Physics rate");
        ui.label(format!("{:.1} Hz", snapshot.physics_hz));
        ui.end_row();
    });
}

fn show_wind_section(ui: &mut egui::Ui, snapshot: &SimulationUiSnapshot) {
    show_section_title(ui, "Wind");
    show_two_column_grid(ui, "wind_info_grid", GRID_ROW_SPACING, |ui| {
        ui.label("Active");
        ui.label(if snapshot.wind_active { "Yes" } else { "No" });
        ui.end_row();
        ui.label("Speed");
        ui.label(format!("{:.3} m/s", snapshot.wind_speed));
        ui.end_row();
        ui.label("Direction");
        ui.label(format!("{:.1}°", snapshot.wind_direction));
        ui.end_row();
        ui.label("Turbulence direction");
        ui.label(format!("{:.1}°", snapshot.wind_turbulence_direction));
        ui.end_row();
    });
}

fn show_bullet_section(
    ui: &mut egui::Ui,
    shape_library: &ShapeLibrary,
    selected_shape: &mut SelectedShape
) {
    show_section_title(ui, "Bullet");
    egui::ComboBox
        ::from_label("Shape")
        .selected_text(selected_shape.get_shape_name())
        .show_ui(ui, |ui| {
            for shape_name in shape_library.get_shape_names() {
                let is_selected = selected_shape.get_shape_name() == shape_name;
                if ui.selectable_label(is_selected, shape_name).clicked() {
                    selected_shape.set_shape_name(shape_name.to_string());
                }
            }
        });
}

fn show_bullet_spawn_settings_section(
    ui: &mut egui::Ui,
    shape_library: &ShapeLibrary,
    selected_shape: &SelectedShape,
    spawn_settings: &mut BulletSpawnSettings
) {
    show_section_title(ui, "Bullet Spawn Settings");
    show_two_column_grid(ui, "bullet_spawn_settings_grid", SETTINGS_GRID_ROW_SPACING, |ui| {
        show_size_setting(ui, spawn_settings);
        show_density_setting(ui, spawn_settings);
        show_mass_properties(ui, shape_library, selected_shape, spawn_settings);
        show_restitution_setting(ui, spawn_settings);
        show_static_friction_setting(ui, spawn_settings);
        show_dynamic_friction_setting(ui, spawn_settings);
    });
}

fn show_size_setting(ui: &mut egui::Ui, spawn_settings: &mut BulletSpawnSettings) {
    let mut size = spawn_settings.get_size();

    ui.label("Size");

    if ui.add(egui::Slider::new(&mut size, 0.1..=100.0)).changed() {
        spawn_settings.set_size(size);
    }

    ui.end_row();
}

fn show_density_setting(ui: &mut egui::Ui, spawn_settings: &mut BulletSpawnSettings) {
    let mut density = spawn_settings.get_density();

    ui.label("Density");

    if ui.add(egui::Slider::new(&mut density, 0.1..=10.0)).changed() {
        spawn_settings.set_density(density);
    }

    ui.end_row();
}

fn show_mass_properties(
    ui: &mut egui::Ui,
    shape_library: &ShapeLibrary,
    selected_shape: &SelectedShape,
    spawn_settings: &BulletSpawnSettings
) {
    let selected_shape_name = selected_shape.get_shape_name();

    let Some(shape) = shape_library.get(selected_shape_name) else {
        return;
    };

    let local_area = shape.get_area();

    let mass_properties = compute_mass_properties(
        shape,
        spawn_settings.get_size(),
        spawn_settings.get_density()
    );

    ui.label("Local area");
    ui.label(format!("{local_area:.3}"));
    ui.end_row();
    ui.label("Scaled area");
    ui.label(format!("{:.3}", mass_properties.get_scaled_area()));
    ui.end_row();
    ui.label("Mass");
    ui.label(format!("{:.3}", mass_properties.get_mass()));
    ui.end_row();
    ui.label("Moment of inertia");
    ui.label(format!("{:.3}", mass_properties.get_moment_of_inertia()));
    ui.end_row();
}

fn show_restitution_setting(ui: &mut egui::Ui, spawn_settings: &mut BulletSpawnSettings) {
    let mut restitution = spawn_settings.get_restitution();

    ui.label("Restitution");

    if ui.add(egui::Slider::new(&mut restitution, 0.0..=1.0)).changed() {
        spawn_settings.set_restitution(restitution);
    }

    ui.end_row();
}

fn show_static_friction_setting(ui: &mut egui::Ui, spawn_settings: &mut BulletSpawnSettings) {
    let mut static_friction = spawn_settings.get_static_friction();

    ui.label("Static friction");

    if ui.add(egui::Slider::new(&mut static_friction, 0.0..=2.0)).changed() {
        spawn_settings.set_static_friction(static_friction);

        if spawn_settings.get_dynamic_friction() > static_friction {
            spawn_settings.set_dynamic_friction(static_friction);
        }
    }

    ui.end_row();
}

fn show_dynamic_friction_setting(ui: &mut egui::Ui, spawn_settings: &mut BulletSpawnSettings) {
    let maximum_dynamic_friction = spawn_settings.get_static_friction();
    let mut dynamic_friction = spawn_settings.get_dynamic_friction().min(maximum_dynamic_friction);

    ui.label("Dynamic friction");

    if ui.add(egui::Slider::new(&mut dynamic_friction, 0.0..=maximum_dynamic_friction)).changed() {
        spawn_settings.set_dynamic_friction(dynamic_friction);
    }

    ui.end_row();
}

fn show_controls_section(ui: &mut egui::Ui) {
    show_section_title(ui, "Controls");
    show_two_column_grid(ui, "controls_grid", GRID_ROW_SPACING, |ui| {
        for &(input, action) in CONTROLS {
            ui.label(input);
            ui.label(action);
            ui.end_row();
        }
    });
}

fn show_section_title(ui: &mut egui::Ui, title: &str) {
    ui.separator();
    ui.heading(title);
    ui.add_space(4.0);
}

fn show_two_column_grid(
    ui: &mut egui::Ui,
    id: &'static str,
    row_spacing: f32,
    add_contents: impl FnOnce(&mut egui::Ui)
) {
    egui::Grid
        ::new(id)
        .num_columns(2)
        .spacing([GRID_COLUMN_SPACING, row_spacing])
        .show(ui, add_contents);
}

use bevy::diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin };
use bevy::prelude::*;
use bevy_egui::{ egui, EguiContexts };

use crate::models::world::SimulationWorld;
use crate::resources::selected_shape::SelectedShape;
use crate::resources::shape_library::ShapeLibrary;

pub fn simulation_ui(
    mut contexts: EguiContexts,
    world: Res<SimulationWorld>,
    diagnostics: Res<DiagnosticsStore>,
    fixed_time: Res<Time<Fixed>>,
    virtual_time: Res<Time<Virtual>>,
    shape_library: Res<ShapeLibrary>,
    mut selected_shape: ResMut<SelectedShape>
) -> Result {
    let context = contexts.ctx_mut()?;

    let physics = world.get_physics();
    let bullets = world.get_bullets_read();

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .unwrap_or(0.0);

    let simulation_time = fixed_time.elapsed_secs_f64();

    let delta_time = physics.get_delta_time();
    let physics_hz = 1.0 / delta_time;

    let world_size = world.get_size();

    let wind = physics.get_wind();

    egui::Window
        ::new("Rusty Ballistic")
        .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
        .default_width(330.0)
        .resizable(false)
        .show(context, |ui| {
            ui.heading("Simulation");

            ui.add_space(4.0);

            egui::Grid
                ::new("simulation_info_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Status");
                    ui.label(if virtual_time.is_paused() { "Paused" } else { "Running" });
                    ui.end_row();

                    ui.label("Time");
                    ui.label(format!("{simulation_time:.3} s"));
                    ui.end_row();

                    ui.label("FPS");
                    ui.label(format!("{fps:.1}"));
                    ui.end_row();

                    ui.label("Physics rate");
                    ui.label(format!("{physics_hz:.1} Hz"));
                    ui.end_row();
                });

            ui.separator();
            ui.heading("World");
            ui.add_space(4.0);

            egui::Grid
                ::new("world_info_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Width");
                    ui.label(format!("{:.0}", world_size.0));
                    ui.end_row();

                    ui.label("Height");
                    ui.label(format!("{:.0}", world_size.1));
                    ui.end_row();

                    ui.label("Bullets");
                    ui.label(bullets.len().to_string());
                    ui.end_row();
                });

            ui.separator();
            ui.heading("Physics");
            ui.add_space(4.0);

            egui::Grid
                ::new("physics_info_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Gravity");
                    ui.label(format!("{:.3} m/s²", physics.get_gravity()));
                    ui.end_row();

                    ui.label("Air resistance");
                    ui.label(format!("{:.3}", physics.get_air_resistance()));
                    ui.end_row();

                    ui.label("Delta time");
                    ui.label(format!("{delta_time:.6} s"));
                    ui.end_row();

                    ui.label("Physics rate");
                    ui.label(format!("{physics_hz:.1} Hz"));
                    ui.end_row();
                });

            ui.separator();
            ui.heading("Wind");
            ui.add_space(4.0);

            egui::Grid
                ::new("wind_info_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Active");
                    ui.label(if wind.is_active() { "Yes" } else { "No" });
                    ui.end_row();

                    ui.label("Speed");
                    ui.label(format!("{:.3} m/s", wind.get_speed()));
                    ui.end_row();

                    ui.label("Direction");
                    ui.label(format!("{:.1}°", wind.get_direction_degrees()));
                    ui.end_row();

                    ui.label("Turbulence direction");
                    ui.label(format!("{:.1}°", wind.get_turbulence_direction_degrees()));
                    ui.end_row();
                });

            ui.separator();
            ui.heading("Bullet");
            ui.add_space(4.0);

            egui::ComboBox
                ::from_label("Shape")
                .selected_text(selected_shape.get_shape_name())
                .show_ui(ui, |ui| {
                    for shape_name in shape_library.get_shape_names() {
                        let is_selected = selected_shape.get_shape_name() == &shape_name;

                        if ui.selectable_label(is_selected, &shape_name).clicked() {
                            selected_shape.set_shape_name(shape_name);
                        }
                    }
                });

            ui.separator();
            ui.heading("Controls");
            ui.add_space(4.0);

            egui::Grid
                ::new("controls_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Left mouse and drag");
                    ui.label("Launch bullet");
                    ui.end_row();

                    ui.label("Right mouse click");
                    ui.label("Spawn firework of bullets");
                    ui.end_row();

                    ui.label("Space");
                    ui.label("Pause / Resume");
                    ui.end_row();

                    ui.label("R");
                    ui.label("Regenerate");
                    ui.end_row();

                    ui.label("C");
                    ui.label("Clear");
                    ui.end_row();

                    ui.label("Y");
                    ui.label("Toggle wind");
                    ui.end_row();

                    ui.label("Mouse wheel");
                    ui.label("Zoom camera");
                    ui.end_row();

                    ui.label("WASD or Arrow keys");
                    ui.label("Move camera");
                    ui.end_row();
                });
        });

    Ok(())
}

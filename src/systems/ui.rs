use bevy::diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin };
use bevy::prelude::*;
use bevy_egui::{ egui, EguiContexts };

use crate::models::world::SimulationWorld;
use crate::resources::bullet_spawn_settings::BulletSpawnSettings;
use crate::resources::selected_shape::SelectedShape;
use crate::resources::shape_library::ShapeLibrary;

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

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .unwrap_or(0.0);

    let simulation_time = fixed_time.elapsed_secs_f64();

    let world_size = world.get_size();
    let bullet_count = world.get_bullets_read().len();

    // extracts the physics parameters in order to avoid the borrow checking
    let (
        gravity,
        air_resistance,
        angular_damping,
        delta_time,
        wind_active,
        wind_speed,
        wind_direction,
        wind_turbulence_direction,
    ) = {
        let physics = world.get_physics();
        let wind = physics.get_wind();

        (
            physics.get_gravity(),
            physics.get_air_resistance(),
            physics.get_angular_damping(),
            physics.get_delta_time(),
            wind.is_active(),
            wind.get_speed(),
            wind.get_direction_degrees(),
            wind.get_turbulence_direction_degrees(),
        )
    };

    let physics_hz = 1.0 / delta_time;

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
                    ui.label(bullet_count.to_string());
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
                    ui.label(format!("{gravity:.3} m/s^2"));
                    ui.end_row();

                    ui.label("Air resistance");
                    ui.label(format!("{air_resistance:.3}"));
                    ui.end_row();

                    let mut angular_damping_value = angular_damping;

                    ui.label("Angular damping");

                    if
                        ui
                            .add(
                                egui::Slider
                                    ::new(&mut angular_damping_value, 0.0..=2.0)
                                    .suffix(" s⁻¹")
                            )
                            .changed()
                    {
                        world.get_physics_mut().set_angular_damping(angular_damping_value);
                    }

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
                    ui.label(if wind_active { "Yes" } else { "No" });
                    ui.end_row();

                    ui.label("Speed");
                    ui.label(format!("{:.3} m/s", wind_speed));
                    ui.end_row();

                    ui.label("Direction");
                    ui.label(format!("{:.1}°", wind_direction));
                    ui.end_row();

                    ui.label("Turbulence direction");
                    ui.label(format!("{:.1}°", wind_turbulence_direction));
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
            ui.heading("Bullet Spawn Settings");
            ui.add_space(4.0);

            egui::Grid
                ::new("bullet_spawn_settings_grid")
                .num_columns(2)
                .spacing([20.0, 8.0])
                .show(ui, |ui| {
                    let mut size = spawn_settings.get_size();

                    ui.label("Size");

                    if ui.add(egui::Slider::new(&mut size, 0.1..=100.0)).changed() {
                        spawn_settings.set_size(size);
                    }

                    ui.end_row();

                    let mut density = spawn_settings.get_density();

                    ui.label("Density");

                    if ui.add(egui::Slider::new(&mut density, 0.1..=10.0)).changed() {
                        spawn_settings.set_density(density);
                    }

                    ui.end_row();

                    let selected_shape_name = selected_shape.get_shape_name();

                    if let Some(shape) = shape_library.get(selected_shape_name) {
                        let local_area = shape.get_area();

                        let scaled_area = local_area * spawn_settings.get_size().powi(2);

                        let derived_mass = spawn_settings.get_density() * scaled_area;

                        let moment_of_inertia =
                            derived_mass *
                            spawn_settings.get_size().powi(2) *
                            shape.get_inertia_factor();

                        ui.label("Local area");
                        ui.label(format!("{local_area:.3}"));
                        ui.end_row();

                        ui.label("Scaled area");
                        ui.label(format!("{scaled_area:.3}"));
                        ui.end_row();

                        ui.label("Mass");
                        ui.label(format!("{derived_mass:.3}"));
                        ui.end_row();

                        ui.label("Moment of inertia");
                        ui.label(format!("{moment_of_inertia:.3}"));
                        ui.end_row();
                    }

                    let mut restitution = spawn_settings.get_restitution();

                    ui.label("Restitution");

                    if ui.add(egui::Slider::new(&mut restitution, 0.0..=1.0)).changed() {
                        spawn_settings.set_restitution(restitution);
                    }

                    ui.end_row();

                    let mut static_friction = spawn_settings.get_static_friction();

                    ui.label("Static friction");

                    if ui.add(egui::Slider::new(&mut static_friction, 0.0..=2.0)).changed() {
                        spawn_settings.set_static_friction(static_friction);

                        if spawn_settings.get_dynamic_friction() > static_friction {
                            spawn_settings.set_dynamic_friction(static_friction);
                        }
                    }

                    ui.end_row();

                    let maximum_dynamic_friction = spawn_settings.get_static_friction();

                    let mut dynamic_friction = spawn_settings
                        .get_dynamic_friction()
                        .min(maximum_dynamic_friction);

                    ui.label("Dynamic friction");

                    if
                        ui
                            .add(
                                egui::Slider::new(
                                    &mut dynamic_friction,
                                    0.0..=maximum_dynamic_friction
                                )
                            )
                            .changed()
                    {
                        spawn_settings.set_dynamic_friction(dynamic_friction);
                    }

                    ui.end_row();
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

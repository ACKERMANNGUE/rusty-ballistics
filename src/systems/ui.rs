use bevy::diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin };

use bevy::prelude::*;

use crate::config::UI_FONT_SIZE;
use crate::models::world::SimulationWorld;

#[derive(Component)]
pub struct SimulationUiText;

pub fn setup_ui(mut commands: Commands) {
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
            BackgroundColor(Color::srgba(0.02, 0.02, 0.03, 0.9)),
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

pub fn update_ui(
    world: Res<SimulationWorld>,
    diagnostics: Res<DiagnosticsStore>,
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<&mut Text, With<SimulationUiText>>
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
Wind active ? {}
Wind: {:.3} m/s
Wind direction: {:.1} degrees
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
        physics.get_wind().is_active(),
        physics.get_wind().get_speed(),
        physics.get_wind().get_direction_degrees()
    );

    for mut text in &mut query {
        text.0 = content.clone();
    }
}

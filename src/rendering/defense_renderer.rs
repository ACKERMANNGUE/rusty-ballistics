use bevy::prelude::*;

use crate::{
    defense::{
        defense_system::DefenseSystem,
        interceptor::InterceptorRegistry,
        threat_detection::detect_threats,
    },
    models::world::SimulationWorld,
};

const DEFENSE_POINT_RADIUS: f32 = 20.0;
const THREAT_POINT_RADIUS: f32 = 15.0;
// const PREDICTED_ENTRY_POINT_RADIUS: f32 = 10.0;
const INTERCEPTOR_POINT_RADIUS: f32 = 12.0;

pub fn draw_defense_system(mut gizmos: Gizmos, defense_system: Res<DefenseSystem>) {
    if !defense_system.is_enabled() {
        return;
    }

    let position = defense_system.get_position();

    gizmos.circle_2d(position, DEFENSE_POINT_RADIUS, Color::srgb(1.0, 1.0, 1.0));
    gizmos.circle_2d(position, defense_system.get_protection_radius(), Color::srgb(0.9, 0.68, 0.2));
    gizmos.circle_2d(position, defense_system.get_detection_radius(), Color::srgb(0.3, 0.7, 0.8));
}

pub fn draw_detected_threats(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    defense_system: Res<DefenseSystem>
) {
    let threats = detect_threats(world.get_bullets(), &defense_system);

    for threat in threats {
        let Some(bullet) = world.get_bullet_by_id(threat.get_bullet_id()) else {
            continue;
        };

        let bullet_position = bullet.get_position();
        let predicted_entry_position =
            bullet_position + bullet.get_velocity() * threat.get_time_to_protected_area();

        gizmos.circle_2d(bullet_position, THREAT_POINT_RADIUS, Color::srgb(1.0, 0.1, 0.1));
        gizmos.line_2d(bullet_position, predicted_entry_position, Color::srgb(1.0, 0.2, 0.2));
        // gizmos.circle_2d(
        //     predicted_entry_position,
        //     PREDICTED_ENTRY_POINT_RADIUS,
        //     Color::srgb(1.0, 0.2, 0.2)
        // );
    }
}

pub fn draw_interceptor_targets(
    mut gizmos: Gizmos,
    world: Res<SimulationWorld>,
    interceptor_registry: Res<InterceptorRegistry>
) {
    for interceptor in interceptor_registry.get_interceptors() {
        let Some(interceptor_bullet) = world.get_bullet_by_id(interceptor.get_bullet_id()) else {
            continue;
        };

        let Some(target_bullet) = world.get_bullet_by_id(interceptor.get_target_bullet_id()) else {
            continue;
        };

        let interceptor_position = interceptor_bullet.get_position();

        gizmos.circle_2d(
            interceptor_position,
            INTERCEPTOR_POINT_RADIUS,
            Color::srgb(0.2, 0.7, 1.0)
        );

        gizmos.line_2d(
            interceptor_position,
            target_bullet.get_position(),
            Color::srgb(0.2, 0.7, 1.0)
        );
    }
}

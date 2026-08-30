use bevy::prelude::*;

use crate::{
    config::{ DEFENSE_INTERCEPTION_RADIUS, DEFENSE_INTERCEPTOR_MAX_TURN_RATE, EPSILON }, defense::{
        defense_system::DefenseSystem,
        interceptor::{ Interceptor, InterceptorRegistry },
        threat_detection::detect_threats,
    }, factories::bullet_factory::generate_interceptor, models::world::SimulationWorld, rendering::bullet_renderer::spawn_bullet_entity, resources::shape_library::ShapeLibrary,
};

pub fn update_defense_cooldown(
    fixed_time: Res<Time<Fixed>>,
    mut defense_system: ResMut<DefenseSystem>
) {
    defense_system.update_cooldown(fixed_time.delta_secs());
}

pub fn cleanup_interceptor_registry(
    world: Res<SimulationWorld>,
    mut interceptor_registry: ResMut<InterceptorRegistry>
) {
    interceptor_registry.retain(|interceptor| {
        world.get_bullet_by_id(interceptor.get_bullet_id()).is_some() &&
            world.get_bullet_by_id(interceptor.get_target_bullet_id()).is_some()
    });
}

pub fn launch_interceptor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut world: ResMut<SimulationWorld>,
    mut defense_system: ResMut<DefenseSystem>,
    mut interceptor_registry: ResMut<InterceptorRegistry>,
    shape_library: Res<ShapeLibrary>
) {
    if !defense_system.can_launch_interceptor(interceptor_registry.get_active_count()) {
        return;
    }

    let threats = detect_threats(world.get_bullets(), &defense_system);

    let Some(threat) = threats
        .iter()
        .find(|threat| { !interceptor_registry.is_target_engaged(threat.get_bullet_id()) }) else {
        return;
    };

    let target_bullet_id = threat.get_bullet_id();
    let Some(target_bullet) = world.get_bullet_by_id(target_bullet_id) else {
        return;
    };

    let interceptor_position = defense_system.get_position();
    let direction_to_target = target_bullet.get_position() - interceptor_position;

    if direction_to_target.length_squared() <= EPSILON {
        return;
    }

    let interceptor_velocity =
        direction_to_target.normalize() * defense_system.get_interceptor_speed();

    let interceptor = generate_interceptor(
        interceptor_position,
        interceptor_velocity,
        &shape_library
    );

    let interceptor_bullet_id = interceptor.get_id();

    spawn_bullet_entity(&mut commands, &mut meshes, &mut materials, &shape_library, &interceptor);
    world.add_bullet(interceptor);
    interceptor_registry.add(Interceptor::new(interceptor_bullet_id, target_bullet_id));
    defense_system.start_launch_cooldown();
}

pub fn update_interceptor_guidance(
    fixed_time: Res<Time<Fixed>>,
    mut world: ResMut<SimulationWorld>,
    defense_system: Res<DefenseSystem>,
    interceptor_registry: Res<InterceptorRegistry>
) {
    let delta_time = fixed_time.delta_secs();

    for interceptor in interceptor_registry.get_interceptors() {
        let target_bullet_id = interceptor.get_target_bullet_id();
        let interceptor_bullet_id = interceptor.get_bullet_id();

        let Some(target_bullet) = world.get_bullet_by_id(target_bullet_id) else {
            continue;
        };

        let target_position = target_bullet.get_position();

        let Some(interceptor_bullet) = world.get_bullet_by_id_mut(interceptor_bullet_id) else {
            continue;
        };

        let interceptor_position = interceptor_bullet.get_position();
        let direction_to_target = target_position - interceptor_position;

        if direction_to_target.length_squared() <= EPSILON {
            continue;
        }

        let desired_direction = direction_to_target.normalize();
        let current_velocity = interceptor_bullet.get_velocity();
        let current_direction = if current_velocity.length_squared() > EPSILON {
            current_velocity.normalize()
        } else {
            desired_direction
        };

        let cross = current_direction.perp_dot(desired_direction);
        let dot = current_direction.dot(desired_direction);

        let angle_to_target = cross.atan2(dot);
        let maximum_turn_angle = DEFENSE_INTERCEPTOR_MAX_TURN_RATE * delta_time;

        let turn_angle = angle_to_target.clamp(-maximum_turn_angle, maximum_turn_angle);
        let cos_angle = turn_angle.cos();
        let sin_angle = turn_angle.sin();

        // 2d rotation matrix multiplication to rotate the current direction towards the desired direction
        // https://en.wikipedia.org/wiki/Rotation_matrix#In_two_dimensions
        let new_direction = Vec2::new(
            current_direction.x * cos_angle - current_direction.y * sin_angle,
            current_direction.x * sin_angle + current_direction.y * cos_angle
        ).normalize();

        let interceptor_velocity = new_direction * defense_system.get_interceptor_speed();

        interceptor_bullet.set_velocity(interceptor_velocity);
        interceptor_bullet.set_rotation(interceptor_velocity.to_angle());
        interceptor_bullet.set_angular_velocity(0.0);
    }
}

pub fn resolve_interceptions(
    mut world: ResMut<SimulationWorld>,
    mut interceptor_registry: ResMut<InterceptorRegistry>
) {
    let mut intercepted_pairs = Vec::new();
    for interceptor in interceptor_registry.get_interceptors() {
        let Some(interceptor_bullet) = world.get_bullet_by_id(interceptor.get_bullet_id()) else {
            continue;
        };

        let Some(target_bullet) = world.get_bullet_by_id(interceptor.get_target_bullet_id()) else {
            continue;
        };

        let distance = interceptor_bullet.get_position().distance(target_bullet.get_position());
        if distance <= DEFENSE_INTERCEPTION_RADIUS {
            intercepted_pairs.push((
                interceptor.get_bullet_id(),
                interceptor.get_target_bullet_id(),
            ));
        }
    }

    if intercepted_pairs.is_empty() {
        return;
    }

    for (interceptor_bullet_id, target_bullet_id) in intercepted_pairs {
        if let Some(interceptor_bullet) = world.get_bullet_by_id_mut(interceptor_bullet_id) {
            interceptor_bullet.set_is_dead(true);
        }

        if let Some(target_bullet) = world.get_bullet_by_id_mut(target_bullet_id) {
            target_bullet.set_is_dead(true);
        }
    }

    world.remove_dead_bullets();
    interceptor_registry.retain(|interceptor| {
        world.get_bullet_by_id(interceptor.get_bullet_id()).is_some() &&
            world.get_bullet_by_id(interceptor.get_target_bullet_id()).is_some()
    });
}

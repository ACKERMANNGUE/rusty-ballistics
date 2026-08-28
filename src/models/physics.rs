use bevy::prelude::Vec2;

use crate::geometry::aabb::AABB;
use crate::geometry::projection::project_polygon;
use crate::models::bullet::{ Bullet };
use crate::models::wind::Wind;
use crate::resources::shape_library::ShapeLibrary;

use crate::collision::narrow_phase::detect_collision_manifolds;

use crate::config::{ ANGULAR_VELOCITY_STOP_THRESHOLD, EPSILON };

use crate::collision::contact_manifold::ContactManifold;

use crate::geometry::bullet_shape::{ get_bullet_world_aabb, get_bullet_world_shape };

use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct CellRange {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

pub struct Physics {
    delta_time: f32,
    air_resistance: f32,
    gravity: f32,
    wind: Wind,
    angular_damping: f32,
}

impl Physics {
    pub fn new(
        delta_time: f32,
        air_resistance: f32,
        gravity: f32,
        wind: Wind,
        angular_damping: f32
    ) -> Self {
        Self {
            delta_time,
            air_resistance,
            gravity,
            wind,
            angular_damping,
        }
    }

    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }

    pub fn get_air_resistance(&self) -> f32 {
        self.air_resistance
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_wind(&self) -> &Wind {
        &self.wind
    }

    pub fn get_wind_mut(&mut self) -> &mut Wind {
        &mut self.wind
    }

    pub fn update(
        &mut self,
        bullets: &mut Vec<Bullet>,
        world_size: (f32, f32),
        shape_library: &ShapeLibrary
    ) {
        self.wind.update_turbulence();

        for bullet in bullets.iter_mut() {
            let (new_position, new_velocity, new_rotation, new_angular_velocity) =
                self.compute_new_state(bullet, shape_library);

            bullet.set_position(new_position);
            bullet.set_velocity(new_velocity);
            bullet.set_rotation(new_rotation);
            bullet.set_angular_velocity(new_angular_velocity);

            let Some(aabb) = get_bullet_world_aabb(bullet, shape_library) else {
                println!(
                    "Warning: Cannot compute AABB for bullet {} with shape '{}'.",
                    bullet.get_id(),
                    bullet.get_shape()
                );

                bullet.set_is_dead(true);
                continue;
            };

            if self.is_out_of_bounds(&aabb, world_size) {
                bullet.set_is_dead(true);
            }
        }

        bullets.retain(|bullet| !bullet.is_dead());

        self.compute_collisions(bullets, world_size, shape_library);
    }

    fn compute_new_state(
        &self,
        bullet: &Bullet,
        shape_library: &ShapeLibrary
    ) -> (Vec2, Vec2, f32, f32) {
        let bullet_velocity = *bullet.get_velocity();

        let air_relative_velocity = if self.wind.is_active() {
            let wind_velocity = self.wind.get_direction() * self.wind.get_speed();

            bullet_velocity - wind_velocity - *self.wind.get_turbulence()
        } else {
            bullet_velocity
        };

        let projected_width = self.compute_projected_width(
            bullet,
            air_relative_velocity,
            shape_library
        );

        let reference_width = 1.0;
        let shape_drag_factor = projected_width / reference_width;
        let drag_force = -self.air_resistance * shape_drag_factor * air_relative_velocity;
        let drag_acceleration = drag_force / bullet.get_mass();
        let gravity_acceleration = Vec2::new(0.0, -self.gravity);
        let acceleration = gravity_acceleration + drag_acceleration;

        let new_velocity = bullet_velocity + acceleration * self.delta_time;
        let new_position = *bullet.get_position() + new_velocity * self.delta_time;

        let old_angular_velocity = bullet.get_angular_velocity();
        let new_angular_velocity = self.compute_angular_velocity(old_angular_velocity);
        let rotation = bullet.get_rotation() + new_angular_velocity * self.delta_time;
        (new_position, new_velocity, rotation, new_angular_velocity)
    }

    fn compute_collisions(
        &self,
        bullets: &mut Vec<Bullet>,
        world_size: (f32, f32),
        shape_library: &ShapeLibrary
    ) {
        let grid_cell_size = 100.0;

        let spatial_grid = self.build_spatial_grid(
            bullets,
            world_size,
            grid_cell_size,
            shape_library
        );

        let candidate_pairs = self.build_candidate_pairs(&spatial_grid);

        for (bullet_index, other_index) in candidate_pairs {
            let (left, right) = bullets.split_at_mut(other_index);

            let bullet = &mut left[bullet_index];
            let other_bullet = &mut right[0];

            if bullet.is_dead() || other_bullet.is_dead() {
                continue;
            }

            let manifolds = detect_collision_manifolds(bullet, other_bullet, shape_library);
            for manifold in &manifolds {
                self.compute_collision_response(bullet, other_bullet, manifold);
            }
        }
    }

    fn compute_grid_size(&self, world_size: (f32, f32), cell_size: f32) -> (usize, usize) {
        let grid_width = (world_size.0 / cell_size).ceil() as usize;
        let grid_height = (world_size.1 / cell_size).ceil() as usize;
        (grid_width, grid_height)
    }

    fn compute_projected_width(
        &self,
        bullet: &Bullet,
        relative_velocity: Vec2,
        shape_library: &ShapeLibrary
    ) -> f32 {
        if relative_velocity.length_squared() == 0.0 {
            return 0.0;
        }

        let Some(world_shape) = get_bullet_world_shape(bullet, shape_library) else {
            return 0.0;
        };

        if world_shape.is_empty() {
            return 0.0;
        }

        let direction = relative_velocity.normalize();

        let perpendicular_axis = Vec2::new(-direction.y, direction.x);

        let (min, max) = project_polygon(perpendicular_axis, &world_shape);

        max - min
    }

    fn get_inverse(&self, a: f32) -> f32 {
        if a.abs() < EPSILON { 0.0 } else { 1.0 / a }
    }

    fn get_lever_arm(&self, contact_point: Vec2, position: Vec2) -> Vec2 {
        contact_point - position
    }

    fn get_contact_velocity(&self, velocity: Vec2, angular_velocity: f32, lever_arm: Vec2) -> Vec2 {
        velocity + self.angular_velocity_cross_radius(angular_velocity, lever_arm)
    }

    fn get_impulse_denominator(
        &self,
        inverse_mass1: f32,
        inverse_mass2: f32,
        r1_cross_normal: f32,
        r2_cross_normal: f32,
        inverse_inertia1: f32,
        inverse_inertia2: f32
    ) -> f32 {
        inverse_mass1 +
            inverse_mass2 +
            r1_cross_normal.powi(2) * inverse_inertia1 +
            r2_cross_normal.powi(2) * inverse_inertia2
    }

    fn get_normal_impulse_magnitude(
        &self,
        restitution: f32,
        velocity_along_normal: f32,
        impulse_denominator: f32
    ) -> f32 {
        (-(1.0 + restitution) * velocity_along_normal) / impulse_denominator
    }

    fn compute_collision_response(
        &self,
        bullet1: &mut Bullet,
        bullet2: &mut Bullet,
        manifold: &ContactManifold
    ) {
        let mass1 = bullet1.get_mass();
        let mass2 = bullet2.get_mass();

        if mass1 <= EPSILON || mass2 <= EPSILON {
            return;
        }

        let inverse_mass1 = self.get_inverse(mass1);
        let inverse_mass2 = self.get_inverse(mass2);

        let inverse_mass_sum = inverse_mass1 + inverse_mass2;

        if inverse_mass_sum <= EPSILON {
            return;
        }

        let moment_of_inertia1 = bullet1.get_moment_of_inertia();
        let moment_of_inertia2 = bullet2.get_moment_of_inertia();

        if moment_of_inertia1 <= EPSILON || moment_of_inertia2 <= EPSILON {
            return;
        }

        let inverse_inertia1 = self.get_inverse(moment_of_inertia1);
        let inverse_inertia2 = self.get_inverse(moment_of_inertia2);

        let position1 = *bullet1.get_position();
        let position2 = *bullet2.get_position();

        let normal = manifold.get_normal();
        let contacts = manifold.get_contacts();

        if contacts.is_empty() {
            return;
        }

        let contact_count = contacts.len() as f32;

        let mut new_velocity1 = *bullet1.get_velocity();
        let mut new_velocity2 = *bullet2.get_velocity();

        let mut new_angular_velocity1 = bullet1.get_angular_velocity();
        let mut new_angular_velocity2 = bullet2.get_angular_velocity();

        let restitution = bullet1.get_restitution().min(bullet2.get_restitution());

        let combined_static_friction = (
            bullet1.get_static_friction() * bullet2.get_static_friction()
        ).sqrt();

        let combined_dynamic_friction = (
            bullet1.get_dynamic_friction() * bullet2.get_dynamic_friction()
        ).sqrt();

        for &contact_point in contacts {
            let r1 = self.get_lever_arm(contact_point, position1);
            let r2 = self.get_lever_arm(contact_point, position2);

            let contact_velocity1 = self.get_contact_velocity(
                new_velocity1,
                new_angular_velocity1,
                r1
            );

            let contact_velocity2 = self.get_contact_velocity(
                new_velocity2,
                new_angular_velocity2,
                r2
            );

            let relative_velocity = contact_velocity2 - contact_velocity1;
            let velocity_along_normal = relative_velocity.dot(normal);

            if velocity_along_normal >= 0.0 {
                continue;
            }

            let r1_cross_normal = r1.perp_dot(normal);
            let r2_cross_normal = r2.perp_dot(normal);

            let impulse_denominator = self.get_impulse_denominator(
                inverse_mass1,
                inverse_mass2,
                r1_cross_normal,
                r2_cross_normal,
                inverse_inertia1,
                inverse_inertia2
            );

            if impulse_denominator <= EPSILON {
                continue;
            }

            let normal_impulse_magnitude =
                self.get_normal_impulse_magnitude(
                    restitution,
                    velocity_along_normal,
                    impulse_denominator
                ) / contact_count;

            let normal_impulse = normal * normal_impulse_magnitude;

            new_velocity1 -= normal_impulse * inverse_mass1;
            new_velocity2 += normal_impulse * inverse_mass2;

            new_angular_velocity1 -= r1.perp_dot(normal_impulse) * inverse_inertia1;
            new_angular_velocity2 += r2.perp_dot(normal_impulse) * inverse_inertia2;

            // recompute contact velocities after the normal impulse before calculating friction
            let contact_velocity1_after_normal = self.get_contact_velocity(
                new_velocity1,
                new_angular_velocity1,
                r1
            );

            let contact_velocity2_after_normal = self.get_contact_velocity(
                new_velocity2,
                new_angular_velocity2,
                r2
            );

            let relative_velocity_after_normal =
                contact_velocity2_after_normal - contact_velocity1_after_normal;

            let tangent_velocity =
                relative_velocity_after_normal -
                normal * relative_velocity_after_normal.dot(normal);

            if tangent_velocity.length_squared() <= EPSILON {
                continue;
            }

            let tangent = tangent_velocity.normalize();

            let r1_cross_tangent = r1.perp_dot(tangent);
            let r2_cross_tangent = r2.perp_dot(tangent);

            let friction_denominator = self.get_impulse_denominator(
                inverse_mass1,
                inverse_mass2,
                r1_cross_tangent,
                r2_cross_tangent,
                inverse_inertia1,
                inverse_inertia2
            );

            if friction_denominator <= EPSILON {
                continue;
            }

            let friction_impulse_magnitude =
                -relative_velocity_after_normal.dot(tangent) / friction_denominator / contact_count;
            let maximum_static_friction = normal_impulse_magnitude * combined_static_friction;

            let friction_impulse = if friction_impulse_magnitude.abs() <= maximum_static_friction {
                tangent * friction_impulse_magnitude
            } else {
                tangent * (-normal_impulse_magnitude * combined_dynamic_friction)
            };

            new_velocity1 -= friction_impulse * inverse_mass1;
            new_velocity2 += friction_impulse * inverse_mass2;

            new_angular_velocity1 -= r1.perp_dot(friction_impulse) * inverse_inertia1;
            new_angular_velocity2 += r2.perp_dot(friction_impulse) * inverse_inertia2;
        }

        bullet1.set_velocity(new_velocity1);
        bullet2.set_velocity(new_velocity2);

        bullet1.set_angular_velocity(new_angular_velocity1);
        bullet2.set_angular_velocity(new_angular_velocity2);

        self.correct_penetration(
            bullet1,
            bullet2,
            &manifold,
            inverse_mass1,
            inverse_mass2,
            inverse_mass_sum
        );
    }

    fn build_spatial_grid(
        &self,
        bullets: &[Bullet],
        world_size: (f32, f32),
        cell_size: f32,
        shape_library: &ShapeLibrary
    ) -> Vec<Vec<usize>> {
        let (grid_width, grid_height) = self.compute_grid_size(world_size, cell_size);

        let mut grid = vec![
            Vec::new();
            grid_width * grid_height
        ];

        for (bullet_index, bullet) in bullets.iter().enumerate() {
            let Some(aabb) = get_bullet_world_aabb(bullet, shape_library) else {
                continue;
            };

            let cell_range = self.compute_aabb_cell_range(
                &aabb,
                world_size,
                cell_size,
                grid_width,
                grid_height
            );

            for y in cell_range.min_y..=cell_range.max_y {
                for x in cell_range.min_x..=cell_range.max_x {
                    let cell_index = y * grid_width + x;

                    grid[cell_index].push(bullet_index);
                }
            }
        }

        grid
    }

    fn compute_aabb_cell_range(
        &self,
        aabb: &AABB,
        world_size: (f32, f32),
        cell_size: f32,
        grid_width: usize,
        grid_height: usize
    ) -> CellRange {
        let half_width = world_size.0 * 0.5;
        let half_height = world_size.1 * 0.5;

        let min = aabb.get_min();
        let max = aabb.get_max();

        let min_x = ((min.x + half_width) / cell_size).floor() as isize;
        let max_x = ((max.x + half_width) / cell_size).floor() as isize;
        let min_y = ((min.y + half_height) / cell_size).floor() as isize;
        let max_y = ((max.y + half_height) / cell_size).floor() as isize;

        CellRange {
            min_x: min_x.clamp(0, (grid_width as isize) - 1) as usize,
            max_x: max_x.clamp(0, (grid_width as isize) - 1) as usize,
            min_y: min_y.clamp(0, (grid_height as isize) - 1) as usize,
            max_y: max_y.clamp(0, (grid_height as isize) - 1) as usize,
        }
    }

    fn build_candidate_pairs(&self, spatial_grid: &[Vec<usize>]) -> HashSet<(usize, usize)> {
        let mut pairs = HashSet::new();

        for cell in spatial_grid {
            for first_index in 0..cell.len() {
                for second_index in first_index + 1..cell.len() {
                    let bullet_a = cell[first_index];
                    let bullet_b = cell[second_index];

                    let pair = if bullet_a < bullet_b {
                        (bullet_a, bullet_b)
                    } else {
                        (bullet_b, bullet_a)
                    };

                    pairs.insert(pair);
                }
            }
        }

        pairs
    }

    fn is_out_of_bounds(&self, aabb: &AABB, world_size: (f32, f32)) -> bool {
        let (half_width, half_height) = (world_size.0 * 0.5, world_size.1 * 0.5);
        aabb.is_outside_bounds(half_width, half_height)
    }

    fn correct_penetration(
        &self,
        bullet1: &mut Bullet,
        bullet2: &mut Bullet,
        manifold: &ContactManifold,
        inverse_mass1: f32,
        inverse_mass2: f32,
        inverse_mass_sum: f32
    ) {
        if inverse_mass_sum <= EPSILON {
            return;
        }

        const PENETRATION_SLOP: f32 = 0.01;
        const CORRECTION_PERCENTAGE: f32 = 0.8;

        let penetration_depth = manifold.get_penetration_depth();

        let correction_magnitude =
            ((penetration_depth - PENETRATION_SLOP).max(0.0) / inverse_mass_sum) *
            CORRECTION_PERCENTAGE;

        if correction_magnitude <= 0.0 {
            return;
        }

        let correction = manifold.get_normal() * correction_magnitude;

        let position1 = *bullet1.get_position();
        let position2 = *bullet2.get_position();

        bullet1.set_position(position1 - correction * inverse_mass1);
        bullet2.set_position(position2 + correction * inverse_mass2);
    }

    fn angular_velocity_cross_radius(&self, angular_velocity: f32, radius: Vec2) -> Vec2 {
        Vec2::new(-angular_velocity * radius.y, angular_velocity * radius.x)
    }

    pub fn get_angular_damping(&self) -> f32 {
        self.angular_damping
    }

    pub fn set_angular_damping(&mut self, angular_damping: f32) {
        self.angular_damping = angular_damping;
    }

    pub fn compute_angular_velocity(&self, angular_velocity: f32) -> f32 {
        let damping_factor = (-self.angular_damping * self.delta_time).exp();
        let new_angular_velocity = angular_velocity * damping_factor;

        if new_angular_velocity.abs() < ANGULAR_VELOCITY_STOP_THRESHOLD {
            0.0
        } else {
            new_angular_velocity
        }
    }
}

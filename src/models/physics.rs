use bevy::prelude::Vec2;

use crate::collision::broad_phase::build_candidate_pairs;
use crate::collision::solver::solve_collision_manifolds;
use crate::geometry::aabb::AABB;
use crate::geometry::projection::project_polygon;
use crate::geometry::vector::perpendicular;
use crate::models::bullet::Bullet;
use crate::models::wind::Wind;
use crate::resources::shape_library::ShapeLibrary;

use crate::collision::narrow_phase::detect_collision_manifolds;

use crate::config::ANGULAR_VELOCITY_STOP_THRESHOLD;

use crate::geometry::bullet_shape::{get_bullet_world_aabb, get_bullet_world_shape};

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
        angular_damping: f32,
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
        shape_library: &ShapeLibrary,
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
        shape_library: &ShapeLibrary,
    ) -> (Vec2, Vec2, f32, f32) {
        let bullet_velocity = *bullet.get_velocity();

        let air_relative_velocity = if self.wind.is_active() {
            let wind_velocity = self.wind.get_direction() * self.wind.get_speed();

            bullet_velocity - wind_velocity - *self.wind.get_turbulence()
        } else {
            bullet_velocity
        };

        let projected_width =
            self.compute_projected_width(bullet, air_relative_velocity, shape_library);

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
        shape_library: &ShapeLibrary,
    ) {
        let candidate_pairs = build_candidate_pairs(bullets, world_size, shape_library);

        for (bullet_index, other_index) in candidate_pairs {
            let (left, right) = bullets.split_at_mut(other_index);

            let bullet = &mut left[bullet_index];
            let other_bullet = &mut right[0];

            if bullet.is_dead() || other_bullet.is_dead() {
                continue;
            }

            let manifolds = detect_collision_manifolds(bullet, other_bullet, shape_library);

            if manifolds.is_empty() {
                continue;
            }

            solve_collision_manifolds(bullet, other_bullet, &manifolds);
        }
    }

    fn compute_projected_width(
        &self,
        bullet: &Bullet,
        relative_velocity: Vec2,
        shape_library: &ShapeLibrary,
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
        let perpendicular_axis = perpendicular(direction);
        let (min, max) = project_polygon(perpendicular_axis, &world_shape);

        max - min
    }

    fn is_out_of_bounds(&self, aabb: &AABB, world_size: (f32, f32)) -> bool {
        let (half_width, half_height) = (world_size.0 * 0.5, world_size.1 * 0.5);
        aabb.is_outside_bounds(half_width, half_height)
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

use std::f32::EPSILON;

use bevy::prelude::Vec2;

use crate::collision::collision_info::CollisionInfo;
use crate::geometry::projection::project_polygon;
use crate::models::bullet::{ Bullet };
use crate::models::wind::Wind;
use crate::resources::shape_library::ShapeLibrary;
use crate::geometry::bullet_shape::{ get_bullet_world_shape, get_bullet_world_triangles };
use crate::collision::separating_axis_theorem::{ check_triangles_collision };

pub struct Physics {
    delta_time: f32,
    air_resistance: f32,
    gravity: f32,
    wind: Wind,
}

impl Physics {
    pub fn new(delta_time: f32, air_resistance: f32, gravity: f32, wind: Wind) -> Self {
        Self {
            delta_time,
            air_resistance,
            gravity,
            wind,
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
            let (new_position, new_velocity, rotation) = self.compute_new_state(
                bullet,
                shape_library
            );

            if self.is_out_of_bounds(&new_position, world_size, bullet.get_size()) {
                bullet.set_is_dead(true);
                continue;
            }

            bullet.set_position(new_position);
            bullet.set_velocity(new_velocity);
            bullet.set_rotation(rotation);
        }

        bullets.retain(|bullet| !bullet.is_dead());

        self.compute_collisions(bullets, world_size, shape_library);
    }

    fn compute_new_state(
        &self,
        bullet: &Bullet,
        shape_library: &ShapeLibrary
    ) -> (Vec2, Vec2, f32) {
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
        let rotation = bullet.get_rotation() + bullet.get_angular_velocity() * self.delta_time;
        (new_position, new_velocity, rotation)
    }

    fn compute_collisions(
        &self,
        bullets: &mut Vec<Bullet>,
        world_size: (f32, f32),
        shape_library: &ShapeLibrary
    ) {
        let grid_cell_size = 100.0;
        let spatial_grid = self.build_spatial_grid(bullets, world_size, grid_cell_size);
        let (grid_width, grid_height) = self.compute_grid_size(world_size, grid_cell_size);

        for bullet_index in 0..bullets.len() {
            let position = *bullets[bullet_index].get_position();
            let (x_index, y_index) = self.compute_x_y_indices(
                &position,
                world_size,
                grid_cell_size
            );

            self.check_collisions_in_neighbours(
                bullet_index,
                x_index,
                y_index,
                grid_width,
                grid_height,
                &spatial_grid,
                bullets,
                shape_library
            );
        }
    }

    fn compute_x_y_indices(
        &self,
        position: &Vec2,
        world_size: (f32, f32),
        cell_size: f32
    ) -> (isize, isize) {
        let x_index = ((position.x + world_size.0 / 2.0) / cell_size).floor() as isize;
        let y_index = ((position.y + world_size.1 / 2.0) / cell_size).floor() as isize;
        (x_index, y_index)
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

    fn check_collisions_in_neighbours(
        &self,
        bullet_index: usize,
        x_index: isize,
        y_index: isize,
        grid_width: usize,
        grid_height: usize,
        spatial_grid: &Vec<Vec<usize>>,
        bullets: &mut Vec<Bullet>,
        shape_library: &ShapeLibrary
    ) {
        // use of isize for x_index and y_index allows us to check neighboring cells without worrying about underflow when subtracting 1
        // compared to using usize which is unsigned and would underflow when subtracting 1 from 0
        for dy in -1isize..=1 {
            for dx in -1isize..=1 {
                let neighbor_x = x_index + dx;
                let neighbor_y = y_index + dy;

                if neighbor_x < 0 || neighbor_y < 0 {
                    continue;
                }

                if (neighbor_x as usize) >= grid_width || (neighbor_y as usize) >= grid_height {
                    continue;
                }

                let cell_index = (neighbor_y as usize) * grid_width + (neighbor_x as usize);

                for &other_index in &spatial_grid[cell_index] {
                    // Avoid self-collision and checking the same pair twice
                    if other_index <= bullet_index {
                        continue;
                    }

                    let (left, right) = bullets.split_at_mut(other_index);

                    let bullet = &mut left[bullet_index];
                    let other_bullet = &mut right[0];

                    if bullet.is_dead() || other_bullet.is_dead() {
                        continue;
                    }

                    let Some(bullet_world_triangles) = get_bullet_world_triangles(
                        bullet,
                        shape_library
                    ) else {
                        println!(
                            "Warning: Shape '{}' not found in shape library.",
                            bullet.get_shape()
                        );
                        continue;
                    };

                    let Some(other_bullet_world_triangles) = get_bullet_world_triangles(
                        other_bullet,
                        shape_library
                    ) else {
                        println!(
                            "Warning: Shape '{}' not found in shape library.",
                            other_bullet.get_shape()
                        );
                        continue;
                    };

                    let Some(collision_info) = check_triangles_collision(
                        &bullet_world_triangles,
                        &other_bullet_world_triangles
                    ) else {
                        continue;
                    };

                    self.compute_collision_response(bullet, other_bullet, collision_info);
                }
            }
        }
    }

    fn get_inverse(&self, a: f32) -> f32 {
        if a.abs() < EPSILON {
            0.0
        } else {
            1.0 / a
        }
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
        inverse_mass1 + inverse_mass2 +
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
        collision_info: CollisionInfo
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

        let normal = collision_info.get_normal();
        let contact_point = collision_info.get_contact_point();

        let position1 = *bullet1.get_position();
        let position2 = *bullet2.get_position();

        // lever arms from each center of mass to the contact point
        let r1 = self.get_lever_arm(contact_point, position1);
        let r2 = self.get_lever_arm(contact_point, position2);

        let velocity1 = *bullet1.get_velocity();
        let velocity2 = *bullet2.get_velocity();

        let angular_velocity1 = bullet1.get_angular_velocity();
        let angular_velocity2 = bullet2.get_angular_velocity();

        // belocity of each body exactly at the contact point
        let contact_velocity1 =
            self.get_contact_velocity(velocity1, angular_velocity1, r1);

        let contact_velocity2 =
            self.get_contact_velocity(velocity2, angular_velocity2, r2);

        let relative_velocity = contact_velocity2 - contact_velocity1;

        let velocity_along_normal = relative_velocity.dot(normal);

        // the bodies are moving apart, so no need to resolve the collision
        if velocity_along_normal >= 0.0 {
            self.correct_penetration(
                bullet1,
                bullet2,
                &collision_info,
                inverse_mass1,
                inverse_mass2,
                inverse_mass_sum
            );

            return;
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
            return;
        }

        let restitution = bullet1.get_restitution().min(bullet2.get_restitution());

        let normal_impulse_magnitude =
            self.get_normal_impulse_magnitude(
                restitution,
                velocity_along_normal,
                impulse_denominator
            );

        let normal_impulse = normal * normal_impulse_magnitude;

        let new_velocity1 = velocity1 - normal_impulse * inverse_mass1;
        let new_velocity2 = velocity2 + normal_impulse * inverse_mass2;

        let new_angular_velocity1 =
            angular_velocity1 - r1.perp_dot(normal_impulse) * inverse_inertia1;
        let new_angular_velocity2 =
            angular_velocity2 + r2.perp_dot(normal_impulse) * inverse_inertia2;

        bullet1.set_velocity(new_velocity1);
        bullet2.set_velocity(new_velocity2);

        bullet1.set_angular_velocity(new_angular_velocity1);
        bullet2.set_angular_velocity(new_angular_velocity2);

        self.correct_penetration(
            bullet1,
            bullet2,
            &collision_info,
            inverse_mass1,
            inverse_mass2,
            inverse_mass_sum
        );
    }

    fn build_spatial_grid(
        &self,
        bullets: &Vec<Bullet>,
        world_size: (f32, f32),
        cell_size: f32
    ) -> Vec<Vec<usize>> {
        let grid_width = (world_size.0 / cell_size).ceil() as usize;
        let grid_height = (world_size.1 / cell_size).ceil() as usize;
        let mut grid = vec![vec![]; grid_width * grid_height];

        for (i, bullet) in bullets.iter().enumerate() {
            let position = bullet.get_position();
            let x_index = ((position.x + world_size.0 / 2.0) / cell_size).floor() as usize;
            let y_index = ((position.y + world_size.1 / 2.0) / cell_size).floor() as usize;

            if x_index < grid_width && y_index < grid_height {
                grid[y_index * grid_width + x_index].push(i);
            }
        }

        grid
    }

    fn is_out_of_bounds(
        &self,
        position: &Vec2,
        world_size: (f32, f32),
        bullet_radius: f32
    ) -> bool {
        let half_width = world_size.0 / 2.0;
        let half_height = world_size.1 / 2.0;

        position.x - bullet_radius < -half_width ||
            position.x + bullet_radius > half_width ||
            position.y - bullet_radius < -half_height ||
            position.y + bullet_radius > half_height
    }

    fn correct_penetration(
        &self,
        bullet1: &mut Bullet,
        bullet2: &mut Bullet,
        collision_info: &CollisionInfo,
        inverse_mass1: f32,
        inverse_mass2: f32,
        inverse_mass_sum: f32
    ) {
        if inverse_mass_sum <= EPSILON {
            return;
        }

        const PENETRATION_SLOP: f32 = 0.01;
        const CORRECTION_PERCENTAGE: f32 = 0.8;

        let penetration_depth = collision_info.get_penetration_depth();

        let correction_magnitude =
            ((penetration_depth - PENETRATION_SLOP).max(0.0) / inverse_mass_sum) *
            CORRECTION_PERCENTAGE;

        if correction_magnitude <= 0.0 {
            return;
        }

        let correction = collision_info.get_normal() * correction_magnitude;

        let position1 = *bullet1.get_position();
        let position2 = *bullet2.get_position();

        bullet1.set_position(position1 - correction * inverse_mass1);
        bullet2.set_position(position2 + correction * inverse_mass2);
    }

    fn angular_velocity_cross_radius(&self, angular_velocity: f32, radius: Vec2) -> Vec2 {
        Vec2::new(-angular_velocity * radius.y, angular_velocity * radius.x)
    }
}

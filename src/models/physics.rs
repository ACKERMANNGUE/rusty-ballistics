use bevy::prelude::Vec2;

use crate::models::bullet::Bullet;
use crate::models::wind::Wind;

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

    pub fn update(&mut self, bullets: &mut Vec<Bullet>, world_size: (f32, f32)) {
        self.wind.update_turbulence();
        for bullet in bullets.iter_mut() {
            let new_position = self.compute_new_position(bullet);

            if self.is_out_of_bounds(&new_position, world_size, bullet.get_size()) {
                bullet.set_is_dead(true);
                continue;
            }

            self.set_new_position_and_velocity(bullet, new_position);
        }

        bullets.retain(|bullet: &Bullet| !bullet.is_dead());
        self.compute_collisions(bullets, world_size);
    }

    fn compute_collisions(&self, bullets: &mut Vec<Bullet>, world_size: (f32, f32)) {
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
                bullets
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

    fn compute_new_position(&self, bullet: &Bullet) -> Vec2 {
        let wind_velocity = self.wind.get_direction() * self.wind.get_speed();
        let relative_velocity =
            *bullet.get_velocity() - wind_velocity - *self.wind.get_turbulence();
        let mut drag_force = -self.air_resistance * *bullet.get_velocity();

        if self.wind.is_active() {
            drag_force = -self.air_resistance * relative_velocity;
        }

        let drag_acceleration = drag_force / bullet.get_mass();
        let gravity_acceleration = Vec2::new(0.0, -self.gravity);
        let acceleration = gravity_acceleration + drag_acceleration;

        let new_velocity = *bullet.get_velocity() + acceleration * self.delta_time;
        let new_position = *bullet.get_position() + new_velocity * self.delta_time;
        new_position
    }

    fn set_new_position_and_velocity(&self, bullet: &mut Bullet, new_position: Vec2) {
        bullet.set_position(new_position);
        bullet.set_velocity(
            Vec2::new(
                bullet.get_velocity().x,
                bullet.get_velocity().y - self.gravity * self.delta_time
            )
        );
        bullet.set_velocity(*bullet.get_velocity() * (1.0 - self.air_resistance * self.delta_time));
    }

    fn check_collisions_in_neighbours(
        &self,
        bullet_index: usize,
        x_index: isize,
        y_index: isize,
        grid_width: usize,
        grid_height: usize,
        spatial_grid: &Vec<Vec<usize>>,
        bullets: &mut Vec<Bullet>
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
                    let delta = *bullet.get_position() - *other_bullet.get_position();
                    let distance_squared = delta.length_squared();
                    if distance_squared == 0.0 {
                        println!(
                            "WARNING: Two bullets are at the same position, skipping collision resolution!"
                        );
                        continue;
                    }

                    // TODO: Use the actual shapes of the bullets for collision detection instead of just using their sizes
                    let combined_radius = bullet.get_size() + other_bullet.get_size();
                    if
                        distance_squared < combined_radius * combined_radius &&
                        !bullet.is_dead() &&
                        !other_bullet.is_dead()
                    {
                        self.compute_collision_response(bullet, other_bullet);
                    }
                }
            }
        }
    }

    fn compute_collision_response(&self, bullet1: &mut Bullet, bullet2: &mut Bullet) {
        let mass1 = bullet1.get_mass();
        let mass2 = bullet2.get_mass();

        if mass1 <= 0.0 || mass2 <= 0.0 {
            // TODO: Handle this case properly, maybe by removing the bullet from the simulation, maybe by adding a flag "dead" to the bullet,
            // maybe by doing something else. For now, we just skip the collision resolution.
            return;
        }

        let position1 = *bullet1.get_position();
        let position2 = *bullet2.get_position();

        let velocity1 = *bullet1.get_velocity();
        let velocity2 = *bullet2.get_velocity();

        let direction = (position2 - position1).normalize();
        let relative_velocity = velocity1 - velocity2;
        let s = relative_velocity.dot(direction);

        if s <= 0.0 {
            return;
        }

        let impulse = (2.0 * s) / (1.0 / mass1 + 1.0 / mass2);
        bullet1.set_velocity(velocity1 - (impulse / mass1) * direction);
        bullet2.set_velocity(velocity2 + (impulse / mass2) * direction);
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
}

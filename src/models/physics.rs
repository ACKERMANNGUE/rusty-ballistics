use crate::models::bullet::Bullet;

pub struct Physics {
    delta_time: f32,
    air_resistance: f32,
    gravity: f32,
}

impl Physics {
    pub fn new(delta_time: f32, air_resistance: f32, gravity: f32) -> Self {
        Self {
            delta_time,
            air_resistance,
            gravity,
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

    pub fn update(&self, bullets: &mut Vec<Bullet>, world_size: (f32, f32)) {
        let gravity = self.gravity;
        let air_resistance = self.air_resistance;
        let delta_time = self.delta_time;

        for bullet in bullets.iter_mut() {
            let new_position = *bullet.get_position() + *bullet.get_velocity() * delta_time;

            if self.is_out_of_bounds(&new_position, world_size, bullet.get_radius()) {
                bullet.set_velocity(glam::Vec2::new(0.0, 0.0));
                bullet.set_mass(0.0);
                continue;
            }

            bullet.set_position(new_position);
            bullet.set_velocity(glam::Vec2::new(
                bullet.get_velocity().x,
                bullet.get_velocity().y - gravity * delta_time,
            ));
            bullet.set_velocity(*bullet.get_velocity() * (1.0 - air_resistance * delta_time));
        }

        let grid_cell_size = 100.0;

        let spatial_grid = self.build_spatial_grid(bullets, world_size, grid_cell_size);

        let grid_width = (world_size.0 / grid_cell_size).ceil() as usize;

        let grid_height = (world_size.1 / grid_cell_size).ceil() as usize;

        for i in 0..bullets.len() {
            let position = *bullets[i].get_position();

            let x_index = ((position.x + world_size.0 / 2.0) / grid_cell_size).floor() as isize;
            let y_index = ((position.y + world_size.1 / 2.0) / grid_cell_size).floor() as isize;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    let neighbor_x = x_index + dx;
                    let neighbor_y = y_index + dy;

                    if neighbor_x < 0 || neighbor_y < 0 {
                        continue;
                    }

                    if neighbor_x as usize >= grid_width || neighbor_y as usize >= grid_height {
                        continue;
                    }

                    let cell_index = neighbor_y as usize * grid_width + neighbor_x as usize;

                    for &j in &spatial_grid[cell_index] {
                        if j <= i {
                            // made to avoid double checking and self-collision
                            continue;
                        }

                        let (left, right) = bullets.split_at_mut(j);

                        let bullet = &mut left[i];
                        let other_bullet = &mut right[0];

                        let delta = *bullet.get_position() - *other_bullet.get_position();
                        let distance_squared = delta.length_squared();
                        if distance_squared == 0.0 {
                            println!("WARNING : Two bullets are at the same position, skipping collision resolution !");
                            continue;
                        }
                        let combined_radius = bullet.get_radius() + other_bullet.get_radius();

                        if distance_squared < combined_radius * combined_radius {
                            if distance_squared == 0.0 {
                                continue;
                            }

                            let mass_1 = bullet.get_mass();
                            let mass_2 = other_bullet.get_mass();

                            if mass_1 <= 0.0 || mass_2 <= 0.0 { 
                                // TODO: Handle this case properly, maybe by removing the bullet from the simulation, maybe by adding a flag "dead" to the bullet,
                                // maybe by doing something else. For now, we just skip the collision resolution.
                                continue;
                            }

                            let position_1 = *bullet.get_position();
                            let position_2 = *other_bullet.get_position();

                            let velocity_1 = *bullet.get_velocity();
                            let velocity_2 = *other_bullet.get_velocity();

                            let direction = (position_2 - position_1).normalize();

                            let relative_velocity = velocity_1 - velocity_2;

                            let s = relative_velocity.dot(direction);

                            if s <= 0.0 {
                                continue;
                            }

                            let impulse = (2.0 * s) / ((1.0 / mass_1) + (1.0 / mass_2));

                            bullet.set_velocity(velocity_1 - (impulse / mass_1) * direction);

                            other_bullet.set_velocity(velocity_2 + (impulse / mass_2) * direction);
                        }
                    }
                }
            }
        }
    }

    fn build_spatial_grid(
        &self,
        bullets: &Vec<Bullet>,
        world_size: (f32, f32),
        cell_size: f32,
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
        position: &glam::Vec2,
        world_size: (f32, f32),
        bullet_radius: f32,
    ) -> bool {
        let half_width = world_size.0 / 2.0;
        let half_height = world_size.1 / 2.0;

        position.x - bullet_radius < -half_width
            || position.x + bullet_radius > half_width
            || position.y - bullet_radius < -half_height
            || position.y + bullet_radius > half_height
    }
}

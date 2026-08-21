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

    // Basic physics update function that updates the position and velocity of bullets in the world based on gravity and air resistance
    pub fn update(&self, bullets: &mut Vec<Bullet>, world_size: (f32, f32)) {
        let gravity = self.gravity;
        let air_resistance = self.air_resistance;
        let delta_time = self.delta_time;

        for bullet in bullets.iter_mut() {
            let new_position = *bullet.get_position() + *bullet.get_velocity() * delta_time;

            if self.is_out_of_bounds(&new_position, world_size, bullet.get_radius()) {
                // println!("Bullet {} has left the world boundaries", bullet.get_name());
                // TODO: Handle bullet leaving the world boundaries (remove it from the world)
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

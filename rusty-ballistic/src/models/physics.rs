use crate::models::bullet::Bullet;

pub struct Physics {
    delta_time: f32,
    air_resistance: f32,
    gravity: f32,
}

impl Physics {
    pub fn new(delta_time: f32, air_resistance: f32, gravity: f32) -> Self {
        Self { delta_time, air_resistance, gravity }
    }

    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }

    pub fn get_air_resistance(&self) -> f32 {
        self.air_resistance
    }

    // Basic physics update function that updates the position and velocity of bullets in the world based on gravity and air resistance
    pub fn update(&self, bullets: &mut Vec<Bullet>, world_size: (f32, f32)) {
        let gravity = self.gravity;
        let air_resistance = self.air_resistance;
        let delta_time = self.delta_time;

        for bullet in bullets.iter_mut() {
            let new_position = *bullet.get_position() + *bullet.get_velocity() * delta_time;

            if new_position.x < 0.0 || new_position.x > world_size.0 || new_position.y < 0.0 || new_position.y > world_size.1 {
                println!("Bullet {} has left the world boundaries", bullet.get_name());
                // TODO: Handle bullet leaving the world boundaries (remove it from the world)
                bullet.set_velocity(glam::Vec2::new(0.0, 0.0));
                continue; 
            }

            bullet.set_position(new_position);
            bullet.set_velocity(glam::Vec2::new(
                bullet.get_velocity().x,
                bullet.get_velocity().y - gravity * delta_time
            ));
            bullet.set_velocity(*bullet.get_velocity() * (1.0 - air_resistance * delta_time));
        }
    }
}
use crate::models::world::World;

pub struct Physics {
    world: World,
    delta_time: f32,
}

impl Physics {
    pub fn new(world: World, delta_time: f32) -> Self {
        Self { world, delta_time }
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    // Basic physics update function that updates the position and velocity of bullets in the world based on gravity and air resistance
    pub fn update(&mut self) {
        let gravity = self.world.get_gravity();
        let air_resistance = self.world.get_air_resistance();
        let delta_time = self.delta_time;

        for bullet in self.world.get_bullets() {
            bullet.set_position(*bullet.get_position() + *bullet.get_velocity() * delta_time);
            bullet.set_velocity(glam::Vec2::new(
                bullet.get_velocity().x,
                bullet.get_velocity().y - gravity * delta_time
            ));
            bullet.set_velocity(*bullet.get_velocity() * (1.0 - air_resistance * delta_time));
        }
    }
}
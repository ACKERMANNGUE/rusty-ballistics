use bevy::prelude::Resource;

use crate::models::bullet::Bullet;
use crate::models::physics::Physics;

#[derive(Resource)]
pub struct SimulationWorld {
    bullets: Vec<Bullet>,
    size: (f32, f32),
    physics: Physics,
}

impl SimulationWorld {
    pub fn new(size: (f32, f32), physics: Physics) -> Self {
        Self {
            bullets: Vec::new(),
            size,
            physics,
        }
    }

    pub fn add_bullet(&mut self, bullet: Bullet) {
        self.bullets.push(bullet);
    }

    pub fn get_bullets_read(&self) -> &Vec<Bullet> {
        &self.bullets
    }

    pub fn get_bullets(&mut self) -> &mut Vec<Bullet> {
        &mut self.bullets
    }

    pub fn get_size(&self) -> (f32, f32) {
        self.size
    }

    pub fn update(&mut self) {
        self.physics.update(&mut self.bullets, self.size);
    }

    fn is_frozen(&self, stagnated_counts: usize, total_bullets: usize) -> bool {
        stagnated_counts == total_bullets
    }

    pub fn check_bullet_stagnation(&self, bullet: &Bullet) -> bool {
        let velocity = bullet.get_velocity();
        velocity.x.abs() < 0.01 && velocity.y.abs() < 0.01
    }

    pub fn display_in_term(&self) {
        println!(
            "World\n\tGravity: {}\n\tSize: {:?}\n\tAir Resistance: {}\n\tBullets: {}",
            self.physics.get_gravity(),
            self.size,
            self.physics.get_air_resistance(),
            self.bullets.len()
        );
    }

    pub fn display_bullets_in_term(&self) {
        for bullet in &self.bullets {
            bullet.display_in_term();
        }
    }
}

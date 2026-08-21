use crate::models::bullet::Bullet;
use crate::models::physics::Physics;

pub struct World {
    bullets: Vec<Bullet>,
    size: (f32, f32),
    physics: Physics,
}

impl World {
    pub fn new(size: (f32, f32), physics: Physics) -> Self {
        Self {
            bullets: Vec::new(),
            size,
            physics,
        }
    }

    // pub fn get_size(&self) -> (f32, f32) {
    //     self.size
    // }

    pub fn add_bullet(&mut self, bullet: Bullet) {
        self.bullets.push(bullet);
    }

    pub fn update(&mut self) {
        self.physics.update(&mut self.bullets);
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

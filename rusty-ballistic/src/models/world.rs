use crate::models::bullet::Bullet;

pub struct World {
    bullets: Vec<Bullet>,
    gravity: f32,
    size: (f32, f32),
    air_resistance: f32,
}

impl World {
    pub fn new(gravity: f32, size: (f32, f32), air_resistance: f32) -> Self {
        Self {
            bullets: Vec::new(),
            gravity,
            size,
            air_resistance,
        }
    }

    pub fn add_bullet(&mut self, bullet: Bullet) {
        self.bullets.push(bullet);
    }

    pub fn get_bullets(&mut self) -> &mut Vec<Bullet> {
        &mut self.bullets
    }

    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }

    pub fn get_size(&self) -> (f32, f32) {
        self.size
    }

    pub fn get_air_resistance(&self) -> f32 {
        self.air_resistance
    }

    pub fn display_in_term(&self) {
        println!(
            "World\n\tGravity: {}\n\tSize: {:?}\n\tAir Resistance: {}\n\tBullets: {}",
            self.gravity,
            self.size,
            self.air_resistance,
            self.bullets.len()
        );
    }

    pub fn display_bullets_in_term(&self) {
        for bullet in &self.bullets {
            bullet.display_in_term();
        }
    }
}

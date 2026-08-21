use crate::models::bullet::Bullet;
use crate::models::physics::Physics;

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

    pub fn get_bullets(&mut self) -> &mut Vec<Bullet> {
        &mut self.bullets
    }

    pub fn update(&mut self) {
        // might not be the best place to check for stagnation, but it works for now
        // TODO: remove bullets that have stagnated from the world
        let mut stagnated_counts = 0;
        while !self.is_frozen(stagnated_counts, self.bullets.len()) {
            for bullet in &self.bullets {
                if self.check_bullet_stagnation(bullet) {
                    stagnated_counts += 1;
                    println!("Bullet {} has stagnated", bullet.get_name());
                }
            }
            self.physics.update(&mut self.bullets, self.size);
            self.display_bullets_in_term();
        }
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

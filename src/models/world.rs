use bevy::prelude::Resource;

use crate::models::bullet::Bullet;
use crate::models::physics::Physics;
use crate::resources::shape_library::ShapeLibrary;

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

    pub fn get_physics(&self) -> &Physics {
        &self.physics
    }

    pub fn get_physics_mut(&mut self) -> &mut Physics {
        &mut self.physics
    }

    pub fn add_bullet(&mut self, bullet: Bullet) {
        self.bullets.push(bullet);
    }

    pub fn get_bullets(&self) -> &[Bullet] {
        &self.bullets
    }

    pub fn get_bullet_by_id(&self, id: u32) -> Option<&Bullet> {
        self.bullets.iter().find(|bullet| bullet.get_id() == id)
    }

    pub fn clear_bullets(&mut self) {
        self.bullets.clear();
    }

    pub fn get_size(&self) -> (f32, f32) {
        self.size
    }

    pub fn update(&mut self, shape_library: &ShapeLibrary) {
        self.physics
            .update(&mut self.bullets, self.size, shape_library);
    }
}

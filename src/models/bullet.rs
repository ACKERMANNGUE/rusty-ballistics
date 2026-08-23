use bevy::prelude::Vec2;
use crate::config::BASE_BULLET_SIZE;

pub struct Bullet {
    name: String,
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    color: (f32, f32, f32),
    size: f32,
    is_dead: bool,
    id: u32,
    shape: String,
}

impl Bullet {
    pub fn new(
        name: String,
        position: Vec2,
        velocity: Vec2,
        mass: f32,
        color: (f32, f32, f32),
        id: u32,
        shape: String
    ) -> Self {
        Self {
            name,
            position,
            velocity,
            mass,
            color,
            size: Self::compute_bullet_size(mass),
            is_dead: false,
            id,
            shape,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_shape(&self) -> &String {
        &self.shape
    }

    fn compute_bullet_size(mass: f32) -> f32 {
        BASE_BULLET_SIZE * mass
    }

    pub fn get_size(&self) -> f32 {
        self.size
    }

    pub fn get_color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn get_velocity(&self) -> &Vec2 {
        &self.velocity
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn set_is_dead(&mut self, is_dead: bool) {
        self.is_dead = is_dead;
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }
}

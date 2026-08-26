use bevy::prelude::Vec2;
use crate::config::BASE_BULLET_SIZE;

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    color: (f32, f32, f32),
    size: f32,
    is_dead: bool,
    id: u32,
    shape: String,
    restitution: f32,
    static_friction: f32,
    dynamic_friction: f32,
    rotation: f32, // in radians
    angular_velocity: f32,
}

impl Bullet {
    pub fn new(
        position: Vec2,
        velocity: Vec2,
        mass: f32,
        color: (f32, f32, f32),
        id: u32,
        shape: String,
        restitution: f32,
        static_friction: f32,
        dynamic_friction: f32
    ) -> Self {
        Self {
            position,
            velocity,
            mass,
            color,
            size: Self::compute_bullet_size(mass),
            is_dead: false,
            id,
            shape,
            restitution,
            static_friction,
            dynamic_friction,
            rotation: 0.0,
            angular_velocity: 1.0,
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

    pub fn get_restitution(&self) -> f32 {
        self.restitution
    }

    pub fn get_static_friction(&self) -> f32 {
        self.static_friction
    }

    pub fn get_dynamic_friction(&self) -> f32 {
        self.dynamic_friction
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn get_angular_velocity(&self) -> f32 {
        self.angular_velocity
    }

    pub fn set_angular_velocity(&mut self, angular_velocity: f32) {
        self.angular_velocity = angular_velocity;
    }

    pub fn add_angular_velocity(&mut self, delta: f32) {
        self.angular_velocity += delta;
    }

    pub fn add_rotation(&mut self, delta: f32) {
        self.rotation += delta;
    }
}

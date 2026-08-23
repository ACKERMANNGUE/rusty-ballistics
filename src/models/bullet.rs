use bevy::prelude::Vec2;

const BASE_BULLET_RADIUS: f32 = 10.0;

pub struct Bullet {
    name: String,
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    color: (f32, f32, f32),
    radius: f32,
    is_dead: bool,
    id: u32,
}

impl Bullet {
    pub fn new(
        name: String,
        position: Vec2,
        velocity: Vec2,
        mass: f32,
        color: (f32, f32, f32),
        id: u32
    ) -> Self {
        Self {
            name,
            position,
            velocity,
            mass,
            color,
            radius: Self::compute_bullet_radius(mass),
            is_dead: false,
            id,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn compute_bullet_radius(mass: f32) -> f32 {
        BASE_BULLET_RADIUS * mass * 10.0 + BASE_BULLET_RADIUS
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    // pub fn set_color(&mut self, color: (f32, f32, f32)) {
    //     self.color = color;
    // }

    pub fn get_color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn get_name(&self) -> &String {
        &self.name
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

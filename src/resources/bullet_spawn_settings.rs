use bevy::prelude::Resource;

#[derive(Resource)]
pub struct BulletSpawnSettings {
    size: f32,
    density: f32,
    restitution: f32,
    static_friction: f32,
    dynamic_friction: f32,
}

impl BulletSpawnSettings {
    pub fn new() -> Self {
        Self {
            size: 5.0,
            density: 1.0,
            restitution: 0.8,
            static_friction: 0.5,
            dynamic_friction: 0.3,
        }
    }

    pub fn get_size(&self) -> f32 {
        self.size
    }

    pub fn set_size(&mut self, size: f32) {
        self.size = size.max(0.01);
    }

    pub fn get_density(&self) -> f32 {
        self.density
    }

    pub fn set_density(&mut self, density: f32) {
        self.density = density.max(0.01);
    }

    pub fn get_restitution(&self) -> f32 {
        self.restitution
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    pub fn get_static_friction(&self) -> f32 {
        self.static_friction
    }

    pub fn set_static_friction(&mut self, static_friction: f32) {
        self.static_friction = static_friction;
    }

    pub fn get_dynamic_friction(&self) -> f32 {
        self.dynamic_friction
    }

    pub fn set_dynamic_friction(&mut self, dynamic_friction: f32) {
        self.dynamic_friction = dynamic_friction;
    }
}

use glam::Vec2;

pub struct Wind {
    direction: Vec2,
    speed: f32,
    turbulence: f32,
    active: bool,
}

impl Wind {
    pub fn new(direction: Vec2, speed: f32, turbulence: f32, active: bool) -> Self {
        Self {
            direction,
            speed,
            turbulence,
            active,
        }
    }

    pub fn get_direction(&self) -> &Vec2 {
        &self.direction
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn get_turbulence(&self) -> f32 {
        self.turbulence
    }

    pub fn get_direction_degrees(&self) -> f32 {
        self.direction.y.atan2(self.direction.x).to_degrees()
    }
}

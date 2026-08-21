use glam::Vec2;

pub struct Bullet {
    name: String,
    position: Vec2,
    velocity: Vec2,
    mass: f32,
}

impl Bullet {
    pub fn new(name: String, position: Vec2, velocity: Vec2, mass: f32) -> Self {
        Self {
            name,
            position,
            velocity,
            mass
        }
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

    // pub fn get_mass(&self) -> f32 {
    //     self.mass
    // }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn compute_kinetic_energy(&self) -> f32 {
        0.5 * self.mass * self.velocity.length_squared()
    }

    pub fn compute_momentum(&self) -> Vec2 {
        self.mass * self.velocity
    }

    pub fn compute_angle(&self) -> f32 {
        self.velocity.y.atan2(self.velocity.x) * 180.0 / std::f32::consts::PI
    }

    pub fn display_in_term(&self) {
        println!(
            "Bullet\n\tName: {}\n\tPosition: {:?}\n\tVelocity: {:?}\n\tMass: {}\n\tKinetic Energy: {}\n\tMomentum: {:?}\n\tAngle: {}",
            self.name,
            self.position,
            self.velocity,
            self.mass,
            self.compute_kinetic_energy(),
            self.compute_momentum(),
            self.compute_angle()
        );
    }
}
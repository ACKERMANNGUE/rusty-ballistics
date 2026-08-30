use crate::geometry::shape::Shape;

#[derive(Debug, Clone, Copy)]
pub struct MassProperties {
    scaled_area: f32,
    mass: f32,
    moment_of_inertia: f32,
}

impl MassProperties {
    pub fn get_scaled_area(&self) -> f32 {
        self.scaled_area
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn get_moment_of_inertia(&self) -> f32 {
        self.moment_of_inertia
    }
}

pub fn compute_mass_properties(
    shape: &Shape,
    size: f32,
    density: f32,
) -> MassProperties {
    let scaled_area = shape.get_area() * size.powi(2);
    let mass = density * scaled_area;
    let moment_of_inertia =
        mass * size.powi(2) * shape.get_inertia_factor();

    MassProperties {
        scaled_area,
        mass,
        moment_of_inertia,
    }
}
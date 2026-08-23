use bevy::prelude::*;

pub fn project_polygon(axis: Vec2, polygon: &[Vec2]) -> (f32, f32) {
    let first_projection = axis.dot(polygon[0]);

    let mut min = first_projection;
    let mut max = first_projection;

    for point in polygon.iter().skip(1) {
        let projection = axis.dot(*point);

        if projection < min {
            min = projection;
        }

        if projection > max {
            max = projection;
        }
    }

    (min, max)
}

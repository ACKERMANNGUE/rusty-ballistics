// used vecDeque to store the points of the bullet trail
use std::collections::VecDeque;
/*
add here ->
[p1][p2][p3][p4][p5]
 ^
delete here
*/

use bevy::prelude::*;

#[derive(Component)]
pub struct BulletTrail {
    pub points: VecDeque<Vec2>,
    pub max_points: usize,
}

impl BulletTrail {
    pub fn new(max_points: usize) -> Self {
        Self {
            points: VecDeque::new(),
            max_points,
        }
    }

    pub fn push(&mut self, point: Vec2) {
        self.points.push_back(point);

        if self.points.len() > self.max_points {
            self.points.pop_front();
        }
    }
}

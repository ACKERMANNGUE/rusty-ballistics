use bevy::prelude::*;

#[derive(Component)]
pub struct BulletEntity {
    id: u32,
}

impl BulletEntity {
    pub fn new(id: u32) -> Self {
        BulletEntity { id }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

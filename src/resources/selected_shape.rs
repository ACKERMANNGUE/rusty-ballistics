use bevy::prelude::*;

const DEFAULT_SHAPE_NAME: &str = "square";

#[derive(Resource)]
pub struct SelectedShape {
    shape_name: String,
}

impl SelectedShape {
    pub fn new() -> Self {
        Self {
            shape_name: DEFAULT_SHAPE_NAME.into(),
        }
    }

    pub fn get_shape_name(&self) -> &String {
        &self.shape_name
    }

    pub fn set_shape_name(&mut self, shape_name: String) {
        self.shape_name = shape_name;
    }
}

use bevy::prelude::{Res, Resource};

#[derive(Resource)]
pub struct DebugVisibility {
    visible: bool,
}

impl DebugVisibility {
    pub fn new() -> Self {
        Self {
            visible: true,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

pub fn debug_visuals_visible(
    debug_visibility: Res<DebugVisibility>,
) -> bool {
    debug_visibility.is_visible()
}
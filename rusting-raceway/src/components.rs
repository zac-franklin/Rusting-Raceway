use bevy::prelude::*;

/// Player component with handle ID.
#[derive(Component, Clone, Copy)]
pub struct Player {
    pub handle: usize,
    pub pos_index: usize,  // last path point crossed
    pub distance: f32,  // distance traveled since last path point
}
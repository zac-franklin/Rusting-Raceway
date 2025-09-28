use bevy::prelude::*;

/// Player component with handle ID.
#[derive(Component, Clone, Copy)]
pub struct Player {
    pub handle: usize,
}
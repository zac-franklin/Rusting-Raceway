pub mod args;
mod components;
pub mod gameplay;
mod inputs;
pub mod menu;
pub mod networking;
mod physics;
pub mod resources;
pub mod splash;
pub mod states;

use bevy::prelude::*;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

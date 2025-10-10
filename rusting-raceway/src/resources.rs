use bevy::prelude::*;

/// Represents a path
#[derive(Resource, Default)]
pub struct Paths(pub Vec<Vec<Vec3>>);

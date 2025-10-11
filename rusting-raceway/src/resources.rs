use bevy::prelude::*;

// Newtype to use a `Timer` for splash screen
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

/// Represents a path
#[derive(Resource, Default)]
pub struct Paths(pub Vec<Vec<Vec3>>);

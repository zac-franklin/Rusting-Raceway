use bevy::prelude::*;

// Newtype to use a `Timer` for splash screen
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
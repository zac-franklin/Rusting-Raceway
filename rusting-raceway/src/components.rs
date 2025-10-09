use bevy::prelude::*;

/// Player component with handle ID.
#[derive(Component, Clone, Copy)]
pub struct Player {
    pub handle: usize,
}

// Component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

// Component used to tag entities added on the splash screen
#[derive(Component)]
pub struct OnSplashScreen;

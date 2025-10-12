use bevy::prelude::*;

// Newtype to use a `Timer` for splash screen
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

/// Represents a path
#[derive(Resource, Default)]
pub struct Paths(pub Vec<Vec<Vec3>>);

/// Our Font
#[derive(Resource)]
pub struct FontHandle(pub Handle<Font>);

/// Right Menu Icon
#[derive(Resource)]
pub struct RightMenuIcon(pub Handle<Image>);

/// WrenchMenuIcon
#[derive(Resource)]
pub struct WrenchMenuIcon(pub Handle<Image>);

/// ExitMenuIcon
#[derive(Resource)]
pub struct ExitMenuIcon(pub Handle<Image>);

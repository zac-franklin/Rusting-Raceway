use bevy::prelude::*;

/// Player component with handle ID.
#[derive(Component, Clone, Copy)]
pub struct Player {
    pub handle: usize,
    pub pos_index: usize,  // last path point crossed
    pub distance: f32,  // distance traveled since last path point
}

/// Component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

/// Component used to tag entities added on the splash screen
#[derive(Component)]
pub struct OnSplashScreen;

/// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

/// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnSettingsMenuScreen;

/// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
pub struct OnDisplaySettingsMenuScreen;

/// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
pub struct OnSoundSettingsMenuScreen;

/// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

/// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

/// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

/// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

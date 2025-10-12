use bevy::prelude::*;

use super::{components::OnSplashScreen, despawn_screen, resources::SplashTimer, states::GameState};

const BEVY_COLOR: Color = Color::srgba(0.16, 0.16, 0.16, 1.);

/// Plugin to display a splash screen with Bevy logo for 2 seconds
pub fn splash_plugin(app: &mut App) {
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        // While in this state, run the `countdown` system
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

/// Setup the splash screen
fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("images/bevy_banner.png");
    // Display the logo
    commands.spawn((
        OnSplashScreen,
        Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(BEVY_COLOR),
        children![(
            ImageNode::new(icon),
            Node {
                // This will set the logo to be 300px height, and auto adjust its width
                height: Val::Px(300.0),
                ..default()
            },
        )],
    ));
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

/// Tick the timer and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
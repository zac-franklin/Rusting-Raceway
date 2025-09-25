use rusting_raceway::{args, gameplay, inputs, networking, states};
use bevy::prelude::*;
use bevy_ggrs::prelude::*;

fn main() {
    // Read CMD args
    let args = args::UserInput::parse();

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire browser window
                    fit_canvas_to_parent: true,
                    // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<networking::Config>::default(),
        ))
        .init_state::<states::GameState>()
        .rollback_component_with_clone::<Transform>()
        .insert_resource(args)
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(
            Startup, //This will eventual move to OnEnter(GameState::Matchmaking)
            (gameplay::setup, gameplay::spawn_player, networking::start_matchbox_socket.run_if(p2p_mode))
        )
        .add_systems(
            Update, 
            (
                networking::wait_for_players.run_if(p2p_mode),
                networking::start_synctest_session.run_if(local_mode),
            )
                .run_if(in_state(states::GameState::Matchmaking))
        )
        .add_systems(ReadInputs, inputs::read_local_inputs)
        .add_systems(GgrsSchedule, gameplay::move_players) 
        .run();
}

/// mode for no network dependencies.
fn local_mode(args: Res<args::UserInput>) -> bool {
    args.local_only
}

/// mode for live networking.
fn p2p_mode(args: Res<args::UserInput>) -> bool {
    !args.local_only
}

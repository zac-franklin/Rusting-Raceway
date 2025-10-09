use rusting_raceway::{args, gameplay, networking, splash, states};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::prelude::*;
use bevy_polyline::PolylinePlugin;

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
            PolylinePlugin,
        ))
        .init_state::<states::GameState>()
        .rollback_component_with_clone::<Transform>()
        .insert_resource(args)
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(
            Startup,
            setup_camera,
        )
        .add_plugins((splash::splash_plugin, gameplay::game_plugin)) 
        .run();
}

/// Setup the camera and view.
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1000.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Light source for 3d rendering
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1_000.0,
        ..default()
    });
}

use super::{args, despawn_screen, components, inputs, networking, physics, states};
use bevy::prelude::*;
use bevy_ggrs::prelude::*;
use bevy_polyline::prelude::*;

// Game specific setup
pub fn game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(states::GameState::Matchmaking), //TODO: create a matchmaking screen between game screen and move game setup here to OnEnter(gamestate::InGame)
            (
                spawn_stadium, 
                spawn_player_tracks,
                spawn_player,
                networking::start_matchbox_socket.run_if(p2p_mode)
            )
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
        .add_systems(GgrsSchedule, move_players) 
        .add_systems(OnExit(states::GameState::InGame), despawn_screen::<components::OnGameScreen>);
}

/// Spawn stadium
fn spawn_stadium(
    mut commands: Commands, 
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let length = 500.0;
    let inner_radius = 225.0;
    let radius_ratio = 2.0; // ratio between inner and outer radius
    let bend_sections: u32 = 20;
    debug_assert!(bend_sections % 2 == 0, "Section size needs to be even to preserve symmetry");

    // Spawn grass
    let grass_color = Color::hsl(99.0, 0.66, 0.55);
    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline { 
                vertices: physics::determine_track_points(
                    length, inner_radius, bend_sections
                )
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 3.0,
                color: grass_color.into(),
                perspective: false,
                ..default()
            })),
            ..default()
        },
        components::OnGameScreen,
    ));

    // Spawn tracks
    let track_color = Color::hsl(35.0, 0.69, 0.63);
    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline { 
                vertices: physics::determine_track_points(
                    length, radius_ratio * inner_radius, bend_sections
                )
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 3.0,
                color: track_color.into(),
                perspective: false,
                ..default()
            })),
            ..default()
        },
        components::OnGameScreen,
    ));
}

/// Spawn player tracks (paths)
fn spawn_player_tracks(
    mut commands: Commands, 
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let length = 500.0;  // length of horizontal section (stadium)
    let inner_radius = 225.0;
    let num_tracks = 4;
    let bend_sections: u32 = 20;
    debug_assert!(bend_sections % 2 == 0, "Section size needs to be even to preserve symmetry");

    let ratio_increase = 1.0 / (num_tracks as f32 + 1.0);
    let color = Color::hsl(35.0, 0.69, 0.32);

    // Iterate over individual tracks and spawn them
    for track_no in 1..=num_tracks {
        let ratio = 1.0 + (track_no as f32) * ratio_increase;
        let radius = inner_radius * ratio;

        // Spawn track
        commands.spawn((
            PolylineBundle {
                polyline: PolylineHandle(polylines.add(Polyline { 
                    vertices: physics::determine_track_points(
                        length, radius, bend_sections
                    )
                })),
                material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                    width: 3.0,
                    color: color.into(),
                    perspective: false,
                    ..default()
                })),
                ..default()
            },
            components::OnGameScreen,
        ));
    }
}

/// Spawn the players with GGRS Rollback in the game
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Sphere::default().mesh().uv(32, 18));

    commands.spawn((
        components::OnGameScreen,
        components::Player{ handle: 0 },
        Mesh3d(sphere.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0., 0.47, 1.))),
        Transform::from_translation(Vec3::new(-100., 0., 0.))
            .with_scale(Vec3::splat(25.))
    ))
    .add_rollback();

    commands.spawn((
        components::OnGameScreen,
        components::Player{ handle: 1 },
        Mesh3d(sphere),
        MeshMaterial3d(materials.add(Color::srgb(0., 0.4, 0.))),
        Transform::from_translation(Vec3::new(100., 0., 0.))
            .with_scale(Vec3::splat(25.))
    ))
    .add_rollback();
}

/// Move players based on GGRS input.
fn move_players(
    mut players: Query<(&mut Transform, &components::Player)>,
    inputs: Res<PlayerInputs<networking::Config>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let mut direction = Vec2::ZERO;

        //Apply bitmasks to get the bit data for the specific direction
        if input & inputs::INPUT_UP != 0 {
            direction.y += 1.;
        }
        if input & inputs::INPUT_DOWN != 0 {
            direction.y -= 1.;
        }
        if input & inputs::INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & inputs::INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 300.;
        let move_delta = direction * move_speed * time.delta_secs();

        // transform.translation expects a 3d Vec3 so, extend our 2d Vec2 
        // into a 3d Vec3 by setting the z component to 0 since we are in 2d land
        // https://docs.rs/bevy/latest/bevy/math/struct.Vec2.html#method.extend
        transform.translation += move_delta.extend(0.);
    }
}

/// mode for no network dependencies.
fn local_mode(args: Res<args::UserInput>) -> bool {
    args.local_only
}

/// mode for live networking.
fn p2p_mode(args: Res<args::UserInput>) -> bool {
    !args.local_only
}
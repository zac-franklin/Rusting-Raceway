use super::{args, despawn_screen, components, inputs, networking, physics, resources, states};
use bevy::prelude::*;
use bevy_ggrs::prelude::*;
use bevy_polyline::prelude::*;

// Game specific setup
pub fn game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(states::GameState::Matchmaking), //TODO: create a matchmaking screen between game screen and move game setup here to OnEnter(gamestate::InGame)
            (
                spawn_stadium, 
                spawn_players.after(spawn_stadium),
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
        .add_systems(GgrsSchedule, move_players_along_tracks) 
        .add_systems(OnExit(states::GameState::InGame), despawn_screen::<components::OnGameScreen>);
}

/// Spawn stadium
fn spawn_stadium(
    mut commands: Commands, 
    mut paths: ResMut<resources::Paths>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let length = 500.;
    let inner_radius = 225.;
    let radius_ratio = 2.; // ratio between inner and outer radius
    let bend_sections: u32 = 20;
    let num_tracks = 4;
    debug_assert!(bend_sections % 2 == 0, "Section size needs to be even to preserve symmetry");
    debug_assert!(bend_sections > 2, "Section size needs to be greater than 2");

    // Spawn grass
    let grass_color = Color::hsl(99.0, 0.66, 0.55);
    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline { 
                vertices: physics::determine_track_points(
                    length, inner_radius, bend_sections
                )
            })),
            material: PolylineMaterialHandle(polyline_materials.add(
                PolylineMaterial {
                    width: 3.,
                    color: grass_color.into(),
                    perspective: false,
                    ..default()
                }
            )),
            ..default()
        },
        components::OnGameScreen,
    ));

    // Spawn track section
    let track_section_color = Color::hsl(35.0, 0.69, 0.63);
    commands.spawn((
        PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline { 
                vertices: physics::determine_track_points(
                    length, radius_ratio * inner_radius, bend_sections
                )
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: 3.,
                color: track_section_color.into(),
                perspective: false,
                ..default()
            })),
            ..default()
        },
        components::OnGameScreen,
    ));

    // Spawn individual tracks
    let ratio_increase = 1. / (num_tracks as f32 + 1.);
    let track_color = Color::hsl(35.0, 0.69, 0.32);
    paths.0.clear();
    for track_id in 0..num_tracks {
        // Determine points
        let ratio = 1. + (track_id as f32 + 1.) * ratio_increase;
        let radius = inner_radius * ratio;
        let vertices = physics::determine_track_points(
            length, radius, bend_sections
        );

        // Spawn tracks
        commands.spawn((
            PolylineBundle {
                polyline: PolylineHandle(polylines.add(Polyline { 
                    vertices: vertices.clone()
                })),
                material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                    width: 3.,
                    color: track_color.into(),
                    perspective: false,
                    ..default()
                })),
                ..default()
            },
            components::OnGameScreen,
        ));

        // Store points
        paths.0.push(vertices);
    }
}

/// Spawn players with GGRS Rollback
fn spawn_players(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    paths: Res<resources::Paths>,
) {
    let num_players = 2;
    let player_shape = meshes.add(Sphere::default().mesh().uv(32, 18));
    let player_color = Color::srgb(0., 0.47, 1.);
    
    // Iterate over player ids and spawn them
    for player_id in 0..num_players {
        let starting_pos = &paths.0[player_id][0];
        commands.spawn((
            components::Player{ 
                handle: player_id,
                pos_index: 0,
                distance: 0.0
            },
            components::OnGameScreen,
            Mesh3d(player_shape.clone()),
            MeshMaterial3d(materials.add(player_color)),
            Transform::from_translation(*starting_pos)
                .with_scale(Vec3::splat(25.))
        ))
        .add_rollback();
    }
}

/// Move players along the tracks based on GGRS input
fn move_players_along_tracks(
    mut players: Query<(&mut Transform, &mut components::Player)>,
    inputs: Res<PlayerInputs<networking::Config>>,
    paths: Res<resources::Paths>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut players {
        let (input, _) = inputs[player.handle];

        // Apply bitmasks to get the bit data
        if !(input & inputs::INPUT_FORWARD != 0) {
            continue;
        }

        // Update distance traveled
        player.distance += 250. * time.delta_secs();

        // Get player path
        let path = &paths.0[player.handle];

        // Determine distance between last track point and the next one
        let next_index = (player.pos_index + 1) % path.len();
        let mut last_point = path[player.pos_index];
        let mut next_point = path[next_index];
        let distance_to_next_point = next_point.distance(last_point);

        // If the distance the player has traveled is greater than the distance 
        // between the last point and the next point, then the player has moved past 
        // the next point
        if player.distance >= distance_to_next_point {
            player.pos_index = next_index;
            player.distance = 0.;
            last_point = path[next_index];
            next_point = path[(next_index + 1) % path.len()];
        }

        // Interpolate new position
        transform.translation = last_point.lerp(next_point, player.distance / distance_to_next_point);
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
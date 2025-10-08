use super::{components, inputs, networking, physics};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::prelude::*;
use bevy_polyline::prelude::*;

/// Setup the camera and view.
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1000.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Light source for 3d rendering
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1_000.,
        ..default()
    });
}

/// Spawn stadium
pub fn spawn_stadium(
    mut commands: Commands, 
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    let length = 500.;
    let inner_radius = 225.;
    let radius_ratio = 2.; // ratio between inner and outer radius
    let bend_sections: u32 = 20;
    debug_assert!(bend_sections % 2 == 0, "Section size needs to be even to preserve symmetry");

    // Spawn grass
    let grass_color = Color::hsl(99.0, 0.66, 0.55);
    commands.spawn(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline { 
            vertices: physics::determine_track_points(
                length, inner_radius, bend_sections
            )
        })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: 3.,
            color: grass_color.into(),
            perspective: false,
            ..default()
        })),
        ..default()
    });

    // Spawn tracks
    let track_color = Color::hsl(35.0, 0.69, 0.63);
    commands.spawn(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline { 
            vertices: physics::determine_track_points(
                length, radius_ratio * inner_radius, bend_sections
            )
        })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: 3.,
            color: track_color.into(),
            perspective: false,
            ..default()
        })),
        ..default()
    });
}

/// Spawn players with GGRS Rollback and their tracks (paths)
pub fn spawn_players(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
) {
    let bend_sections: u32 = 20;
    debug_assert!(bend_sections % 2 == 0, "Section size needs to be even to preserve symmetry");
    let inner_radius = 225.;
    let length = 500.;  // length of horizontal section (stadium)
    let num_players = 2;
    let num_tracks = 4;
    let ratio_increase = 1. / (num_tracks as f32 + 1.);
    let mut paths: Vec<Vec<Vec3>> = Vec::new();

    // Iterate over individual tracks and spawn them
    let track_color = Color::hsl(35.0, 0.69, 0.32);
    for track_id in 0..num_tracks {
        // Determine points
        let ratio = 1. + (track_id as f32 + 1.) * ratio_increase;
        let radius = inner_radius * ratio;
        let vertices = physics::determine_track_points(
            length, radius, bend_sections
        );

        // Spawn tracks
        commands.spawn(PolylineBundle {
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
        });

        // Store points
        paths.push(vertices);
    }

    // Iterate over player ids and spawn them
    let player_shape = meshes.add(Sphere::default().mesh().uv(32, 18));
    let player_color = Color::srgb(0., 0.47, 1.);
    for player_id in 0..num_players {
        commands.spawn((
            components::Player{ handle: player_id },
            components::Runner{ 
                path: paths[player_id].to_owned(),
                pos_index: 0,
                distance: 0.0
            },
            Mesh3d(player_shape.clone()),
            MeshMaterial3d(materials.add(player_color)),
            Transform::from_translation(paths[player_id][0])
                .with_scale(Vec3::splat(25.))
        ))
        .add_rollback();
    }
}

/// Move players along the tracks based on GGRS input
pub fn move_players_along_tracks(
    mut players: Query<(&mut Transform, &components::Player, &mut components::Runner)>,
    inputs: Res<PlayerInputs<networking::Config>>,
    time: Res<Time>,
) {
    for (mut transform, player, mut runner) in &mut players {
        let (input, _) = inputs[player.handle];

        // Apply bitmasks to get the bit data
        if !(input & inputs::INPUT_RIGHT != 0) {
            continue;
        }

        // Update distance traveled
        runner.distance += 250. * time.delta_secs();

        // Determine distance between last track point and the next one
        let next_index = (runner.pos_index + 1) % runner.path.len();
        let mut last_point = runner.path[runner.pos_index];
        let mut next_point = runner.path[next_index];
        let distance_to_next_point = next_point.distance(last_point);

        // If the distance the player has traveled is greater than the distance 
        // between the last point and the next point, then the player has moved past 
        // the next point
        if runner.distance >= distance_to_next_point {
            runner.pos_index = next_index;
            runner.distance = 0.;
            last_point = runner.path[next_index];
            next_point = runner.path[(next_index + 1) % runner.path.len()];
        }

        // Interpolate new position
        transform.translation = last_point.lerp(next_point, runner.distance / distance_to_next_point);
    }
}
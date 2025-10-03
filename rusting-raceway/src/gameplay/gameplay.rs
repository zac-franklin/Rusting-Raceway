// Standard imports
use std::f32::consts::FRAC_PI_2;

// Third party imports
use bevy::{prelude::*, color::palettes::css::RED, render::camera::ScalingMode};
use bevy_ggrs::prelude::*;
use bevy_polyline::prelude::*;

// Imports
use crate::networking;
use super::{components, constants, utils};

/// Setup the 3D camera
pub fn setup_camera(mut commands: Commands) {
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
}

/// Spawn stadium
pub fn spawn_stadium(
    mut commands: Commands, 
    stadium: Res<components::Stadium>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {

    // Spawn grass
    let grass_color = Color::hsla(
        constants::STADIUM_GRASS_COLOR.hue, 
        constants::STADIUM_GRASS_COLOR.saturation, 
        constants::STADIUM_GRASS_COLOR.lightness,
        constants::STADIUM_GRASS_COLOR.alpha
    );
    commands.spawn(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline { 
            vertices: utils::determine_track_points(
                stadium.inner_shape.length, 
                stadium.inner_shape.radius, 
                constants::STADIUM_SECTION_SIZE
            )
        })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: constants::PLAYER_TRACK_WIDTH,
            color: grass_color.into(),
            perspective: false,
            ..default()
        })),
        ..default()
    });

    // Spawn tracks
    let track_color = Color::hsla(
        constants::STADIUM_TRACK_COLOR.hue, 
        constants::STADIUM_TRACK_COLOR.saturation, 
        constants::STADIUM_TRACK_COLOR.lightness,
        constants::STADIUM_TRACK_COLOR.alpha
    );
    commands.spawn(PolylineBundle {
        polyline: PolylineHandle(polylines.add(Polyline { 
            vertices: utils::determine_track_points(
                stadium.inner_shape.length, 
                stadium.ratio * stadium.inner_shape.radius, 
                constants::STADIUM_SECTION_SIZE
            )
        })),
        material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
            width: constants::STADIUM_TRACK_WIDTH,
            color: track_color.into(),
            perspective: false,
            ..default()
        })),
        ..default()
    });
}

/// Spawn player tracks (paths)
pub fn spawn_player_tracks(
    mut commands: Commands, 
    stadium: Res<components::Stadium>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    assets: Res<AssetServer>
) {
    // Read stadium variables
    let length = stadium.inner_shape.length;
    let inner_radius = stadium.inner_shape.radius;
    let stadium_ratio = stadium.ratio;

    // Iterate over individual tracks and spawn them
    let starting_ratio = 1.0;
    let stadium_ratio_difference = stadium_ratio - starting_ratio;
    let ratio_increase = stadium_ratio_difference / (constants::STADIUM_NUM_TRACKS as f32 + 1.0);
    let color = Color::hsla(
        constants::PLAYER_TRACK_COLOR.hue, 
        constants::PLAYER_TRACK_COLOR.saturation, 
        constants::PLAYER_TRACK_COLOR.lightness,
        constants::PLAYER_TRACK_COLOR.alpha
    );

    let mut travel_distances: Vec<f32> = Vec::new();
    for track_no in 1..=constants::STADIUM_NUM_TRACKS {
        // Determine radius of track using ratio between current track and
        // stadium inner shape
        let ratio = starting_ratio + (track_no as f32) * ratio_increase;
        let radius = inner_radius * ratio;
        let vertices = utils::determine_track_points(
            length, radius, constants::STADIUM_SECTION_SIZE
        );

        // Store travel distances to calculate starting position of players
        travel_distances.push( utils::determine_path_distance(&vertices) );

        // Spawn track
        commands.spawn(PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline { 
                vertices: vertices
            })),
            material: PolylineMaterialHandle(polyline_materials.add(PolylineMaterial {
                width: constants::PLAYER_TRACK_WIDTH,
                color: color.into(),
                perspective: false,
                ..default()
            })),
            ..default()
        });
    }

    /* TODO: incorporate either here or in a separate function
    // Iterate over player id and spawn the players
    let starting_position = (0.5 - constants::PLAYER_START_PERC) * constants::STADIUM_LENGTH;
    let mut staggered_offsets: Vec<f32> = vec![0.0];
    let mut distances_diffs: Vec<f32> = travel_distances.windows(2)
        .map( |window| -(window[1] - window[0]) )
        .collect();
    staggered_offsets.extend(distances_diffs); 
    for player_id in 1..=networking::constants::NUM_PLAYERS {
        // Determine radius of track using ratio between current track and
        // stadium inner shape
        let ratio = starting_ratio + (player_id as f32) * ratio_increase;
        let radius = inner_radius * ratio;
        let offset = staggered_offsets[player_id - 1];

        commands.spawn((
            components::Player::new(player_id, radius, length),
            Sprite::from_image(assets.load(constants::PLAYER_SPRITE)),
            Transform::from_xyz(
                starting_position + offset, radius, 0.0
            ).with_scale( Vec3::splat(constants::PLAYER_SCALE) ),
        ));
    }*/
}


/*/// Spawn players
// pub fn spawn_players(
    mut commands: Commands, 
    stadium: Res<components::Stadium>,
    assets: Res<AssetServer>
) {
    // Read stadium variables
    let length = stadium.inner_shape.length;
    let inner_radius = stadium.inner_shape.radius;
    let stadium_ratio = stadium.ratio;

    // Iterate over player id and spawn the players
    let starting_ratio = 1.0;
    let stadium_ratio_difference = stadium_ratio - starting_ratio;
    let ratio_increase = stadium_ratio_difference / (constants::STADIUM_NUM_TRACKS as f32 + 1.0);

    for player_id in 1..=networking::constants::NUM_PLAYERS {
        // Determine radius of track using ratio between current track and
        // stadium inner shape
        let ratio = starting_ratio + (track_no as f32) * ratio_increase;
        let radius = inner_radius * ratio;

        commands.spawn((
            components::Player::new(handle, radius, length),
            Sprite::from_image(asset_server.load(constants::PLAYER_SPRITE)),
            Transform::from_xyz(
                constants::P1_START_X + player_id * 20.0, constants::P1_START_Y, constants::P1_START_Z // TODO: move offset to a variable
            ).with_scale( Vec3::splat(constants::PLAYER_SCALE) ),
        ));
    }
}*/

/*
/// Spawn the players with GGRS Rollback 
pub fn spawn_players(mut commands: Commands, assets: Res<AssetServer>) {
    // Iterate over num of players and spawn each
    for player_id in 0..constants::NUM_PLAYERS {
        commands.spawn((
            components::Player::new(
                handle, 
                constants::P1_TRACK_RADIUS, 
                constants::P1_TRACK_LENGTH
            ),
            Sprite::from_image(asset_server.load("icon.png")), // TODO: Should be some variable
            Transform::from_xyz(
                constants::P1_START_X + player_id * 20.0, constants::P1_START_Y, constants::P1_START_Z // TODO: move offset to a variable
            ).with_scale( Vec3::splat(constants::PLAYER_SCALE) ),
        )).add_rollback();
    }
}

/// Move players based on GGRS input
pub fn move_players(
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

        let move_speed = 7.;
        let move_delta = direction * move_speed * time.delta_secs();

        // transform.translation expects a 3d Vec3 so, extend our 2d Vec2 
        // into a 3d Vec3 by setting the z component to 0 since we are in 2d land
        // https://docs.rs/bevy/latest/bevy/math/struct.Vec2.html#method.extend
        transform.translation += move_delta.extend(0.);
    }
}*/
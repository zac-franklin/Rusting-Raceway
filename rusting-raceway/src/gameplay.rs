use super::{components, inputs, networking};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::prelude::*;

/// Setup the camera and view.
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// Spawn the players with GGRS Rollback in the game
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        components::Player{ handle: 0 },
        Transform::from_translation(Vec3::new(-2., 0., 0.)),
        Sprite {
            color: Color::srgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
    ))
    .add_rollback();

    // Player 2
    commands
        .spawn((
            components::Player{ handle: 1 },
            Transform::from_translation(Vec3::new(2., 0., 0.)),
            Sprite {
                color: Color::srgb(0., 0.4, 0.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
        ))
        .add_rollback();
}

/// Move players based on GGRS input.
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
        transform.translation += move_delta.extend(0.);
    }
}
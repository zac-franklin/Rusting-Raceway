use super::networking;
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ggrs::{LocalInputs, LocalPlayers};

// Clever way to store inpt efficiently bit masks that we later apply AND and OR opperations to get information back. 
pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;

/// Reads local input, applies OR masks to u8 array based on current input.
/// adds the input to the GGRS session.
pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0u8;

        //apply masks based on input.
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<networking::Config>(local_inputs));
}
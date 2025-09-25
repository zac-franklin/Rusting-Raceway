use bevy::prelude::*;
use clap::{Arg, Command, ArgAction, crate_version, crate_name};

/// Contorls dynamic user input required to run the game in the desired way.
#[derive(Resource)]
pub struct UserInput {
    /// runs the game in local mode
    pub local_only: bool,
}

impl UserInput {
    pub fn parse() -> UserInput {
        let matches = Command::new(crate_name!())
            .version(crate_version!())
            .author("zac-franklin and victored")
            .about("Bevy recreation of Mario Party's Rocking Raceway")
            .arg(
                Arg::new("local_only")
                    .help("run game without matchmaking")
                    .long("local_only")
                    .short('l')
                    .action(ArgAction::SetTrue)
            )
            .get_matches();

        UserInput{local_only: matches.get_flag("local_only")}
    }
}
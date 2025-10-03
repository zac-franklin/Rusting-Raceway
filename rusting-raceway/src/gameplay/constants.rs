//! Contains gameplay constants

// Color struct
pub struct Hsla {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32
}

/// Stadium constants
pub const STADIUM_GRASS_COLOR: Hsla = Hsla { hue: 99.0, saturation: 0.66, lightness: 0.55, alpha: 1.0 };
pub const STADIUM_INNER_RADIUS: f32 = 225.0;
pub const STADIUM_LENGTH: f32 = 500.0;
pub const STADIUM_NUM_TRACKS: usize = 4;
pub const STADIUM_SHAPE_RATIO: f32 = 2.0;
pub const STADIUM_SECTION_SIZE: u32 = 20;  // Needs to be even to preserve symmetry
pub const STADIUM_TRACK_WIDTH: f32 = 3.0;
pub const STADIUM_TRACK_COLOR: Hsla = Hsla { hue: 35.0, saturation: 0.69, lightness: 0.63, alpha: 1.0 };

/// Player constants
pub const PLAYER_MOVEMENT_COOLDOWN: f32 = 0.5;  // in seconds
pub const PLAYER_START_PERC: f32 = 0.5;  // percentage of section TODO: decide if keep/remove
pub const PLAYER_SCALE: f32 = 0.2;
pub const PLAYER_SPRITE: &str = "icon.png";
pub const PLAYER_START_ANGLE: f32 = -180.0;  // in degrees
pub const PLAYER_TRACK_WIDTH: f32 = 3.0;
pub const PLAYER_TRACK_COLOR: Hsla = Hsla { hue: 35.0, saturation: 0.69, lightness: 0.32, alpha: 1.0 };
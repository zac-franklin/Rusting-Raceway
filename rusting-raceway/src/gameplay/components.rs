//! Contains gameplay components

/// Third party imports
use bevy::prelude::{Component, Resource};

/// Imports
use super::{constants, utils};

/// Camera component for easy querying
#[derive(Component)]
pub struct GameCamera;

/// 2D Capsule shape for player paths and track stadium
#[derive(Clone)]
pub struct Capsule {
    pub length: f32,
    pub radius: f32,
}

/// Player component with handle ID and gameplay variables
#[derive(Component, Clone)]
pub struct Player {
    pub angle: f32,
    pub angular_speed: f32,
    pub handle: usize,
    pub linear_speed: f32,
    pub movement_cooldown: f32,
    pub path: Capsule,
    pub section: Section,
    pub tangential_speed: f32,
}

/// Stadium resource which we use to build player paths
#[derive(Resource)]
pub struct Stadium {
    pub inner_shape: Capsule,
    pub ratio: f32  // ratio between inner and outer capsule dimensions
}

// Represents a section of the track
#[derive(Clone, Copy)]
pub enum Section {
    Top,     // top horizontal line
    Left,    // left half circle
    Bottom,   // bottom horizontal line
    Right     // right half circle
}

// Struct implementations
impl Player {
    pub fn new(handle: usize, radius: f32, length: f32) -> Self {
        Player {
            angle: constants::PLAYER_START_ANGLE,
            angular_speed: utils::determine_angular_speed(constants::STADIUM_SECTION_SIZE),
            handle: handle,
            linear_speed: utils::determine_linear_speed(length, constants::STADIUM_SECTION_SIZE),
            movement_cooldown: constants::PLAYER_MOVEMENT_COOLDOWN,
            path: Capsule { length: length, radius: radius},
            section: Section::Top,
            tangential_speed: utils::determine_tangential_speed(radius, constants::STADIUM_SECTION_SIZE),
        }
    }
}

impl Default for Stadium {
    fn default() -> Self {
        Stadium {
            inner_shape: Capsule { 
                length: constants::STADIUM_LENGTH,
                radius: constants::STADIUM_INNER_RADIUS
            },
            ratio: constants::STADIUM_SHAPE_RATIO
        }
    }
}
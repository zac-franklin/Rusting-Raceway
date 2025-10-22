use std::f32::consts::PI;
use bevy::prelude::*;

/// Determines the incremental rotation angle needed to make a 180 degree turn
/// in a number of steps equal to the section size
pub fn determine_incremental_rotation_angle(bend_sections: u32) -> f32 {
    let angle_turn = 180.0;
    if !(bend_sections > 0) {
        return 0.0;
    }
    angle_turn / bend_sections as f32
}

/// Determines the bend section speed needed to travel a distance equal to the
/// perimeter of a half circle in a number of steps equal to the bend sections
pub fn determine_bend_section_length(radius: f32, bend_sections: u32) -> f32 {
    if !(bend_sections > 0) {
        return 0.0;
    }
    (radius * PI) / bend_sections as f32
}

/// Determines stadium path from a player path
pub fn determine_track_points(path: Vec<Vec3>) -> Vec<Vec3> {
    // Make a copy of the path
    let mut points = path.clone();

    // Reflect along y axis
    let mut reflections = points.clone();
    reflections.reverse();
    for pt in &mut reflections {
        pt.x *= -1.0;
    }
    points.extend(reflections);

    points
}


/// Determines a player's path along the stadium tracks
/// with the starting point corresponding to a staggered position and some endpoint
pub fn determine_player_path(
    bend_sections: u32, 
    start: Vec3,
    end: Vec3, 
    length: f32, 
    radius: f32, 
) -> Vec<Vec3> {
    let incremental_rotation_angle = determine_incremental_rotation_angle(bend_sections);
    let bend_section_length = determine_bend_section_length(radius, bend_sections);

    // Top section from start point until left bend
    let mut points: Vec<Vec3> = vec![ 
        start, 
        Vec3::new(-length / 2.0, radius, 0.0) 
    ];

    // Left bend from top to bend midpoint
    let mut angle = -180.0;
    let mut bend_position = Vec3::new(-length / 2.0, radius, 0.0);
    let mut bend_points = Vec::new();
    // We can obtain the rest of the bend by reflecting along the x axis.
    // To avoid backtracking, we start the loop at 2
    for _ in (2..bend_sections).step_by(2) {
        angle += incremental_rotation_angle;
        let direction = Vec2::from_angle(angle.to_radians()).extend(0.0);
        bend_position += direction * bend_section_length;
        bend_points.push( bend_position );
    }

    // Reflect the bend points to obtain the rest of the bend
    let mut reflections = bend_points.clone();
    reflections.reverse();
    for pt in &mut reflections {
        pt.y *= -1.0;
    }
    bend_points.extend(reflections);

    // Add bend points
    points.extend(bend_points);

    // Lastly we add the endpoint
    points.push(end);

    // Return path points
    points
}
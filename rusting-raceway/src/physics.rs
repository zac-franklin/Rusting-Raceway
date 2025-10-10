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

/// Determines stadium path from its length, radius and section size
pub fn determine_track_points(length: f32, radius: f32, bend_sections: u32) -> Vec<Vec3> {
    let incremental_rotation_angle = determine_incremental_rotation_angle(bend_sections);
    let bend_section_length = determine_bend_section_length(radius, bend_sections);

    // We determine the inner points of the top left half and then reflect the results
    // to obtain the other three quadrants
    let mut points: Vec<Vec3> = Vec::new();

    // Top section (left-half)
    points.extend(vec![
        Vec3::new(0.0, radius, 0.0), Vec3::new(-length / 2.0, radius, 0.0)
    ]);

    // Left section (top-half)
    let mut angle = -180.0;
    let mut position = Vec3 { x: -length / 2.0, y: radius, z: 0.0 };
    // Since we only want the upper left quadrant, we only need to go up to half 
    // the number of sections in the curve. To avoid backtracking, we start the loop at 2
    for _ in (2..bend_sections).step_by(2) {
        angle += incremental_rotation_angle;
        let direction = Vec2::from_angle(angle.to_radians()).extend(0.0);
        position += direction * bend_section_length;
        points.push( position );
    }

    // Reflect along x axis to obtain bottom left quadrant
    let mut reflections = points.clone();
    reflections.reverse();
    for pt in &mut reflections {
        pt.y *= -1.0;
    }
    points.extend(reflections);

    // Reflect along y axis to obtain remaining quadrants
    let mut reflections = points.clone();
    reflections.reverse();
    for pt in &mut reflections {
        pt.x *= -1.0;
    }
    points.extend(reflections);

    // Return points
    points
}
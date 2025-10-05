use std::f32::consts::PI;
use bevy::prelude::*;

/// Determines the angular speed needed to make a 180 degree turn
/// in a number of steps equal to the section size
pub fn determine_angular_speed(section_size: u32) -> f32 {
    let angle_turn = 180.0;
    if !(section_size > 0) {
        return 0.0;
    }
    angle_turn / section_size as f32
}

/// Determines the linear speed needed to travel a distance equal to the
/// length argument in a number of steps equal to the section size
pub fn determine_linear_speed(length: f32, section_size: u32) -> f32 {
    if !(section_size > 0) {
        return 0.0;
    }
    length / section_size as f32
}

/// Determines the tangential speed needed to travel a distance equal to the
/// perimeter of a half circle in a number of steps equal to the section size
pub fn determine_tangential_speed(radius: f32, section_size: u32) -> f32 {
    if !(section_size > 0) {
        return 0.0;
    }
    (radius * PI) / section_size as f32
}

/// Determines stadium path from its length, radius and section size
pub fn determine_track_points(length: f32, radius: f32, section_size: u32) -> Vec<Vec3> {
    let angular_speed = determine_angular_speed(section_size);
    let tangential_speed = determine_tangential_speed(radius, section_size);

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
    for _ in (0..section_size).step_by(2) {
        angle += angular_speed;
        let direction = Vec2::from_angle(angle.to_radians()).extend(0.0);
        position += direction * tangential_speed;
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
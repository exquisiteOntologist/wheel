use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use cgmath::{Angle, InnerSpace, Rotation3};

/// Degrees to Radians
/// https://clickcalculators.com/degree-to-radian-converter/90
pub fn degrees_to_radians(deg: f32) -> f32 {
    (deg * PI) / 180.
}

/// Get a single positive XYZ axis value of a quaternion using a W value.
/// The W value must be between -1. and 1.
/// Returns a positive value between 0. and 1.
pub fn quat_w_to_axis_adjust(w: f32) -> f32 {
    let theta = cgmath::Rad::acos(w) * 2.;
    let sin = cgmath::Rad::sin(theta.normalize() * 0.5);

    println!("w {}", w);
    // println!("theta {}", theta.0);
    println!("sin {}", sin);

    // the XYZ vector (whichever it is meant to be)
    let out = 1. * sin;
    out
}

/// Get a single positive XYZ axis value of a quaternion using a W value.
/// The W value must be between -1. and 1.
/// Returns a positive value between 0. and 1.
pub fn quat_w_to_axis_adjust_v(w: f32, v: f32) -> (f32, f32) {
    let theta = cgmath::Rad::acos(w) * 2.;
    let sin = cgmath::Rad::sin(theta / 2.);

    println!("w {}", w);
    println!("theta {}", theta.0);
    println!("sin {}", sin);

    // the XYZ (whichever it is meant to be)
    let out = v * sin;
    (theta.0, out)
}

/// Create a positive Y rotation from an input between 0. and 1.
/// Note the Y value change from W is linear, so there is no amplification
/// at certain degrees.
pub fn rotate_y_by_w(w: f32) -> Quat {
    // Ensure w is within the expected range
    assert!(w >= 0.0 && w <= 1.0, "w must be between 0 and 1");

    // Calculate the angle in radians (360 degrees = 2 * PI radians)
    let angle = w * 2.0 * std::f32::consts::PI;

    // Create a quaternion representing the rotation around the Y axis
    Quat::from_axis_angle(Vec3::Y, angle)
}

#[test]
fn test_rotate_y_by_w() {
    let range: Vec<f32> = (0..10).map(|v| v as f32 / 10.).collect();
    for w in range {
        println!("i {}", w);
        let quat = rotate_y_by_w(w);
        println!("quat {}", quat);

        assert!(w >= 0. && w <= 1., "w out of range");
        // assert!(quat.w >= 0. && quat.w <= 1., "quat.w out of range");
        assert!(quat.y >= 0. && quat.y <= 1., "quat.y out of range");
    }
}

// #[test]
// fn test_rotate_y_by_w_consecutive() {
//     let range: Vec<f32> = (0..10).map(|v| v as f32 / 10.).collect();
//     let mut last_quat: Option<Quat> = None;
//     for i in range {
//         println!("i {}", i);
//         let mut w = i;
//         if let Some(last_quat) = last_quat.clone() {
//             w = ((last_quat.w + 0.1) % 1.).clamp(0., 1.);
//         }
//         let rot_new = rotate_y_by_w(w);
//         let quat = if Some(last_quat) = last_quat {
//             last_quat.rot
//         }
//         println!("quat {}", quat);

//         assert!(w >= 0. && w <= 1., "w out of range");
//         // assert!(quat.w >= 0. && quat.w <= 1., "quat.w out of range");
//         assert!(quat.y >= 0. && quat.y <= 1., "quat.y out of range");

//         last_quat = Some(quat);
//     }
// }

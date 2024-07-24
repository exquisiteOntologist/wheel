use std::f32::consts::PI;

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
    assert!(w >= -1. && w <= 1., "w param out of range");

    let theta = cgmath::Rad::acos(w) * 2.;
    let sin = cgmath::Rad::sin(theta / 2.);

    // // between -1 and 1
    // let percentage = w;
    // let theta = cgmath::Rad::acos(percentage) * 2.;
    // let sin = cgmath::Rad::sin(theta / 2.);

    println!("w {}", w);
    println!("theta {}", theta.0);
    println!("sin {}", sin);

    // the XYZ (whichever it is meant to be)
    let out = 1. * sin;
    out
}

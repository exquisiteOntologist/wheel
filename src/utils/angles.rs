use std::f32::consts::PI;

/// Degrees to Radians
/// https://clickcalculators.com/degree-to-radian-converter/90
pub fn degrees_to_radians(deg: f32) -> f32 {
    (deg * PI) / 180.
}

use bevy::{
    math::{Dir3, Vec3},
    prelude::Res,
    time::Time,
};

use super::constants::GRAVITY_DIR;

/// Translate in the current direction at speed.
/// The translation is relative to Zero,
/// so it must be added to the target's translation.
pub fn move_dir_translate(dir: Dir3, speed: f32) -> Vec3 {
    let movement = Vec3::ZERO + dir * speed;
    // we presume the speed has the time.delta_second() applied already
    movement
}

pub fn move_gravity_translate(gravity_acceleration: f32, time: Res<Time>) -> Vec3 {
    let translation = gravity_acceleration * GRAVITY_DIR * time.delta_secs();
    translation
}

// Subtract translation vector B from A.
pub fn diff_translations(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z)
}

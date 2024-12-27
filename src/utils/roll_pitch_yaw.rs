use bevy::math::Quat;
use cgmath::{Angle, Rad};

#[derive(Default)]
pub struct RPY {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

/// quat2eulers.py
/// https://gist.github.com/michaelwro/1450283a6a1226eaf707d9adde378798
///
/// Compute yaw-pitch-roll Euler angles from a quaternion.
///
/// Args:
/// q0: Scalar component of quaternion.
/// q1: X
/// q2: Y
/// q3: Z
///
/// Returns:
/// (roll: f32, pitch: f32, yaw: f32): 321 Euler angles in radians
///
pub fn roll_pitch_yaw(q0: f32, q1: f32, q2: f32, q3: f32) -> (f32, f32, f32) {
    let roll = Rad::atan2(2.0 * (q0 * q1 + q2 * q3), 1.0 - 2.0 * (q1 * q1 + q2 * q2)).0;
    let pitch = Rad::asin(2.0 * (q0 * q2 - q3 * q1)).0;
    let yaw = Rad::atan2(2.0 * (q0 * q3 + q1 * q2), 1.0 - 2.0 * (q2 * q2 + q3 * q3)).0;
    (roll, pitch, yaw)
}

pub fn roll_pitch_yaw_from_quat(q: Quat) -> (f32, f32, f32) {
    roll_pitch_yaw(q.x, q.y, q.z, q.w)
}

/// Create quaternion values from roll, pitch, yaw
pub fn quaternion_from_rpy(roll: f32, pitch: f32, yaw: f32) -> (f32, f32, f32, f32) {
    let (sr, cr) = (Rad(roll).0 * 0.5).sin_cos();
    let (sp, cp) = (Rad(pitch).0 * 0.5).sin_cos();
    let (sy, cy) = (Rad(yaw).0 * 0.5).sin_cos();

    let q0 = cr * cp * cy + sr * sp * sy;
    let q1 = sr * cp * cy - cr * sp * sy;
    let q2 = cr * sp * cy + sr * cp * sy;
    let q3 = cr * cp * sy - sr * sp * cy;

    if q0.is_nan() || q1.is_nan() || q2.is_nan() || q3.is_nan() {
        eprintln!("NAN Error {:1} {:2} {:3} {:4}", q0, q1, q2, q3);
        panic!("Encountered a NANNY");
    }

    (q0, q1, q2, q3)
}

pub fn quaternion_from_rpy_quat(roll: f32, pitch: f32, yaw: f32) -> Quat {
    let (q0, q1, q2, q3) = quaternion_from_rpy(roll, pitch, yaw);

    Quat::from_xyzw(q0, q1, q2, q3)
}

use bevy::prelude::Resource;

use crate::utils::roll_pitch_yaw::RPY;

#[derive(Resource, Default)]
pub struct WheelState {
    /// Roll is tilting sideways,
    /// Pitch is rolling the wheel,
    /// Yaw is turning to another direction
    pub rpy: RPY,
}

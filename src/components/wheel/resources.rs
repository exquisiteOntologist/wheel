use bevy::prelude::{Component, Resource};

use crate::utils::roll_pitch_yaw::RPY;

#[derive(Component)]
pub struct PlayerWheel;

#[derive(Resource, Default)]
pub struct WheelState {
    /// Roll is tilting sideways,
    /// Pitch is rolling the wheel,
    /// Yaw is turning to another direction
    pub rpy: RPY,
}

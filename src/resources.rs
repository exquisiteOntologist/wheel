use bevy::ecs::{component::Component, system::Resource};

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct WheelParticles;

/// struct for marking terrain that contains the player
#[derive(Component)]
pub struct ContainsPlayer(pub bool);

#[derive(Default)]
pub struct MotionState {
    pub speed_x: f32,
    pub speed_y: f32,
    pub speed_z: f32,
}

#[derive(Resource, Default)]
pub struct Game {
    pub camera: MotionState,
    pub player_wheel: MotionState,
}

#[derive(Resource, Default)]
pub struct PlayState {
    pub paused: bool,
}

#[derive(Resource, Default)]
pub struct DebugState {
    /// Reset the player's position
    pub reset: bool,
}

#[derive(Resource, Default)]
pub struct DebugRoller {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

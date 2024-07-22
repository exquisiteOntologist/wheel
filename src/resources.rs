use bevy::{
    animation::AnimationClip,
    asset::Handle,
    ecs::{component::Component, system::Resource},
};

#[derive(Resource)]
pub struct Animations(pub(crate) Vec<Handle<AnimationClip>>);

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct PlayerWheel;

#[derive(Component)]
pub struct PlayerParticles;

#[derive(Component)]
pub struct WheelParticles;

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

use bevy::{
    asset::AssetServer,
    prelude::{Commands, Entity, Transform},
    scene::SceneRoot,
};

use super::resources::PlayerWheel;

pub fn spawn_wheel(commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    let child_wheel = commands
        .spawn((
            SceneRoot(asset_server.load("models/Wheel.glb#Scene0")),
            // also see parent character Y position
            Transform::IDENTITY,
            PlayerWheel,
        ))
        .id();

    child_wheel
}

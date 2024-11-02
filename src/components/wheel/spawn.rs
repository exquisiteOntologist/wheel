use bevy::{
    asset::AssetServer,
    prelude::{Commands, Entity, Transform},
    scene::SceneBundle,
    utils::default,
};

use super::resources::PlayerWheel;

pub fn spawn_wheel(commands: &mut Commands, asset_server: &AssetServer) -> Entity {
    let child_wheel = commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("models/Wheel.glb#Scene0"),
                // also see parent character Y position
                transform: Transform::IDENTITY,
                ..default()
            },
            PlayerWheel,
        ))
        .id();

    child_wheel
}

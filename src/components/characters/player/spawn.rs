use bevy::{
    asset::AssetServer,
    core::Name,
    math::Quat,
    prelude::{BuildChildren, Commands, Entity, Res, SpatialBundle},
    utils::default,
};
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};

use crate::{components::wheel::spawn::spawn_wheel, constants::SPAWN_TRANSFORM};

use super::resources::PlayerCharacter;

/// Spawn player. Can be called from a lifecycle.
pub fn spawn_player_with_wheel(mut commands: Commands, asset_server: Res<AssetServer>) {
    let child_wheel = spawn_wheel(&mut commands, &asset_server);
    let _player_character = spawn_player(&mut commands, child_wheel);
}

/// Spawn player. This function cannot be provided to a lifecycle directly.
pub fn spawn_player(commands: &mut Commands, child_wheel: Entity) -> Entity {
    let mut player_character = commands.spawn((
        SpatialBundle {
            // TODO: this transform needs to be applied in a level-specific function
            transform: SPAWN_TRANSFORM.with_rotation(Quat::from_rotation_y(-60.)),
            // transform: Transform::IDENTITY,
            ..default()
        },
        PlayerCharacter,
    ));

    player_character
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(1.0))
        .insert(KinematicCharacterController::default())
        .insert(Name::new("Player"));

    player_character.add_child(child_wheel);

    player_character.id()
}

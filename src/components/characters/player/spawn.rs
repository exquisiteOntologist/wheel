use bevy::{
    core::Name,
    math::Quat,
    prelude::{BuildChildren, Commands, Entity, SpatialBundle, Transform},
    utils::default,
};
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};

use crate::constants::SPAWN_TRANSFORM;

use super::resources::PlayerCharacter;

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

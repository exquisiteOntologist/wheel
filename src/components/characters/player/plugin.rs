use bevy::{
    app::{App, Plugin, PostStartup, Update},
    hierarchy::BuildChildren,
    math::{Dir3, Quat, Vec3},
    prelude::{Commands, EntityRef, Mut, Query, Res, ResMut, SpatialBundle, With, Without},
    time::Time,
    transform::components::Transform,
    utils::default,
};
use bevy_hanabi::{EffectProperties, EffectSpawner};
use bevy_rapier3d::prelude::KinematicCharacterController;

use crate::{
    constants::MAX_SPEED,
    gens::particles::{ParticlesPlugin, MAX_SAND_RATE},
    resources::{DebugRoller, Game, WheelParticles},
    utils::{
        angles::{degrees_to_radians, quat_w_to_axis_adjust, quat_w_to_axis_adjust_v},
        matrix::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
    },
};

use super::{
    effects::{attach_particles, update_particles_relative_to_char},
    movement::{move_character, turn_character},
};

pub struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_character,
                turn_character,
                update_particles_relative_to_char,
                // update_axis,
            ),
        );
        app.add_plugins(ParticlesPlugin);
        app.add_systems(PostStartup, attach_particles);
    }
}

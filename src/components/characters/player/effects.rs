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
    resources::{DebugRoller, Game, PlayerWheel, WheelParticles},
    utils::{
        angles::{degrees_to_radians, quat_w_to_axis_adjust, quat_w_to_axis_adjust_v},
        matrix::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
    },
};

use super::resources::{PlayerCharacter, PlayerParticles};

/// Add particles to the character.
/// Note that for the query to work we probably have to
/// run this post-startup.
pub fn attach_particles(
    mut commands: Commands,
    mut q_character: Query<EntityRef, With<PlayerCharacter>>,
    q_particles: Query<EntityRef, With<WheelParticles>>,
) {
    let mut particles = q_particles.iter();

    // each entity gets its own particles emitter,
    // as each entity spawns a particles instance
    for entity in q_character.iter_mut() {
        let particles_bundle = commands
            .spawn((SpatialBundle { ..default() }, PlayerParticles))
            .add_child(particles.next().unwrap().id())
            .id();

        commands
            .get_entity(entity.id())
            .unwrap()
            .add_child(particles_bundle);

        // commands
        //     .get_entity(entity.id())
        //     .unwrap()
        //     .add_child(particles.next().unwrap().id());
    }
}

pub fn update_particles_relative_to_char(
    time: Res<Time>,
    mut commands: Commands,
    // mut q_character: Query<(&mut Transform, &PlayerCharacter)>,
    mut q_character: Query<(&mut Transform, &PlayerWheel)>,
    // mut q_particles: Query<(&mut Transform, &PlayerParticles), Without<PlayerWheel>>,
    mut q_particles: Query<
        (&mut Transform, &mut EffectProperties, &WheelParticles),
        Without<PlayerWheel>,
    >,
    mut q_spawner: Query<&mut EffectSpawner>,
    mut game: ResMut<Game>,
) {
    let mut particle_emitters = q_particles.iter_mut();
    let mut effect_spawners = q_spawner.iter_mut();

    for character in q_character.iter_mut() {
        let (mut p_t, mut p_ep, _) = particle_emitters.next().unwrap();
        let Some(mut e_s) = effect_spawners.next() else {
            // println!("No spawners");
            // On startup the spawners may not yet exist.
            return;
        };
        let x = game.player_wheel.speed_z / (MAX_SPEED * time.delta_seconds());
        e_s.set_active(x > 1.);
        let rate: f32 = (1. - x) / MAX_SAND_RATE;
        e_s.spawner().with_count(rate.into());
        p_ep.set("opacity", (x / 2.).into());
    }
}

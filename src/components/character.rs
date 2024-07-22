use std::default;

use bevy::{
    app::{App, Plugin, PostStartup, Update},
    hierarchy::BuildChildren,
    math::{Dir3, Vec3},
    prelude::{Commands, EntityRef, Query, Res, ResMut, SpatialBundle, With, Without},
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{
    gens::particles::ParticlesPlugin,
    resources::{Game, PlayerCharacter, PlayerParticles, WheelParticles},
    utils::{angles::degrees_to_radians, matrix::quaternion_from_rpy_quat},
};

use super::wheel::{wheel_y_rotation, WheelState};

pub fn move_character(
    // this may have to be global transform
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    // to find the direction
    mut wheel: ResMut<WheelState>,
) {
    let mut t = q.single_mut();

    // TURNING DIRECTION
    // let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());
    // let updated_rot_quat = quaternion_from_rpy_quat(0., pitch, 0.);
    // t.rotation = t.rotation.normalize();
    // t.rotate(updated_rot_quat);
    // let updated_rot_quat = quaternion_from_rpy_quat(0., -wheel.rpy.pitch, 0.);
    // t.rotation = t.rotation.normalize();
    // t.rotate(updated_rot_quat);

    // t.rotate_local_y(wheel.rpy.pitch * 0.01);

    let turn_speed = 0.01;
    let turn_factor = if game.player_wheel.speed_y == 0. {
        0.
    } else if game.player_wheel.speed_y > 0. {
        1.
    } else {
        -1.
    };

    // because this rotation is relative and not absolute
    t.rotate_local_y(turn_speed * turn_factor);

    println!("parent rotation {}", t.rotation);

    let speed = game.player_wheel.speed_z;
    let rotation = wheel_y_rotation(&t.rotation).normalize();
    if let Ok(direction) = Dir3::new(rotation * -Vec3::Z) {
        // t.translation += direction * speed;
        let f = t.right();
        t.translation += f * speed;
        // t.translation.y = 2.1;
        // t.translation.z += 0.01;
        //
        // ^ This wheel should not move,
        // it instead should be parented,
        // with the parent being them mover
    }

    println!("char rotation {}", t.rotation);
    println!("char translation {}", t.translation);
}

/// Add particles to the character.
/// Note that for the query to work we probably have to
/// run this post-startup.
fn add_particles(
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

pub fn update_particles(
    mut commands: Commands,
    mut q_character: Query<(&mut Transform, &PlayerCharacter)>,
    mut q_particles: Query<(&mut Transform, &WheelParticles), Without<PlayerCharacter>>,
) {
    let mut particle_emitters = q_particles.iter_mut();

    for character in q_character.iter_mut() {
        let mut particles = particle_emitters.next().unwrap().0;
        // particles.translation = character.0.translation;
        println!("particles xyz {}", particles.translation);

        // The center & origin of the effect modifiers probably need to change,
        // instead of the transform of the particles

        // let rot = if character.0.translation.x > 0. {
        //     1.
        // } else {
        //     0.
        // };

        // let updated_rot_quat = quaternion_from_rpy_quat(degrees_to_radians(180. * rot), 0., 0.);
        // particles.rotation = particles.rotation.normalize();
        // particles.rotate(updated_rot_quat);
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_character, update_particles));
        app.add_plugins(ParticlesPlugin);
        app.add_systems(PostStartup, add_particles);
    }
}

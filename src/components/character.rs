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
use bevy_hanabi::EffectProperties;

use crate::{
    components::wheel::wheel_x_rotation,
    gens::particles::ParticlesPlugin,
    resources::{Game, PlayerCharacter, PlayerParticles, PlayerWheel, WheelParticles},
    utils::{
        angles::degrees_to_radians,
        matrix::{quaternion_from_rpy, quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
    },
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

    // println!("parent rotation {}", t.rotation);

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

    // println!("char rotation {}", t.rotation);
    // println!("char translation {}", t.translation);
}

/// Add particles to the character.
/// Note that for the query to work we probably have to
/// run this post-startup.
fn attach_particles(
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
    mut commands: Commands,
    // mut q_character: Query<(&mut Transform, &PlayerCharacter)>,
    mut q_character: Query<(&mut Transform, &PlayerWheel)>,
    // mut q_particles: Query<(&mut Transform, &PlayerParticles), Without<PlayerWheel>>,
    mut q_particles: Query<(&mut Transform, &WheelParticles), Without<PlayerWheel>>,
) {
    let mut particle_emitters = q_particles.iter_mut();

    for character in q_character.iter_mut() {
        let mut particles = particle_emitters.next().unwrap().0;
        // particles.translation = character.0.translation;
        // println!("particles xyz {}", particles.translation);
        // particles.rotate_y(1.);
        // particles.rotate_x(1.);
        // particles.rotate_z(1.);
        //

        // particles.rotation = character.0.rotation;
        // particles.rotation.y = character.0.rotation.y;
        // particles.rotate_y(0.1);
        // particles.rotation = particles.rotation.normalize();
        // particles.translation.x = 15.;
        // particles.translation.z = 15.;
        // particles.translation = character.0.translation;

        println!("particles rot {}", particles.rotation);

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

fn update_axis(
    time: Res<Time>,
    mut query: Query<(&mut EffectProperties, &mut Transform), With<WheelParticles>>,
    mut q_character: Query<(&mut Transform, &PlayerCharacter), Without<WheelParticles>>,
) {
    let (mut properties, mut transform) = query.single_mut();

    let mut c_t = q_character.single().0.clone();

    return;

    // println!("char quat {}", c_t.rotation);
    // let rot = c_t.rotation.inverse().xyz();
    // properties.set("pos_axis", rot.into());
    // properties.set("pos_center", rot.into());
    // println!("rot {}", rot);
    // return;

    // let rotation = c_t.rotation.normalize().xyz();
    // // let mut pos_axis = rotation.clone().to_axis_angle().0;
    // // let mut center = rotation.clone().to_axis_angle().0;
    // properties.set("pos_axis", rotation.into());
    // properties.set("pos_center", rotation.into());
    // return;

    // c_t.translation.x = 20.;
    // c_t.translation.z = 20.;

    // c_t.rotate_z(degrees_to_radians(90.));
    //
    let updated_rot_quat = quaternion_from_rpy_quat(0., 0., degrees_to_radians(90.));
    c_t.rotation = c_t.rotation.normalize();
    c_t.rotate(updated_rot_quat);

    let (rotation, angle) = c_t.rotation.to_axis_angle();
    // let (rotation, angle) = transform.rotation.to_axis_angle();

    // let (rotation, angle) = q_character.single_mut().0.rotation.to_axis_angle();

    let mut pos_axis = c_t.rotation.clone().to_axis_angle().0;
    // let pos_axis = c_t.translation;
    // pos_axis.z = 0.;
    // pos_axis.y = 0.;
    // properties.set("pos_axis", pos_axis.into());
    let mut center = c_t.rotation.clone().to_axis_angle().0;
    // let mut center = c_t.translation.clone();
    // center.z = -1.;
    // center.y = 0.;
    // properties.set("pos_center", pos_axis.into());
    println!("pos axis {}", pos_axis);
    println!("pos center {}", center);
    let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(c_t.rotation);
    println!("axis rpy {:1} {:2} {:3}", roll, pitch, yaw);
    let new_quat = quaternion_from_rpy_quat(roll, pitch, 0.);
    let mut pos_axis = new_quat.clone().to_axis_angle().0;
    let mut center = new_quat.clone().to_axis_angle().0;

    println!("pos axis {}", pos_axis);
    println!("pos center {}", center);
    // if pos_axis.y < 0. {
    //     pos_axis.y = pos_axis.y * -1.;
    //     center.y = center.y * -1.;
    //     println!("pos axis {}", pos_axis);
    //     println!("pos center {}", center);
    //     println!("y reversed");
    // }
    properties.set("pos_axis", pos_axis.into());
    properties.set("pos_center", center.into());

    // transform.translation.y = 0.;

    // c_t.rotate_x(0.1);
    // c_t.rotate_y(0.1);
    // c_t.rotate_z(0.1);
    //
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_character,
                update_particles_relative_to_char,
                update_axis,
            ),
        );
        app.add_plugins(ParticlesPlugin);
        app.add_systems(PostStartup, attach_particles);
    }
}

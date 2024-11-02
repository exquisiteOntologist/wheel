use bevy::{
    hierarchy::BuildChildren,
    math::Quat,
    prelude::{Commands, EntityRef, Query, Res, ResMut, SpatialBundle, With, Without},
    time::Time,
    transform::components::Transform,
    utils::default,
};
use bevy_hanabi::{EffectProperties, EffectSpawner};

use crate::{
    constants::{HEIGHT_TEMPERATE_START, MAX_SPEED},
    gens::particles::sand_particles::MAX_SAND_RATE,
    resources::{DebugRoller, Game, WheelParticles},
    utils::{
        angles::{degrees_to_radians, quat_w_to_axis_adjust},
        roll_pitch_yaw::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
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
    commands: Commands,
    mut q_character: Query<(&mut Transform, &PlayerCharacter)>,
    // mut q_wheel: Query<(&mut Transform, &PlayerWheel)>,
    // mut q_particles: Query<(&mut Transform, &PlayerParticles), Without<PlayerWheel>>,
    mut q_particles: Query<
        (&mut Transform, &mut EffectProperties, &WheelParticles),
        Without<PlayerCharacter>,
    >,
    mut q_spawner: Query<&mut EffectSpawner>,
    game: ResMut<Game>,
) {
    let mut particle_emitters = q_particles.iter_mut();
    let mut effect_spawners = q_spawner.iter_mut();
    for character in q_character.iter_mut() {
        let (p_t, mut p_ep, _) = particle_emitters.next().unwrap();
        let Some(mut e_s) = effect_spawners.next() else {
            // On startup the spawners may not yet exist.
            return;
        };
        let x = game.player_wheel.speed_z / (MAX_SPEED * time.delta_seconds());
        let moving = x > 1.;
        let on_sand = character.0.translation.y < HEIGHT_TEMPERATE_START;
        let show_particles = moving && on_sand;

        e_s.set_active(show_particles);

        if show_particles {
            let rate: f32 = (1. - x) / MAX_SAND_RATE;
            e_s.spawner().with_count(rate.into());
            p_ep.set("opacity", (x / 2.).into());
        } else {
            e_s.spawner().with_count((0.).into());
            p_ep.set("opacity", (0.).into());
        }
    }
}

fn update_particles_axis(
    time: Res<Time>,
    mut query: Query<(&mut EffectProperties, &mut Transform), With<WheelParticles>>,
    q_character: Query<(&mut Transform, &PlayerCharacter), Without<WheelParticles>>,
    d_r: ResMut<DebugRoller>,
) {
    let (mut properties, transform) = query.single_mut();

    let c_t = q_character.single().0.clone();

    // This value does not have to change as much as the character's rotation,
    // it can remain relatively static for most of a turn
    let rot_quat = Quat::from_xyzw(d_r.x, d_r.y, d_r.z, d_r.w);
    let rot = rot_quat.xyz();

    let rot_axis = Quat::from_xyzw(0., 1., 0., 0.);

    let rot_quat = rot_axis.mul_quat(Quat::from_xyzw(0., 0., 0., d_r.w));
    let rot = rot_quat.xyz();

    // c_t = c_t.with_rotation(rot);
    // let mut rot = c_t.rotation.inverse().xyz();

    // rot.x = d_r.x;
    // rot.z = d_r.z;

    properties.set("pos_axis", rot.into());
    properties.set("pos_center", rot.into());

    println!("char tran {}", c_t.translation);
    println!("char rot {}", c_t.rotation);
    println!("char rot inv {}", c_t.rotation.inverse());
    println!("axis rot quat {}", rot_quat);
    println!("axis rot xyz {}", rot);
    println!("===");

    // Experiment with Theta Sin
    let out = quat_w_to_axis_adjust(d_r.w);
    println!("experiment out {}", out);

    return;

    // println!("char quat {}", c_t.rotation);
    // let mut rot = c_t.rotation.inverse().xyz();
    // rot.x = 0.;
    // // rot.y = 0.;
    // rot.z = 0.;
    // if rot.y < 0. {
    //     rot.x = degrees_to_radians(180.);
    //     rot.z = degrees_to_radians(180.);
    // } else {
    //     rot.x = degrees_to_radians(180.);
    //     // rot.z = degrees_to_radians(180.);
    // }
    // c_t.rotation.w;
    // // c_t.rotate_arou
    // // let mut rot = c_t.rotation.normalize().xyz();
    // println!("w {}", c_t.rotation.w);
    // properties.set("pos_axis", rot.into());
    // properties.set("pos_center", rot.into());
    // println!("rot {}", rot);
    // println!("quat {}", c_t.rotation.inverse());
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

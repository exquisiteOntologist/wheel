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
    components::wheel::{wheel_y_rotation, WheelState},
    constants::MAX_SPEED,
    gens::particles::{ParticlesPlugin, MAX_SAND_RATE},
    resources::{DebugRoller, Game, WheelParticles},
    utils::{
        angles::{degrees_to_radians, quat_w_to_axis_adjust, quat_w_to_axis_adjust_v},
        matrix::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
    },
};

use super::resources::PlayerCharacter;

pub fn turn_character(mut q: Query<&mut Transform, With<PlayerCharacter>>, game: ResMut<Game>) {
    let mut t = q.single_mut();

    // For reference, this works except results in negative Y values that cause issues
    t.rotate_local_y(game.player_wheel.speed_y);
}

fn turn_character_old(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    // to find the direction
    mut wheel: ResMut<WheelState>,
    mut d_r: ResMut<DebugRoller>,
) {
    let mut t = q.single_mut();

    // let turn_speed = 0.01;
    // let turn_factor = if game.player_wheel.speed_y == 0. {
    //     0.
    // } else if game.player_wheel.speed_y > 0. {
    //     1.
    // } else {
    //     -1.
    // };

    // // because this rotation is relative and not absolute
    // let new_turn = turn_speed * turn_factor;

    // For reference, this works except results in negative Y values that cause issues
    t.rotate_local_y(game.player_wheel.speed_y);

    // this function suffers from a jump
    // char_rotation_positive_y(&mut t, new_turn);
    // char_rotation_positive_y_experiment(&mut t, new_turn);

    // println!("parent rotation {}", t.rotation);
}

fn move_character_old(
    mut q: Query<(&mut Transform, &mut KinematicCharacterController), With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    let (mut t, mut c) = q.single_mut();

    let speed = game.player_wheel.speed_z;
    let f = t.right();
    // t.translation += f * speed;
    c.translation = Some(Vec3::ZERO + f * speed);
}

/// Rotate by adjusting the quaternion angle on the y vector.
/// This rotation suffers from an amplification effect near a certain point.
/// The amplification effect makes the turn rush past about 90deg.
fn char_rotation_positive_y(t: &mut Mut<Transform>, new_turn: f32) {
    let curr_w = t.rotation.w;
    let new_w = ((((curr_w + new_turn) + 1.) % 2.) - 1.).clamp(-1., 1.);

    println!("new w {}", new_w);

    let y = quat_w_to_axis_adjust(new_w);

    let rot = Quat::from_xyzw(0., y, 0., new_w);
    t.rotation = rot.normalize();
}

fn char_rotation_positive_y_experiment(t: &mut Mut<Transform>, new_turn: f32) {
    let curr_w = t.rotation.w;
    let new_w = ((((curr_w + new_turn) + 1.) % 2.) - 1.).clamp(-1., 1.);
    let percent = (new_w + 1.) / 2.;
    let y_turn = (percent - t.rotation.y) % 1.;

    t.rotate_y(y_turn);
}

fn char_move_in_y_direction(t: &mut Mut<Transform>, mut game: ResMut<Game>) {
    let speed = game.player_wheel.speed_z;
    let rotation = wheel_y_rotation(&t.rotation).normalize();
    if let Ok(direction) = Dir3::new(rotation * -Vec3::Z) {
        t.translation += direction * speed;
    }
}

fn update_axis(
    time: Res<Time>,
    mut query: Query<(&mut EffectProperties, &mut Transform), With<WheelParticles>>,
    mut q_character: Query<(&mut Transform, &PlayerCharacter), Without<WheelParticles>>,
    mut d_r: ResMut<DebugRoller>,
) {
    let (mut properties, mut transform) = query.single_mut();

    let mut c_t = q_character.single().0.clone();

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

const GRAVITY_ACC: f32 = 4.8;
const GRAVITY_DIR: Vec3 = Vec3::NEG_Y;
// Vec3 {
//     x: 0.,
//     y: -1.,
//     z: 0.,
// };

fn gravity_movement_t(time: Res<Time>) -> Vec3 {
    let base_movement = GRAVITY_ACC * GRAVITY_DIR * time.delta_seconds();
    base_movement
}

fn move_in_direction_t(t: &Transform, game: ResMut<Game>) -> Vec3 {
    let f = t.right();
    let speed = game.player_wheel.speed_z;
    let movement = Vec3::ZERO + f * speed;
    // we presume the speed has the time.delta_second() applied already
    movement
}

pub fn move_character(
    time: Res<Time>,
    mut q: Query<(&Transform, &mut KinematicCharacterController), With<PlayerCharacter>>,
    game: ResMut<Game>,
) {
    let (t, mut c) = q.single_mut();

    let gravity_movement = gravity_movement_t(time);
    let movement = move_in_direction_t(t, game);

    // Using not standard transform, but KinematicCharacterController
    c.translation = Some(gravity_movement + movement)
}
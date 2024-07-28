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

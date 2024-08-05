use bevy::{
    math::{Quat, Vec3},
    prelude::{Mut, Query, Res, ResMut, With},
    time::Time,
    transform::components::Transform,
};

use bevy_rapier3d::prelude::KinematicCharacterController;

use crate::{
    movement::{
        constants::GRAVITY_DIR,
        movement::{move_dir_translate, move_gravity_translate},
    },
    resources::Game,
    utils::angles::quat_w_to_axis_adjust,
};

use super::{constants::GRAVITY_ACC, resources::PlayerCharacter};

pub fn turn_character(mut q: Query<&mut Transform, With<PlayerCharacter>>, game: ResMut<Game>) {
    let mut t = q.single_mut();

    // For reference, this works except results in negative Y values that cause issues
    t.rotate_local_y(game.player_wheel.speed_y);
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

fn gravity_movement_t_old(time: Res<Time>) -> Vec3 {
    let base_movement = GRAVITY_ACC * GRAVITY_DIR * time.delta_seconds();
    base_movement
}

fn gravity_movement_t(time: Res<Time>) -> Vec3 {
    move_gravity_translate(GRAVITY_ACC, time)
}

fn move_in_direction_t_old(t: &Transform, game: ResMut<Game>) -> Vec3 {
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
    let movement = move_dir_translate(t.right(), game.player_wheel.speed_z);

    // Using not standard transform, but KinematicCharacterController
    c.translation = Some(gravity_movement + movement)
}

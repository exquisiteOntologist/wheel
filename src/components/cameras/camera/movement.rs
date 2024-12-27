use crate::{
    components::characters::player::resources::PlayerCharacter,
    constants::{FORWARD_SPEED, MAX_SPEED},
    movement::{
        movement::{diff_translations, move_gravity_translate},
        orientation::look_at_on_y,
    },
    resources::{Game, PlayerCamera},
};
use bevy::{math::Dir3, prelude::*};
use bevy_rapier3d::prelude::KinematicCharacterController;

use super::constants::{GRAVITY_ACC, MAX_CAM_DISTANCE, STOPPING_THRESHOLD};

pub fn move_camera(
    game: Res<Game>,
    mut q_char: Query<(&PlayerCharacter, &mut Transform)>,
    mut q_cam: Query<
        (
            &PlayerCamera,
            &mut Transform,
            &mut KinematicCharacterController,
        ),
        Without<PlayerCharacter>,
    >,
) {
    if q_char.is_empty() {
        return;
    }
    let (_char, t_char) = q_char.single_mut();
    let (_cam, mut t_cam, mut control_cam) = q_cam.single_mut();

    let tran_original: Vec3 = t_cam.translation.clone();
    let mut t_cam_new: Transform = t_cam.clone();

    let dir_of_char = t_char.right();

    // We will move the camera towards behind the character (not to the character)
    let target_behind_char = get_tran_behind_char(&t_cam_new, &t_char, dir_of_char, &game);
    let distance = t_cam_new.translation.distance(t_char.translation);
    move_cam_to(&mut t_cam_new, &target_behind_char);
    set_cam_height(&mut t_cam_new, &target_behind_char, &distance);
    turn_look_in_front(&mut t_cam_new, &t_char, dir_of_char);

    // Translate by the difference of the old and new transforms.
    // We use the controller instead of the transform
    // to handle collisions.
    let t_diff = diff_translations(t_cam_new.translation, tran_original);
    control_cam.translation = Some(t_diff);
    // Apply rotation of new transform to active transform.
    t_cam.rotation = t_cam_new.rotation;
}

pub fn adjust_camera_speed(
    mut game: ResMut<Game>,
    q_char: Query<(&PlayerCharacter, &Transform)>,
    q_cam: Query<(&PlayerCamera, &Transform), Without<PlayerCharacter>>,
) {
    if q_char.is_empty() || q_cam.is_empty() {
        return;
    }
    let (_, t_char) = q_char.single();
    let (_, t_cam) = q_cam.single();
    let distance_x = t_char.translation.x - t_cam.translation.x;
    let distance_z = t_char.translation.z - t_cam.translation.z;
    let camera_should_move_x = distance_x > MAX_CAM_DISTANCE || distance_x < -MAX_CAM_DISTANCE;
    let camera_should_move_z = distance_z > MAX_CAM_DISTANCE || distance_z < -MAX_CAM_DISTANCE;
    let m_x = if distance_x > 0. { 1. } else { -1. };
    let m_z = if distance_z > 0. { 1. } else { -1. };

    // println!("Distance X {:?}", distance_x);
    // println!("Move camera? {:?}", camera_should_move_x);

    if camera_should_move_x {
        game.camera.speed_x += (FORWARD_SPEED * 2.) * m_x;
    }

    if camera_should_move_z {
        game.camera.speed_z += FORWARD_SPEED * m_z;
    };

    if game.camera.speed_x != 0. {
        let dir_m = m_x;
        game.camera.speed_x -= FORWARD_SPEED * (game.camera.speed_x / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_x > STOPPING_THRESHOLD || game.camera.speed_x < -STOPPING_THRESHOLD) {
        game.camera.speed_x = 0.;
    }

    if game.camera.speed_z != 0. {
        let dir_m = m_z;
        game.camera.speed_z += FORWARD_SPEED * (game.camera.speed_z / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_z > STOPPING_THRESHOLD || game.camera.speed_z < -STOPPING_THRESHOLD) {
        game.camera.speed_z = 0.;
    }
}

fn get_tran_behind_char(
    t_cam: &Transform,
    t_char: &Transform,
    char_direction: Dir3,
    game: &Game,
) -> Transform {
    let m_y = if game.player_wheel.speed_y >= 0. {
        1.
    } else {
        -1.
    };
    // When the character is turning we want to pull back to see more of the field
    let dist_behind_char =
        -game.player_wheel.speed_z - (game.player_wheel.speed_y * 500. * m_y).max(5.);
    let mut tran_behind_char = t_cam.clone();
    tran_behind_char.translation = t_char.translation + char_direction * dist_behind_char;
    tran_behind_char
}

fn _get_tran_behind_char_simple_dev(t_char: &Transform) -> Transform {
    let mut tran_behind_char = t_char.clone();
    tran_behind_char.translation.z = -0.;
    tran_behind_char.translation.x = -10.;
    tran_behind_char.translation.x -= 15.;
    tran_behind_char
}

fn move_cam_to(t_cam: &mut Transform, t_dest: &Transform) {
    t_cam.translation.x += (t_dest.translation.x - t_cam.translation.x) * 0.05;
    t_cam.translation.z += (t_dest.translation.z - t_cam.translation.z) * 0.05;
}

fn _move_cam_exactly_behind(t_cam: &mut Mut<Transform>, t_char: &Transform, char_direction: Dir3) {
    // make camera translation match character's, except further back
    t_cam.translation = t_char.translation + char_direction * -10.;
    // t_cam.translation = t_char.translation + char_direction * -d;
}

fn _turn_move(
    t_cam: &mut Mut<Transform>,
    char_direction: Dir3,
    game: &mut ResMut<Game>,
    time: &Res<Time>,
) {
    t_cam.translation += char_direction * (game.player_wheel.speed_z * 10.) * time.delta_secs();
}

fn _get_turn_multiplier(t_cam: &Transform, t_dest: &Transform) -> f32 {
    let rot_diff = t_cam.rotation.y - t_dest.rotation.y;

    println!("rot y diff {:?}", rot_diff);

    if rot_diff > 0.001 {
        -1.
    } else if rot_diff < -0.001 {
        1.
    } else {
        0.
    }
}

fn set_cam_height(t_cam: &mut Transform, t_dest: &Transform, distance: &f32) {
    let base_y = t_dest.translation.y + 3. + ((t_dest.translation.y - t_cam.translation.y) * 0.01);
    let distance_fraction = distance / MAX_CAM_DISTANCE;
    t_cam.translation.y = base_y + (1. * distance_fraction);
    // println!(
    //     "cam height {:1} {:2} {:3} {:4}",
    //     t_dest.translation.y, base_y, distance_fraction, t_cam.translation.y
    // );
}

fn _get_char_direction(rotation: Quat) -> Dir3 {
    match Dir3::new(rotation * -Vec3::X) {
        Ok(v) => v,
        Err(_) => Dir3::NEG_Z,
    }
}

/// Make camera look infront of the character.
/// This turns the camera on the Y axis, and does not move it.
/// The direction argument represents the direction the character is facing.
fn turn_look_in_front(t_cam: &mut Transform, t_char: &Transform, char_direction: Dir3) {
    // This POI transform will be where the camera will face
    let mut tran_infront_char = t_cam.clone().to_owned();
    let dist_infront_char = 5.;
    // Here we translate the POI transform in front of the character
    tran_infront_char.translation = t_char.translation + char_direction * dist_infront_char; /* * time.delta_secs(); */
    // Now we will turn the camera to face the POI
    look_at_on_y(t_cam, &tran_infront_char);
}

pub fn move_camera_gravity(
    time: Res<Time>,
    mut q_cam: Query<(&PlayerCamera, &mut KinematicCharacterController), Without<PlayerCharacter>>,
) {
    let (_, mut control_cam) = q_cam.single_mut();
    let gravity_movement = gravity_movement_t(time);
    control_cam.translation = Some(gravity_movement);
}

fn gravity_movement_t(time: Res<Time>) -> Vec3 {
    move_gravity_translate(GRAVITY_ACC, time)
}

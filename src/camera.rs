use crate::{
    constants::{FORWARD_SPEED, MAX_CAM_DISTANCE, MAX_SPEED},
    resources::{Game, PlayerCamera, PlayerCharacter},
    wheel::wheel_y_rotation,
};
use bevy::prelude::*;

pub fn move_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut q_char: Query<(&PlayerCharacter, &mut Transform)>,
    mut q_cam: Query<(&PlayerCamera, &mut Transform), Without<PlayerCharacter>>,
) {
    let (_char, t_char) = q_char.single_mut();
    let (_cam, mut t_cam) = q_cam.single_mut();

    adjust_camera_speed(&t_cam, &t_char, &mut game);

    let distance = t_cam.translation.distance(t_char.translation);
    // let d = distance.max(game.camera.speed_z);

    let rotation = wheel_y_rotation(&t_char.rotation);
    let char_direction = get_char_direction(rotation);

    let tran_behind_char = get_tran_behind_char(&t_cam, &t_char, char_direction);

    // the further away the faster we want to move the camera
    let s_scale = distance / MAX_CAM_DISTANCE;
    // let s_speed_multi = game.player_wheel.speed_z * 100. * s_scale;
    let s_speed_multi = game.player_wheel.speed_z * 10.;

    move_cam_to(&mut t_cam, &tran_behind_char);
    set_cam_height(&mut t_cam);

    println!("cam speed {:?}", game.camera.speed_z);
    println!("cam distance {:?}", distance);
    println!(
        "rot y cam [{:1}] char [{:2}]",
        t_cam.rotation.y, t_char.rotation.y
    );

    look_in_front(&mut t_cam, &t_char, char_direction);

    println!(
        "cam rot (after) X [{:1}] Y [{:2}] Z [{:3}]",
        t_cam.rotation.x, t_cam.rotation.y, t_cam.rotation.z
    );
}

fn adjust_camera_speed(t_cam: &Transform, t_char: &Transform, game: &mut ResMut<Game>) {
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
        let dir_m = (m_x);
        game.camera.speed_x -= FORWARD_SPEED * (game.camera.speed_x / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_x > 0.0001 || game.camera.speed_x < -0.001) {
        game.camera.speed_x = 0.;
    }

    if game.camera.speed_z != 0. {
        let dir_m = (m_z);
        game.camera.speed_z += FORWARD_SPEED * (game.camera.speed_z / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_z > 0.001 || game.camera.speed_z < -0.001) {
        game.camera.speed_z = 0.;
    }
}

fn get_tran_behind_char(
    t_cam: &Transform,
    t_char: &Transform,
    char_direction: Direction3d,
) -> Transform {
    let dist_behind_char = -15.;
    let mut tran_behind_char = t_cam.clone();
    tran_behind_char.translation = t_char.translation + char_direction * dist_behind_char;
    tran_behind_char
}

fn move_cam_to(t_cam: &mut Mut<Transform>, t_dest: &Transform) {
    t_cam.translation.x += (t_dest.translation.x - t_cam.translation.x) * 0.01;
    t_cam.translation.z += (t_dest.translation.z - t_cam.translation.z) * 0.01;
}

fn _move_cam_exactly_behind(
    t_cam: &mut Mut<Transform>,
    t_char: &Transform,
    char_direction: Direction3d,
) {
    // make camera translation match character's, except further back
    t_cam.translation = t_char.translation + char_direction * -10.;
    // t_cam.translation = t_char.translation + char_direction * -d;
}

fn _turn_move(
    t_cam: &mut Mut<Transform>,
    char_direction: Direction3d,
    game: &mut ResMut<Game>,
    time: &Res<Time>,
) {
    t_cam.translation += char_direction * (game.player_wheel.speed_z * 10.) * time.delta_seconds();
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

fn set_cam_height(t_cam: &mut Mut<Transform>) {
    t_cam.translation.y = 3.;
}

fn get_char_direction(rotation: Quat) -> Direction3d {
    Direction3d::new(rotation * -Vec3::X).unwrap()
}

/// Make camera look infront of the character.
/// The direction argument represents the direction the character is facing.
fn look_in_front(t_cam: &mut Mut<Transform>, t_char: &Mut<Transform>, char_direction: Direction3d) {
    let mut tran_infront_char = t_cam.clone().to_owned();
    let dist_infront_char = 5.;
    tran_infront_char.translation = t_char.translation + char_direction * dist_infront_char; /* * time.delta_seconds(); */
    t_cam.look_at(tran_infront_char.translation.xyz(), Vec3::Y);
}

fn _look_at_char(t_cam: &mut Mut<Transform>, t_char: &Mut<Transform>) {
    t_cam.look_at(t_char.translation.xyz(), Vec3::Y);
}

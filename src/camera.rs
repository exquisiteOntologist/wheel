use crate::{
    constants::{FORWARD_SPEED, MAX_CAM_DISTANCE, MAX_SPEED, MAX_TURN_SPEED, TURN_SPEED},
    resources::{Game, PlayerCamera, PlayerCharacter},
    setup,
    wheel::wheel_y_rotation,
};
use bevy::prelude::*;

pub fn move_camera(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut q_char: Query<(&PlayerCharacter, &mut Transform)>,
    mut q_cam: Query<(&PlayerCamera, &mut Transform), Without<PlayerCharacter>>,
) {
    let (char, mut t_char) = q_char.single_mut();
    let (cam, mut t_cam) = q_cam.single_mut();

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

    // BEGIN char & cam translation lock
    // If not wanted it is still useful for debugging
    // let mut trans_char = t_char.translation + (t_char.right() * -5. * time.delta_seconds());
    // trans_char.z -= 10.;

    // let mut trans_form = t_char.with_translation(trans_char);
    // trans_form.translation.y = 3.;
    // trans_form.translation.z -= 10.;

    // // t_cam.translation = trans_char;
    // // t_cam.translation.y = 3.;
    //
    //
    // if camera_should_move_x {
    // t_cam.translation.x = t_char.translation.x;
    // t_cam.translation.x = trans_form.translation.x + 10.;
    // }
    // if camera_should_move_z {
    // t_cam.translation.z = t_char.translation.z + 10.;
    // t_cam.translation.z = trans_form.translation.z + 10.;
    // }
    //
    //
    // let otter = t_cam.translation.angle_between(t_char.translation);
    // let rotation = wheel_y_rotation(&t_char.rotation);
    // // t_cam.rotate_around(t_char.translation, rotation);
    // t_cam.rotation.y -= otter;
    // t_cam.translate_around(t_char.translation, rotation);
    // t_cam.rotate_around(t_char.translation, rotation);
    // t_cam.rotation.y = rotation.y;
    // t_cam.translation.x -= 10.;
    // END

    // t_cam.translation.x += game.camera.speed_x;
    // t_cam.translation.z += game.camera.speed_z;

    if game.camera.speed_x != 0. {
        let dir_m = (m_x);
        game.camera.speed_x += FORWARD_SPEED * (game.camera.speed_x / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_x > 0.0001 || game.camera.speed_x < -0.0001) {
        game.camera.speed_x = 0.;
    }

    if game.camera.speed_z != 0. {
        let dir_m = (m_z);
        game.camera.speed_z += FORWARD_SPEED * (game.camera.speed_z / MAX_SPEED) * 0.5 * dir_m;
    }

    if !(game.camera.speed_z > 0.0001 || game.camera.speed_z < -0.0001) {
        game.camera.speed_z = 0.;
    }

    // BEGIN movement
    let rotation = wheel_y_rotation(&t_char.rotation);
    let direction = Direction3d::new(rotation * -Vec3::X).unwrap();
    // make camera translation match character's, except further back
    t_cam.translation = t_char.translation + direction * -10.;
    // elevate camera
    t_cam.translation.y = 3.;
    // END movement

    let t_cam_face_char = t_cam.looking_at(
        Vec3::new(
            t_char.translation.x,
            t_char.translation.y + 1.,
            t_char.translation.z,
        ),
        // Vec3::new(t_char.translation.x, 1.0, t_char.translation.z),
        Vec3::Y,
    );
    // let dir = Direction3d::new(t_cam_face_char.rotation * -Vec3::X).unwrap();
    // t_cam.rotation.y = t_cam_face_char.rotation.y;
    // t_cam.rotation.x = 0.;
    // t_cam.rotation.z = 0.;
    println!("rot y [{:1}] [{:2}]", t_cam.rotation.y, t_char.rotation.y);
    // let cam_rot = t_cam.rotation.normalize();
    let rot_diff = t_cam.rotation.y - t_cam_face_char.rotation.y;
    // let rot_diff = t_cam.rotation.y - t_char.rotation.y;
    println!("rot y diff {:?}", rot_diff);

    let cam_spin_m = if rot_diff > 0.001 {
        -1.
    } else if rot_diff < -0.001 {
        1.
    } else {
        0.
    };

    // let cam_spin_m = if t_cam.rotation.y >=  {
    //     -1.
    // } else if rot_diff < -0.0001 {
    //     1.
    // } else {
    //     0.
    // };

    // println!("rot diff {:?}", rot_diff);

    if cam_spin_m != 0. {
        // t_cam.rotate_y(0.001 * cam_spin_m);
        // t_cam.rotation.y = 0.;
        // t_cam.rotation.x = 0.;
        // t_cam.rotation.z = 0.;
        // t_cam.rotation.y = rotation.y;
        // t_cam.rotation.y = t_cam_face_char.rotation.y;
        // t_cam.rotation.
        // t_cam.rotate_local_y(0.01 * cam_spin_m); // seems to stop following wheel and the wheel "wraps" around
        // the wrapping could be because it is heading in the wrong direction
        // t_cam.look_at(t_char.translation.xyz(), Vec3::Y);
        // let angle = t_cam.rotation.angle_between(t_cam_face_char.rotation);
        // t_cam.rotate_local_y(angle * 0.5);
        // t_cam.rotate_local_y(-rot_diff * 1.5);
        // t_cam.rotation.x = 0.;
        // t_cam.rotation.z = 0.;
        // t_cam.rotate_local_y(0.05);

        // t_cam.translation += dir * FORWARD_SPEED * time.delta_seconds();
        // t_cam.translation += dir * game.camera.speed_z * time.delta_seconds();
    }
    t_cam.look_at(t_char.translation.xyz(), Vec3::Y);

    // here we correct the z rotation based on the y rotation
    // https://stackoverflow.com/a/4021898
    let m_z = if t_cam.rotation.y >= 0. { 1. } else { -1. };
    // t_cam.rotation.z *= m_z;

    println!(
        "cam rot x [{:1}] y [{:2}] z [{:3}]",
        t_cam.rotation.x, t_cam.rotation.y, t_cam.rotation.z
    );

    // Quat::
    // t_cam.rotation.
    // t_cam.rotation.z = 0.;
    // let player_translation = t_char.translation.xz();
    // let to_player = (player_translation - t_cam.translation.xz()).normalize();

    // // get the quaternion to rotate from the initial enemy facing direction to the direction
    // // facing the player
    // let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

    // // rotate the enemy to face the player
    // t_cam.rotation = rotate_to_player;

    // t_cam.rotate_y(t_cam_face_char.rotation.y);
    // t_cam.len
    // println!("I'm a character Y: {:?}", t_char.1.local_y());
    // println!("I'm a camera Y: {:?}", t_cam.1.local_y());
    //

    // t_cam.rotation.y = rotation.y;
    // t_cam.rotate_y(0.01 * cam_spin_m);
    // let trans = t_char.translation;
    // t_cam.rotate_around(trans + (direction * -1.) * 1., rotation);
    // t_cam.rotation = rotation;
    // t_cam.rotation.y = t_char.rotation.y;
    // t_cam.rotation.x = 0.;
    // t_cam.rotation.z = 0.;
    // t_cam.rotation.y = -t_char.rotation.y;
    // t_cam.
    // t_cam.translation.
    //
}

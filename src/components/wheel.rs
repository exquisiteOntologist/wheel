use std::f32::{consts::TAU, RADIX};

use bevy::{
    ecs::{
        query::With,
        system::{Res, ResMut},
    },
    math::{Dir3, Vec3},
    prelude::*,
    time::Time,
    transform::components::Transform,
};

use crate::{
    constants::{FORWARD_SPEED, MAX_SPEED, MAX_TURN_SPEED, TURN_SPEED},
    resources::{Game, PlayerCharacter},
    utils::matrix::{quaternion_from_rpy_quat, roll_pitch_yaw, roll_pitch_yaw_from_quat, RPY},
};

fn tilt_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    game: ResMut<Game>,
    mut wheel: ResMut<WheelState>,
) {
    for mut t in &mut q {
        println!("wheel pos {}", t.translation.xyx());
        println!("speed y {}", game.player_wheel.speed_y);
        // // if t.rotation.normalize().x == 0. {
        // //     t.rotate_local_x(0.1);
        // // }
        // // let new_x: f32 = match t.rotation.normalize().x {
        // //     0. => 0.1,
        // //     0.1 => {
        // //         if game.player_wheel.speed_y > 0.0 {
        // //             0.
        // //         } else {
        // //             0.1
        // //         }
        // //     }
        // //     -0.1 => {
        // //         if game.player_wheel.speed_y > 0.0 {
        // //             0.1
        // //         } else {
        // //             0.
        // //         }
        // //     }
        // // };
        let turn_factor = if game.player_wheel.speed_y > 0.01 {
            -1.
        } else if game.player_wheel.speed_y < -0.01 {
            1.
        } else {
            0.
        };
        // // let normal_x = t.rotation.x;
        // // if normal_x == 0. {
        // //     t.rotate_local_x(0.1 * x_speed);
        // // } else if normal_x > 0.1 {
        // //     t.rotate_local_x(0.1 * -1.);
        // // } else if normal_x < 0.1 {
        // //     t.rotate_local_x(0.1 * 1.);
        // // }
        // let x_normal = t.rotation.normalize().x;
        // // println!("x normal {}", x_normal);
        // if x_normal == 0. {
        //     // t.rotate_local_x(0.1 * x_speed);
        // } else {
        //     let mut zz = t.rotation;
        //     zz.x = 0.;
        //     // t.rotate_local(zz);
        //     let diff = 0. - x_normal;
        //     // t.rotate_local_x(diff);
        // }
        // // let new_x = if normal_x == 0.1 {
        // //     -0.1
        // // } else if normal_x == -0.1 {
        // //     0.1
        // // } else {
        // //     x_speed * 0.1
        // // };
        // // t.rotate_local_x(new_x);
        // //
        // //

        // let mut current_x_rotation = wheel_x_rotation(&t.rotation);
        // println!(
        //     "Rotation Y {}",
        //     current_x_rotation.to_euler(EulerRot::default()).1
        // );
        // println!(
        //     "Rotation X {}",
        //     current_x_rotation.to_euler(EulerRot::default()).0
        // );
        // // the x rotation is the same as the Euler Y rotation
        // println!("Rotation X alt {}", current_x_rotation.x);
        // // println!("Rotation Y alt {}", current_x_rotation.y);
        // // println!("Rotation Z alt {}", current_x_rotation.z);

        // // let mut n_transform = Transform::from_xyz(0., 0., 0.);
        // // n_transform.rotate_x(0.001);

        // let mut t_clone = Transform::from_matrix(t.compute_matrix());
        // t_clone.rotate_axis(Dir3::X, 0.1);
        // println!("Rotation X alt after {}", t_clone.rotation.x);

        // t_clone.rotation.y = t.rotation.y;
        // t_clone.rotation.z = t.rotation.z;

        // t.rotate(t_clone.rotation);
        // t.rotate_axis(Dir3::X, 0.1);
        // let xf_rot2d = Transform::from_rotation(Quat::from_rotation_z((30.0_f32).to_radians()));

        // let rot_tran = Transform::from_rotation(Quat::from_euler(
        //     // YXZ order corresponds to the common
        //     // "yaw"/"pitch"/"roll" convention
        //     EulerRot::YXZ,
        //     (0. as f32).to_radians(),
        //     (20. * turn_factor as f32).to_radians(),
        //     (0. as f32).to_radians(),
        // ));
        //
        // let mut rot_tran = Transform::from_xyz(t.translation.x, t.translation.y, t.translation.z);
        // rot_tran.rotate_local_y(30. * turn_factor * TAU);

        // t.rotation.x = xf_
        //
        let turn = 30. * turn_factor * TAU;
        // println!("Y {} ", t.rotation.to_scaled_axis().y / RADIX as f32);
        println!("Y {} ", t.rotation.to_scaled_axis().xyx().y);
        // t.rotate_x(turn);
        // t.apply(rot_tran.as_reflect());

        // t.apply(xf_rot2d.as_reflect());
        // t.rotate(xf_rot2d.rotation);
        // t.rotate(xf_rot2d.as_reflect());
        // t.forward()
        // let l_dir = t.left().slerp(xf_rot2d.left(), 0.5);
        // t.rotate_axis(Dir3::X, 0.1);
        let (axis, angle) = t.rotation.to_axis_angle();
        let diff = 3. - axis.x;
        // t.rotate_axis(Dir3::from_xyz(axis.x, axis.y, axis.z).unwrap(), angle);
        // t.rotate_x(diff * TAU);
        // t.rotation.x += 2.;
        let max_tilt = 45.;
        let max_tilt_r = TAU / max_tilt;
        // let new_tilt = (0.03 * turn_factor) * TAU;
        let new_tilt = (0.1 * turn_factor);
        // if game.player_wheel.speed_y > 0. {
        // if turn_factor == 1. {
        //     if axis.x < max_tilt_r {
        //         // t.rotate_local_x(new_tilt);
        //         // t.rotate_x(axis.x + new_tilt);
        //         t.rotate_x(0.01);
        //     }
        // } else if turn_factor == -1. {
        //     if axis.x > -max_tilt_r {
        //         // t.rotate_local_x(new_tilt);
        //         // t.rotate_x(axis.x + new_tilt);
        //         t.rotate_x(-0.01 * 2.);
        //     }
        // } else {
        //     t.rotate_x(axis.x * 0.01);
        // }
        println!("axis {}", axis);
        // if axis.x > 1. + max_tilt_r {
        //     t.rotate_x(axis.x * -0.01);
        // } else if axis.x < 1. - max_tilt_r {
        //     t.rotate_x(axis.x * -0.01);
        // } else {
        //     println!("tilt {}", new_tilt);
        //     t.rotate_x(new_tilt);
        println!("tilt {}", new_tilt);
        // }
        // let even = axis.x * 1. * TAU;
        let even = 1.;
        println!("even {}", even);
        // println!("even alt {}", even * TAU);
        if (new_tilt != 0.) {
            if t.rotation.x > -max_tilt_r && t.rotation.x < max_tilt_r {
                // t.rotate_x(new_tilt);
            }
        } else {
            if t.rotation.x > 0.01 || t.rotation.x < -0.01 {
                let x = t.rotation.x;
                // t.rotate_x(x * 0.1);
            }
        }
        // if (new_tilt > 0. && (even < max_tilt_r)) || (new_tilt < 0. && (even > -max_tilt_r)) {
        //     println!("tilt inside {}", new_tilt);
        //     t.rotate_x(new_tilt);
        //     //
        // }
        // let turn = (TAU / 30.) * turn_factor;
        // t.rotate_local_x((0.3 * turn_factor) * TAU);
        //
        let (x, y, z) = t.rotation.to_euler(EulerRot::XYZ);
        //
        // t.rotate_axis(t.forward().slerp(xf_rot2d.forward(), 0.5), 0.5);
        // t.rotate_local_x(0.2 * TAU);
        // t.rotate_local(rotation);
        // t.align(Vec3::Z, Vec3::Z, Vec3::X, Vec3::X);
        println!("rot y {}", t.rotation.to_axis_angle().0.y);
        println!("rot x {}", t.rotation.to_axis_angle().0.x);
        println!("rot x ang {}", t.rotation.x);
        println!("rot x ang NORM {}", t.rotation.normalize().x);
        println!("rot x ang TAU {}", t.rotation.normalize().x * TAU);
        println!("rot x tau {}", t.rotation.to_axis_angle().0.x * TAU);
        println!("EULER {:1} {:2} {:3}", x, y, z);
        //
        let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());
        println!("roll {}", roll);
        println!("pitch {}", pitch);
        println!("yaw {}", yaw);
        // t.rotation.

        let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch, yaw);
        t.rotate(updated_rot_quat);
        // let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());
        // let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch + 0.3, yaw + 0.3);
        // t.rotate(updated_rot_quat);
        //
        wheel.rpy.roll -= game.player_wheel.speed_z;
        // turn
        wheel.rpy.pitch += game.player_wheel.speed_y;
        wheel.rpy.yaw -= (game.player_wheel.speed_y / game.player_wheel.speed_z).clamp(-0.1, 0.1);
        //
        let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());
        // ROLLING
        let updated_rot_quat = quaternion_from_rpy_quat(wheel.rpy.roll, 0., 0.);
        t.rotate(updated_rot_quat);
        // TURNING DIRECTION
        let updated_rot_quat = quaternion_from_rpy_quat(0., wheel.rpy.pitch, 0.);
        t.rotate(updated_rot_quat);

        let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch, yaw);
        t.rotate(updated_rot_quat);
        // TILT (NOTE CHANGING YAW TILT RESULTS IN PITCH NAN VALUES THAT BREAK)
        if yaw > -0.3 || yaw < 0.3 {
            // let updated_rot_quat = quaternion_from_rpy_quat(0., 0., wheel.rpy.yaw);
            let updated_rot_quat = quaternion_from_rpy_quat(0., 0., new_tilt);
            t.rotate(updated_rot_quat);
        }

        let zz = updated_rot_quat;
        println!("quat {:1} {:2} {:3} {:4}", zz.x, zz.y, zz.z, zz.w);
        // t.apply(updated_rot_quat.as_reflect())
        //
        //
        // NOTE SOMETIMES IT MAY BE WORKING BUT BECAUSE
        // CAMERA IS USING THE WHEEL ROTATION
        // IT GETS WEIRD WHEN CHANGING THE X VALUE
    }
}

pub fn spin_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    game: ResMut<Game>,
) {
    for mut t in &mut q {
        // spinning the wheel
        return;
        t.rotate_local_z(game.player_wheel.speed_z * 0.5);
    }
}

pub fn turn_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    game: ResMut<Game>,
) {
    return;

    for mut t in &mut q {
        // turning
        if game.player_wheel.speed_y != 0.0 {
            if game.player_wheel.speed_y < 0.0 {
                // t.rotate_local_x(-0.1);
                // t.rotate_x(-0.1);
                // rotation.x = -0.3;
                //
                // TILT
                // t.rotate_local_z(0.1);
                // let mut rotation = wheel_x_rotation(&t.rotation);
                // rotation.x = 0.1;
                // if t.rotation.x < 0.1 {
                //     t.rotate_x(rotation.x);
                // }
                // t.rotate_local_axis(Dir3::X, 0.2)
            } else if game.player_wheel.speed_y > 0.0 {
                // t.rotate_local_x(0.1);
                // t.rotate_x(0.1);
                // rotation.x = 0.3;
                //
                // TILT
                // t.rotate_local_z(-0.1);
                // let mut rotation = wheel_x_rotation(&t.rotation);
                // rotation.x = -0.1;
                // if t.rotation.x > -0.1 {
                //     t.rotate_x(rotation.x);
                // }
            }
            t.rotate_y(game.player_wheel.speed_y);
        }

        // t.rotate_local(Quat {
        //     x: 0.0,
        //     y: game.player_wheel.speed_y,
        //     z: game.player_wheel.speed_z * 0.5,
        //     w: 1.0,
        // })
    }
}

/// because the wheel spins and turns, get just y for the turn
pub fn wheel_y_rotation(rotation: &Quat) -> Quat {
    let mut rotation_y = rotation.normalize();
    rotation_y.z = 0.;
    rotation_y.x = 0.;
    rotation_y
}

/// because the wheel spins and turns, get just x for the turn
pub fn wheel_x_rotation(rotation: &Quat) -> Quat {
    let mut rotation_x = rotation.normalize();
    rotation_x.z = 0.;
    rotation_x.y = 0.;
    // rotation_x.x = 0.;
    rotation_x
}

pub fn move_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    let mut t = q.single_mut();
    let speed = game.player_wheel.speed_z;

    // since we are also spinning the wheel,
    // for the math to work we only want Y,
    // as the wheel pivots around Y
    let rotation = wheel_y_rotation(&t.rotation);
    if let Ok(direction) = Dir3::new(rotation * -Vec3::X) {
        t.translation += direction * speed;
    }

    // Slow down speed
    if game.player_wheel.speed_z > 0.0 {
        game.player_wheel.speed_z -= FORWARD_SPEED * (game.player_wheel.speed_z / MAX_SPEED) * 0.5;
    } else if game.player_wheel.speed_z < 0.0 {
        game.player_wheel.speed_z += FORWARD_SPEED * (game.player_wheel.speed_z / -MAX_SPEED) * 0.5;
    }

    if !(game.player_wheel.speed_z > 0.0001 || game.player_wheel.speed_z < -0.0001) {
        game.player_wheel.speed_z = 0.;
    }

    // Slow down turn
    if game.player_wheel.speed_y > 0.0 {
        game.player_wheel.speed_y -= TURN_SPEED * (game.player_wheel.speed_y / MAX_TURN_SPEED);
    } else if game.player_wheel.speed_y < 0.0 {
        game.player_wheel.speed_y += TURN_SPEED * (game.player_wheel.speed_y / -MAX_TURN_SPEED);
    }

    if !(game.player_wheel.speed_y > 0.0001 || game.player_wheel.speed_y < -0.0001) {
        game.player_wheel.speed_y = 0.;
    }
}

fn setup_wheel(mut game: ResMut<Game>) {
    game.player_wheel.speed_z = FORWARD_SPEED * 10.;
}

#[derive(Resource, Default)]
pub struct WheelState {
    /// Roll is tilting sideways,
    /// Pitch is rolling the wheel,
    /// Yaw is turning to another direction
    pub rpy: RPY,
}

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WheelState>();
        app.add_systems(Startup, setup_wheel);
        app.add_systems(Update, (spin_wheel, turn_wheel, tilt_wheel, move_wheel));
    }
}

use std::f32::consts::TAU;

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
    resources::{Game, PlayerWheel},
    utils::matrix::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat, RPY},
};

fn tilt_wheel(
    mut q: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    game: ResMut<Game>,
    mut wheel: ResMut<WheelState>,
) {
    for mut t in &mut q {
        println!("wheel pos {}", t.translation.xyx());
        println!("speed y {}", game.player_wheel.speed_y);

        let turn_factor = if game.player_wheel.speed_y > 0.01 {
            -1.
        } else if game.player_wheel.speed_y < -0.01 {
            1.
        } else {
            0.
        };

        let turn = 30. * turn_factor * TAU;
        // println!("Y {} ", t.rotation.to_scaled_axis().y / RADIX as f32);
        println!("Y {} ", t.rotation.to_scaled_axis().xyx().y);

        let (axis, angle) = t.rotation.to_axis_angle();
        let diff = 3. - axis.x;

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

        let (x, y, z) = t.rotation.to_euler(EulerRot::XYZ);
        println!("rot y {}", t.rotation.to_axis_angle().0.y);
        println!("rot x {}", t.rotation.to_axis_angle().0.x);
        println!("rot x ang {}", t.rotation.x);
        println!("EULER {:1} {:2} {:3}", x, y, z);

        let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());
        println!("roll {}", roll);
        println!("pitch {}", pitch);
        println!("yaw {}", yaw);

        let mut r_t = t.clone();

        let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch, yaw);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);
        // let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(r_t.rotation.conjugate());
        // let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch + 0.3, yaw + 0.3);
        // r_t.rotate(updated_rot_quat);

        // rolling
        wheel.rpy.roll -= game.player_wheel.speed_z * 0.5;
        // turn
        wheel.rpy.pitch += game.player_wheel.speed_y;
        // tilt
        let base_tilt_speed = 0.01;
        let max_speed = 0.1; // not MAX_SPEED
        let tilt_modifier = game.player_wheel.speed_z / max_speed;
        let tilt_speed = base_tilt_speed * tilt_modifier;
        wheel.rpy.yaw = if turn_factor != 0. {
            (wheel.rpy.yaw + (tilt_speed * turn_factor)).clamp(-0.1, 0.1)
        } else {
            if wheel.rpy.yaw > -0.001 && wheel.rpy.yaw < 0.001 {
                0.
            } else {
                wheel.rpy.yaw * 0.9
            }
        };

        let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(r_t.rotation.conjugate());
        // ROLLING
        let updated_rot_quat = quaternion_from_rpy_quat(wheel.rpy.roll, 0., 0.);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);
        // TURNING DIRECTION
        // Note this is now done on the parent
        // let updated_rot_quat = quaternion_from_rpy_quat(0., wheel.rpy.pitch, 0.);
        let updated_rot_quat = quaternion_from_rpy_quat(0., 0., 0.);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);

        let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch, yaw);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);
        // TILT (NOTE CHANGING YAW TILT RESULTS IN PITCH NAN VALUES THAT BREAK)
        let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(r_t.rotation.conjugate());
        println!("TILT BEFORE {}", r_t.rotation);
        println!(
            "TILT BEFORE roll pitch yaw {:1} {:2} {:3}",
            roll, pitch, yaw
        );
        // let updated_rot_quat = quaternion_from_rpy_quat(0., 0., new_tilt);
        let updated_rot_quat = quaternion_from_rpy_quat(0., 0., wheel.rpy.yaw);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);
        let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(r_t.rotation.conjugate());
        println!("TILT AFTER {}", r_t.rotation);
        println!("TILT AFTER roll pitch yaw {:1} {:2} {:3}", roll, pitch, yaw);

        // t.translation = r_t.translation.normalize();
        t.rotation = r_t.rotation.normalize();

        let zz = updated_rot_quat;
        println!("quat {:1} {:2} {:3} {:4}", zz.x, zz.y, zz.z, zz.w);
        // t.apply(updated_rot_quat.as_reflect())

        // NOTE SOMETIMES IT MAY BE WORKING BUT BECAUSE
        // CAMERA IS USING THE WHEEL ROTATION
        // IT GETS WEIRD WHEN CHANGING THE X VALUE
    }
}

pub fn spin_wheel(
    mut q: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    game: ResMut<Game>,
) {
    for mut t in &mut q {
        // spinning the wheel
        // return;
        t.rotate_local_z(game.player_wheel.speed_z * 0.5);
    }
}

pub fn turn_wheel(
    mut q: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    game: ResMut<Game>,
) {
    // return;

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
    mut q: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    let mut t = q.single_mut();
    let speed = game.player_wheel.speed_z;

    // since we are also spinning the wheel,
    // for the math to work we only want Y,
    // as the wheel pivots around Y
    let rotation = wheel_y_rotation(&t.rotation).normalize();
    // if let Ok(direction) = Dir3::new(rotation * -Vec3::X) {
    if let Ok(direction) = Dir3::new(rotation * -Vec3::Z) {
        // t.translation += direction * speed;
        // let f = t.left();
        // t.translation += f * speed;
        // t.translation.y = 2.1;
        // t.translation.z += 0.01;
        //
        // ^ This wheel should not move,
        // it instead should be parented,
        // with the parent being them mover
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

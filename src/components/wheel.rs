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
    utils::roll_pitch_yaw::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat, RPY},
};

fn wheel_rotation(
    mut q: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    game: ResMut<Game>,
    mut wheel: ResMut<WheelState>,
) {
    for mut t in &mut q {
        // println!("wheel pos {}", t.translation.xyx());

        let turn_factor = if game.player_wheel.speed_y > 0.01 {
            -1.
        } else if game.player_wheel.speed_y < -0.01 {
            1.
        } else {
            0.
        };

        let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());
        let mut r_t = t.clone();

        let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch, yaw);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);

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
        let updated_rot_quat = quaternion_from_rpy_quat(0., 0., 0.);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);

        let updated_rot_quat = quaternion_from_rpy_quat(roll, pitch, yaw);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);

        let updated_rot_quat = quaternion_from_rpy_quat(0., 0., wheel.rpy.yaw);
        r_t.rotation = r_t.rotation.normalize();
        r_t.rotate(updated_rot_quat);

        t.rotation = r_t.rotation.normalize();
    }
}

pub fn _spin_wheel(
    mut q: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    game: ResMut<Game>,
) {
    for mut t in &mut q {
        // spinning the wheel
        // t.rotate_local_z(game.player_wheel.speed_z * 0.5);
    }
}

pub fn _turn_wheel(
    mut q: Query<&mut Transform, With<PlayerWheel>>,
    time: Res<Time>,
    game: ResMut<Game>,
) {
    for mut t in &mut q {
        // turning
        // t.rotate_y(game.player_wheel.speed_y);
    }
}

/// because the wheel spins and turns, get the rotation,
/// with only the Y value (useful for turning)
pub fn wheel_y_rotation(rotation: &Quat) -> Quat {
    let mut rotation_y = rotation.normalize();
    rotation_y.z = 0.;
    rotation_y.x = 0.;
    rotation_y
}

/// because the wheel spins and turns, get the rotation,
/// with only the X value (useful for turning)
pub fn wheel_x_rotation(rotation: &Quat) -> Quat {
    let mut rotation_x = rotation.normalize();
    rotation_x.z = 0.;
    rotation_x.y = 0.;
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

fn setup_wheel(mut commands: Commands, mut game: ResMut<Game>) {
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
        app.add_systems(Update, (wheel_rotation, move_wheel));
    }
}

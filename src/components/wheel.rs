use bevy::{
    ecs::{
        query::With,
        system::{Res, ResMut},
    },
    math::{Direction3d, Vec3},
    prelude::*,
    time::Time,
    transform::components::Transform,
};

use crate::{
    constants::{FORWARD_SPEED, MAX_SPEED, MAX_TURN_SPEED, TURN_SPEED},
    resources::{Game, PlayerCharacter},
};

pub fn spin_wheel(mut q: Query<&mut Transform, With<PlayerCharacter>>, game: ResMut<Game>) {
    for mut t in &mut q {
        // spinning the wheel
        t.rotate_local_z(game.player_wheel.speed_z);

        // turning
        if game.player_wheel.speed_y != 0.0 {
            if game.player_wheel.speed_y < 0.0 {
                // t.rotate_local_x(-0.1);
                // t.rotate_x(-0.1);
                // rotation.x = -0.3;
                //
                // TILT
            } else if game.player_wheel.speed_y > 0.0 {
                // t.rotate_local_x(0.1);
                // t.rotate_x(0.1);
                // rotation.x = 0.3;
                //
                // TILT
            }
            t.rotate_y(game.player_wheel.speed_y);
        }
    }
}

/// because the wheel spins and turns, get just y for the turn
pub fn wheel_y_rotation(rotation: &Quat) -> Quat {
    let mut rotation_y = rotation.normalize();
    rotation_y.z = 0.;
    rotation_y.x = 0.;
    rotation_y
}

pub fn move_wheel(
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    let mut t = q.single_mut();
    let speed = game.player_wheel.speed_z;

    // since we are also spinning the wheel, for the math to work we only want Y, as the wheel pivots around Y
    let rotation = wheel_y_rotation(&t.rotation);
    let direction = Direction3d::new(rotation * -Vec3::X).unwrap();
    t.translation += direction * (speed * 100.) * time.delta_seconds();

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

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spin_wheel, move_wheel));
    }
}

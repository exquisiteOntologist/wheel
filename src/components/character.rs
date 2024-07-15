use bevy::{
    app::{App, Plugin, Update},
    math::{Dir3, Vec3},
    prelude::{Query, Res, ResMut, With},
    time::Time,
    transform::components::Transform,
};

use crate::{
    resources::{Game, PlayerCharacter},
    utils::matrix::{quaternion_from_rpy_quat, roll_pitch_yaw_from_quat},
};

use super::wheel::{wheel_y_rotation, WheelState};

pub fn move_character(
    // this may have to be global transform
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
    // to find the direction
    mut wheel: ResMut<WheelState>,
) {
    let mut t = q.single_mut();

    // TURNING DIRECTION
    // let (roll, pitch, yaw) = roll_pitch_yaw_from_quat(t.rotation.conjugate());
    // let updated_rot_quat = quaternion_from_rpy_quat(0., pitch, 0.);
    // t.rotation = t.rotation.normalize();
    // t.rotate(updated_rot_quat);
    // let updated_rot_quat = quaternion_from_rpy_quat(0., -wheel.rpy.pitch, 0.);
    // t.rotation = t.rotation.normalize();
    // t.rotate(updated_rot_quat);

    // t.rotate_local_y(wheel.rpy.pitch * 0.01);

    let turn_speed = 0.01;
    let turn_factor = if game.player_wheel.speed_y == 0. {
        0.
    } else if game.player_wheel.speed_y > 0. {
        1.
    } else {
        -1.
    };

    // because this rotation is relative and not absolute
    t.rotate_local_y(turn_speed * turn_factor);

    println!("parent rotation {}", t.rotation);

    let speed = game.player_wheel.speed_z;
    let rotation = wheel_y_rotation(&t.rotation).normalize();
    // if let Ok(direction) = Dir3::new(rotation * -Vec3::X) {
    if let Ok(direction) = Dir3::new(rotation * -Vec3::Z) {
        // t.translation += direction * speed;
        let f = t.right();
        t.translation += f * speed;
        // t.translation.y = 2.1;
        // t.translation.z += 0.01;
        //
        // ^ This wheel should not move,
        // it instead should be parented,
        // with the parent being them mover
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_character));
    }
}

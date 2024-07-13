use crate::{
    constants::{FORWARD_SPEED, MAX_SPEED, MAX_TURN_SPEED, TURN_SPEED},
    resources::Game,
};
use bevy::prelude::*;

pub fn keyboard_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    // println!("Wheel Y speed {:?}", game.player_wheel.speed_y);

    let forward_moment = FORWARD_SPEED * time.delta_seconds();
    let turn_moment = TURN_SPEED * time.delta_seconds();

    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        if game.player_wheel.speed_z < MAX_SPEED {
            game.player_wheel.speed_z += forward_moment;
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        if game.player_wheel.speed_z > -MAX_SPEED {
            game.player_wheel.speed_z -= forward_moment;
        }
    }

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        if game.player_wheel.speed_y < MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y += turn_moment;
            } else {
                game.player_wheel.speed_y -= turn_moment;
            }
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        if game.player_wheel.speed_y > -MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y -= turn_moment;
            } else {
                game.player_wheel.speed_y += turn_moment;
            }
        }
    }
}

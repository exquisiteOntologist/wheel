use crate::{
    constants::{FORWARD_SPEED, MAX_SPEED, MAX_TURN_SPEED, TURN_SPEED},
    resources::Game,
};
use bevy::prelude::*;

pub fn keyboard_control(keyboard_input: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    // println!("Wheel Y speed {:?}", game.player_wheel.speed_y);

    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        if game.player_wheel.speed_z < MAX_SPEED {
            game.player_wheel.speed_z += FORWARD_SPEED;
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        if game.player_wheel.speed_z > -MAX_SPEED {
            game.player_wheel.speed_z -= FORWARD_SPEED;
        }
    }

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        if game.player_wheel.speed_y < MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y += TURN_SPEED;
            } else {
                game.player_wheel.speed_y -= TURN_SPEED;
            }
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        if game.player_wheel.speed_y > -MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y -= TURN_SPEED;
            } else {
                game.player_wheel.speed_y += TURN_SPEED;
            }
        }
    }
}

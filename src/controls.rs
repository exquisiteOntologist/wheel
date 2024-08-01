use crate::{
    constants::{FORWARD_SPEED, MAX_SPEED, MAX_TURN_SPEED, TURN_SPEED},
    resources::{DebugRoller, Game},
    ui::letterbox::resources::LetterboxState,
};
use bevy::prelude::*;

pub fn keyboard_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    // println!("Wheel Y speed {:?}", game.player_wheel.speed_y);

    let forward = FORWARD_SPEED * time.delta_seconds();
    let turn = TURN_SPEED * time.delta_seconds();

    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        if game.player_wheel.speed_z < MAX_SPEED {
            game.player_wheel.speed_z += forward;
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        if game.player_wheel.speed_z > -MAX_SPEED {
            game.player_wheel.speed_z -= forward;
        }
    }

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        if game.player_wheel.speed_y < MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y += turn;
            } else {
                game.player_wheel.speed_y -= turn;
            }
        }
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        if game.player_wheel.speed_y > -MAX_TURN_SPEED {
            if game.player_wheel.speed_z >= 0. {
                game.player_wheel.speed_y -= turn;
            } else {
                game.player_wheel.speed_y += turn;
            }
        }
    }
}

pub fn keyboard_control_debugging(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut debug_roller: ResMut<DebugRoller>,
    mut letterbox_state: ResMut<LetterboxState>,
) {
    const INC: f32 = 0.1;

    let inc = if keyboard_input.any_pressed([KeyCode::PageDown]) {
        // page down down
        -INC
    } else {
        // page up or nothing up
        INC
    };

    if keyboard_input.any_pressed([KeyCode::Digit1]) {
        debug_roller.x = (debug_roller.x + inc).clamp(-1., 1.);
    }
    if keyboard_input.any_pressed([KeyCode::Digit2]) {
        debug_roller.y = (debug_roller.y + inc).clamp(-1., 1.);
    }
    if keyboard_input.any_pressed([KeyCode::Digit3]) {
        debug_roller.z = (debug_roller.z + inc).clamp(-1., 1.);
    }
    if keyboard_input.any_pressed([KeyCode::Digit4]) {
        debug_roller.w = (debug_roller.w + inc).clamp(-1., 1.);
    }
    if keyboard_input.any_just_pressed([KeyCode::Space]) {
        letterbox_state.active = !letterbox_state.active;
    }
}

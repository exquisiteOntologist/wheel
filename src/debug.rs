use bevy::prelude::ResMut;

use crate::{
    levels::resources::{InitState, LevelState},
    resources::DebugState,
};

pub fn debug_reset_actors(mut debug: ResMut<DebugState>, mut level_state: ResMut<LevelState>) {
    if debug.reset == false {
        return;
    }

    level_state.init = InitState::Loading;
    debug.reset = false;
}

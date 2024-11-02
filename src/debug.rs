use bevy::prelude::ResMut;

use crate::resources::DebugState;

pub fn debug_reset_actors(mut debug: ResMut<DebugState>) {
    if debug.reset == false {
        return;
    }

    debug.reset = false;

    //
}

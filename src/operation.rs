use bevy::{
    prelude::ResMut,
    time::{Time, Virtual},
};

/// pause or resume `Relative` time
/// Note this is not full pause functionality.
pub fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

use bevy::prelude::{ResMut, World};

use crate::levels::resources::InitState;

use super::resources::LevelState;

/// Generic level function that runs for every level
pub fn setup_level() {
    //
}

/// Clear the entire world before we populate it again (primary app's World)
pub fn unload_world(world: &mut World) {
    world.clear_all();
}

/// Clear the content of the current level from the world
pub fn unload_level(world: &mut World) {
    println!("unloading level");
    world.clear_entities();
}

/// Sets the init value of the level state to false
pub fn finish_level_init(mut level_state: ResMut<LevelState>) {
    level_state.init = InitState::Finished;
    println!("finished level init");
}

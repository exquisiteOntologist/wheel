use bevy::prelude::{Query, Res, Transform, With, World};

use crate::components::characters::player::resources::PlayerCharacter;

use super::resources::{LevelState, SceneId};

/// Generic level function that runs for every level
pub fn setup_level() {
    //
}

/// Clear the entire world before we populate it again (primary app's World)
pub fn unload_world(world: &mut World) {
    world.clear_all();
}

/// Is the level still setting up?
pub fn cond_level_initialising(level: Res<LevelState>) -> bool {
    level.init
}

/// Check whether a SceneId is the current scene
pub fn cond_level_is(scene_id: SceneId) -> impl FnMut(Res<LevelState>) -> bool {
    move |level: Res<LevelState>| level.level == scene_id
}

pub fn cond_level_is_v2(scene_id: SceneId) -> impl FnMut(Option<Res<LevelState>>) -> bool + Clone {
    move |current_level_state: Option<Res<LevelState>>| match current_level_state {
        Some(current_level_state) => current_level_state.level == scene_id,
        None => false,
    }
}

/// Is the player entity missing from the world?
pub fn cond_player_missing(query: Query<&Transform, With<PlayerCharacter>>) -> bool {
    query.is_empty() || query.get_single().is_err()
}

/// Is the player entity in the world (opposite of missing)?
pub fn cond_player_present(query: Query<&Transform, With<PlayerCharacter>>) -> bool {
    !cond_player_missing(query)
}

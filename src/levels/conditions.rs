use bevy::prelude::{Query, Res, Transform, With};

use crate::{
    components::characters::player::resources::PlayerCharacter, levels::resources::InitState,
};

use super::resources::{LevelState, SceneId};

/// Is the level still setting up?
pub fn cond_level_initialising(level_state: Res<LevelState>) -> bool {
    if level_state.init != InitState::Finished {
        println!(
            "checking init state to see if it was true ({})",
            level_state.init != InitState::Finished
        );
    }
    level_state.init != InitState::Finished
}

pub fn cond_level_initialising_alt(level_state: Res<LevelState>) -> bool {
    if level_state.init != InitState::Finished {
        println!(
            "checking init on preupdate and it was true ({})",
            level_state.init != InitState::Finished
        );
    }
    level_state.init != InitState::Finished
}

/// The level is loading and not for the first time
pub fn cond_level_loading(level_state: Res<LevelState>) -> bool {
    if level_state.init == InitState::Loading {
        println!(
            "checked level loading state and it was true ({})",
            level_state.init == InitState::Loading
        );
    }
    level_state.init == InitState::Loading
}

/// Check whether a SceneId is the current scene
pub fn cond_level_is(scene_id: SceneId) -> impl FnMut(Option<Res<LevelState>>) -> bool + Clone {
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

use bevy::{
    app::{App, Plugin, PostStartup, Update},
    prelude::{IntoSystemConfigs, OnEnter},
};

use crate::{
    gens::particles::sand_particles::SandParticlesPlugin,
    levels::{
        common::{cond_player_missing, cond_player_present},
        resources::CurrentSceneState,
    },
};

use super::{
    effects::{attach_particles, update_particles_relative_to_char},
    movement::{move_character, turn_character},
};

pub struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_character,
                turn_character,
                update_particles_relative_to_char,
                // update_axis,
            )
                .run_if(cond_player_present),
        );
        app.add_plugins(SandParticlesPlugin);
        // app.add_systems(PostStartup, attach_particles);
        app.add_systems(OnEnter(CurrentSceneState::SceneSandHills), attach_particles);
    }
}

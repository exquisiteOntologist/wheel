use bevy::app::{App, Plugin, PostStartup, Update};

use crate::gens::particles::ParticlesPlugin;

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
            ),
        );
        app.add_plugins(ParticlesPlugin);
        app.add_systems(PostStartup, attach_particles);
    }
}

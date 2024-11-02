use bevy::prelude::{Entity, Query, Transform, With};

use crate::components::characters::player::resources::PlayerCharacter;

/// Generic level function that runs for every level
pub fn setup_level() {
    //
}

pub fn cond_player_missing(query: Query<&Transform, With<PlayerCharacter>>) -> bool {
    println!("Empty? {}", query.is_empty());
    query.is_empty() || query.get_single().is_err()
}

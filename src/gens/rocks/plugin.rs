use bevy::app::{App, Plugin, Update};

use super::update::{spawn_rocks_basic, update_rocks};

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_rocks, spawn_rocks_basic));
    }
}

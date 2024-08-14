use bevy::app::{App, Plugin, Update};

use super::terrain::update_terrain;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, update_terrain);
        app.add_systems(Update, update_terrain);
    }
}

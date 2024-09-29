use bevy::app::{App, Plugin, PostStartup, Update};

use super::terrain::update_terrain;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, update_terrain);
        // Run on update to generate terrain that hasn't been spawned
        app.add_systems(Update, update_terrain);
    }
}

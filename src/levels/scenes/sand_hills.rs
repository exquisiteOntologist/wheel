use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    prelude::World,
};

use crate::{
    gens::{
        clouds::CloudPlugin, grass::plugin::GrassPlugin, rocks::plugin::RockPlugin,
        terrain::plugin::TerrainPlugin,
    },
    utils::perlin::PerlinPlugin,
};

/// Sand Hills Scene.
/// Does not serialize (at present).
/// https://github.com/bevyengine/bevy/blob/main/examples/scene/scene.rs
pub fn scene_sand_hills_world(world: &mut World) {
    // Documentation suggests PluginGroups can be disabled via their build method.
    // This may allow enabling plugin groups for specific scenes.
    // world.
}

/// Plugins for the first level.
/// Generic plugins for all levels would go in a generic group.
pub struct LevelOnePlugins;

impl PluginGroup for LevelOnePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CloudPlugin)
            .add(PerlinPlugin)
            .add(TerrainPlugin)
            .add(GrassPlugin)
            .add(RockPlugin)
    }
}

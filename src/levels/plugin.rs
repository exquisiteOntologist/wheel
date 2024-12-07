use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::{resource_exists, IntoSystemConfigs, IntoSystemSetConfigs, OnEnter},
};

use crate::components::characters::player::spawn::spawn_player_with_wheel;

use super::{
    common::{
        cond_level_initialising, cond_level_is, cond_level_is_v2, cond_player_missing, setup_level,
        unload_world,
    },
    resources::{CurrentSceneState, LevelLoadSet, LevelState, SceneId},
    scenes::sand_hills::{scene_sand_hills_world, LevelOnePlugins},
};

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelState>();
        app.add_systems(Startup, setup_level);
        app.configure_sets(Update, LevelLoadSet.run_if(cond_level_initialising));

        let scene_sand_hills = (scene_sand_hills_world);
        app.add_systems(OnEnter(CurrentSceneState::SceneSandHills), scene_sand_hills);
        // we cannot conditionally toggle entire plugins
        app.add_plugins(LevelOnePlugins);

        // Note that this was before Bevy 0.15
        // and at the time unsure how to do it.
        // Since this code there are better conventions,
        // implementations, and guidelines for handling this.
        app.add_systems(Startup, (spawn_player_with_wheel));
        app.add_systems(
            Update,
            (
                // unload_world,
                spawn_player_with_wheel.run_if(cond_player_missing),
                scene_sand_hills
                    .run_if(resource_exists::<LevelState>)
                    .run_if(cond_level_is_v2(SceneId::LevelSandHills)),
            )
                .in_set(LevelLoadSet),
        );
    }
}

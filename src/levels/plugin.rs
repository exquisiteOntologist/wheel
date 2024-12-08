use bevy::{
    app::{App, Plugin, PostUpdate, PreUpdate, Startup, Update},
    prelude::{resource_exists, IntoSystemConfigs, IntoSystemSetConfigs, OnEnter},
};

use crate::components::characters::player::spawn::spawn_player_with_wheel;

use super::{
    common::{finish_level_init, setup_level, unload_level},
    conditions::{
        cond_level_initialising, cond_level_initialising_alt, cond_level_is, cond_level_loading,
        cond_player_missing,
    },
    resources::{
        CurrentSceneState, LevelLoadSet, LevelLoadSetPostUpdate, LevelLoadSetPreUpdate,
        LevelLoadSetUpdate, LevelState, SceneId,
    },
    scenes::sand_hills::{scene_sand_hills_world, LevelOnePlugins},
};

pub struct LevelsPlugin;

pub fn did_preupdate() {
    println!("did pre-update");
}

pub fn ran_update() {
    println!("ran update");
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelState>();
        app.add_systems(Startup, setup_level);

        app.configure_sets(
            PostUpdate,
            LevelLoadSetPostUpdate.run_if(cond_level_initialising),
        );

        let scene_sand_hills = (scene_sand_hills_world);
        app.add_systems(OnEnter(CurrentSceneState::SceneSandHills), scene_sand_hills);
        // we cannot conditionally toggle entire plugins
        app.add_plugins(LevelOnePlugins);

        LevelLoadSetPreUpdate.before(LevelLoadSetUpdate);

        // Note that this was before Bevy 0.15,
        // and at the time unsure how to handle loading deps for levels.
        // Since this code there are better conventions,
        // implementations, and guidelines for handling this.
        app.add_systems(Startup, (spawn_player_with_wheel));
        // Do not use PreUpdate - it cancels the other systems
        // app.add_systems(
        //     PreUpdate,
        //     (did_preupdate, unload_level.run_if(cond_level_loading)).in_set(LevelLoadSet),
        // );

        app.add_systems(
            Update,
            (did_preupdate, unload_level.run_if(cond_level_loading))
                .in_set(LevelLoadSetPreUpdate)
                .run_if(cond_level_initialising),
        );

        app.add_systems(
            Update,
            (
                spawn_player_with_wheel.run_if(cond_player_missing),
                scene_sand_hills
                    .run_if(resource_exists::<LevelState>)
                    .run_if(cond_level_is(SceneId::LevelSandHills)),
                ran_update,
            )
                .run_if(cond_level_initialising)
                .after(LevelLoadSetPreUpdate),
            // .in_set(LevelLoadSetUpdate),
        );
        app.add_systems(
            PostUpdate,
            (finish_level_init).in_set(LevelLoadSetPostUpdate),
        );
    }
}

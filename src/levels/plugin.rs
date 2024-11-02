use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::IntoSystemConfigs,
};

use crate::components::characters::player::spawn::spawn_player_with_wheel;

use super::common::{cond_player_missing, setup_level};

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<LevelsState>();
        app.add_systems(Startup, setup_level);
        app.add_systems(
            Update,
            (spawn_player_with_wheel).run_if(cond_player_missing),
        );

        // app.add_systems(Update, (level-one).run_if(run_if_level_one_fn));
    }
}

use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::IntoSystemConfigs,
};

use super::{
    conditions::{cond_just_check_if_rain_grid_need_recalc, update_diff_values},
    resources::{RainComparisons, RainState},
    update::{rain_infreq_value_calc, rain_iteration},
};

pub struct RainPlugin;

impl Plugin for RainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RainState>();
        // Setup the diff state so that we can recalculate the grid when the values change.
        app.init_resource::<RainComparisons>();
        app.add_systems(Startup, (update_diff_values, rain_infreq_value_calc));
        app.add_systems(
            Update,
            (
                // here we are recalculating the grid values if they change
                update_diff_values,
                rain_infreq_value_calc.run_if(cond_just_check_if_rain_grid_need_recalc),
            ),
        );
        // Here we perform rain updates that perform every iteration.
        app.add_systems(Update, (rain_iteration));
    }
}

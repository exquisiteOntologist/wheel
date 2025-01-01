use bevy::prelude::{Res, ResMut};
use rand::Rng;

use super::resources::{RainComparisons, RainState};

pub fn get_activation_comparison_value() -> i32 {
    rand::thread_rng().gen_range(0..5)
}

pub fn should_activate(i: i32, cmp: i32) -> bool {
    i % cmp == 1
}

pub fn update_diff_values(mut rain_comps: ResMut<RainComparisons>, rain_state: Res<RainState>) {
    rain_comps.diff_spacing.update_value(rain_state.spacing);
    rain_comps.diff_capacity.update_value(rain_state.capacity);
}

/// Check if time to do an update and update the comparis onvalues.
/// TODO: Get function signature correct or wait for support
pub fn cond_check_if_rain_grid_need_recalc(
) -> impl FnMut(Option<(ResMut<RainComparisons>, Res<RainState>)>) -> bool + Clone {
    move |values: Option<(ResMut<RainComparisons>, Res<RainState>)>| match values {
        Some((mut rain_comps, rain_state)) => {
            rain_comps
                .diff_spacing
                .compare_and_update(&rain_state.spacing)
                && rain_comps
                    .diff_capacity
                    .compare_and_update(&rain_state.capacity)
        }
        None => false,
    }
}

pub fn cond_just_check_if_rain_grid_need_recalc(
    rain_comps: Res<RainComparisons>,
    rain_state: Res<RainState>,
) -> bool {
    rain_comps.diff_spacing.compare_to(&rain_state.spacing)
        && rain_comps.diff_capacity.compare_to(&rain_state.capacity)
}

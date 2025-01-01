use std::collections::HashMap;

use bevy::prelude::{Query, ResMut, Transform, With};

use crate::components::characters::player::resources::PlayerCharacter;

use super::{
    locator::{calc_drip_pos, calc_grid_infreq_values, get_grid_start},
    resources::{Drip, RainState, XZ},
};

pub fn rain_infreq_value_calc(mut rain_state: ResMut<RainState>) {
    rain_state.row_spec = calc_grid_infreq_values(&rain_state);
}

pub fn rain_iteration(
    mut rain_state: ResMut<RainState>,
    mut q: Query<&mut Transform, With<PlayerCharacter>>,
) {
    let player_pos = q.single();
    let grid_start = get_grid_start(&rain_state.row_spec.row_length_half, &player_pos);

    // is this expensive?
    let mut updated_drops: HashMap<i32, Drip> = HashMap::new();

    let mut drips = rain_state.drips;
    for (i, mut drip) in drips.iter_mut().enumerate() {
        let i: i32 = i as i32;
        if i == 0 {
            println!("drip i=0 {}", drip.x);
        }

        if drip.enabled {
            // if the drip is still activated continue transforming it,
            // regardless if over the current intended capacity (rain must land)
            transform_drop();
            continue;
        }

        println!(
            "drip not enabled, turning drip on. enabled {}",
            drip.enabled
        );

        // if over capactity don't enable any more drips
        if &i > &rain_state.capacity {
            // stop iterating as we have gone over
            return;
        }

        // at this point the drip is disabled, so we determine whether to reactivate
        let refresh_this_drop = true;

        if refresh_this_drop {
            let new_drip = drop_refresh(&rain_state, &grid_start, &i);
            updated_drops.insert(i, new_drip);
        }
    }

    if !updated_drops.is_empty() {
        for (i, updated) in updated_drops.iter() {
            rain_state.drips[*i as usize] = *updated;
        }
    }
}

/// Replaces a drop with a new drop.
fn drop_refresh(rain_state: &RainState, grid_start: &XZ, i: &i32) -> Drip {
    let drip_pos = calc_drip_pos(rain_state, grid_start, i);

    Drip {
        x: drip_pos.x,
        y: 0.,
        z: drip_pos.z,
        y_end: 0.,
        weight: 0.,
        velocity: 0.,
        angle: 0.,
        angle_b: 0.,
        // since we are refreshing the drip this must be `true`
        enabled: true,
    }
}

/// Moves drop.
fn transform_drop() {
    //
}

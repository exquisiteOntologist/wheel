use bevy::prelude::Resource;

use crate::utils::update_checks::ComparisonBody;

use super::constants::{RAIN_COUNT, RAIN_SPACE};

#[derive(Clone)]
pub struct XZ {
    pub x: f32,
    pub z: f32,
}

/// The grid cursor for the current drop.
pub struct GridCursor {
    pub col_i: i32,
    pub row_i: i32,
}

impl GridCursor {
    pub fn col_i_f32(&self) -> f32 {
        self.col_i as f32
    }

    pub fn row_i_f32(&self) -> f32 {
        self.row_i as f32
    }
}

/// A drop of rain, ALA a drip.
#[derive(Default, Clone, Copy)]
pub struct Drip {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub y_end: f32,
    pub weight: f32,
    pub velocity: f32,
    pub angle: f32,
    pub angle_b: f32,
    pub enabled: bool,
}

impl Drip {
    // pub fn update_drip(&mut self, new: Self) -> &Self {
    //     self.x = new.x;
    //     self.y = new.y;
    //     self.z = new.z;
    //     self.y_end = new.y_end;
    //     self.weight = new.weight;
    //     self.velocity = new.velocity;
    //     self.angle = new.angle;
    //     self.angle_b = new.angle_b;
    //     self.enabled = new.enabled;
    //     self
    // }
}

/// The rain state covers all of the rain in the scene.
#[derive(Resource, Clone, Copy)]
pub struct RainState {
    /// Whether to rain or not to rain.
    pub is_raining: bool,
    /// Space between drip points on both dimensions.
    pub spacing: f32,
    /// The capacity is the dynamic number of rain drops in a scene.
    /// It must be <= the drips array length.
    /// If the capacity goes down we don't disable or remove drops, we just won't add any more.
    /// The drops will perish themselves when they hit the ground.
    pub capacity: i32,
    /// The drips array is filled with drips and never changes capacity.
    /// After a drip perishes, the same drip object is reinsantiated with new values.
    pub drips: [Drip; RAIN_COUNT],
    /// The current specification for a single row. Update the values when changing capacity.
    pub row_spec: RainGridRowSpec,
}

impl Default for RainState {
    fn default() -> Self {
        Self {
            is_raining: true,
            spacing: RAIN_SPACE,
            capacity: RAIN_COUNT as i32,
            drips: Default::default(),
            row_spec: Default::default(),
        }
    }
}

#[test]
pub fn test_default_rain_state() {
    let rain = RainState::default();
    assert!(
        rain.capacity <= rain.drips.len() as i32,
        "The default rain capacity is greater than the number of drips in memory."
    );
}

#[derive(Default, Clone, Copy)]
pub struct RainGridRowSpec {
    pub row_capacity: i32,
    pub row_length_half: f32,
}

#[derive(Resource, Default)]
pub struct RainComparisons {
    pub diff_spacing: ComparisonBody<f32>,
    pub diff_capacity: ComparisonBody<i32>,
}

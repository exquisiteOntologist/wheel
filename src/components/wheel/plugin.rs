use bevy::app::{App, Plugin, Startup, Update};

use super::{
    resources::WheelState,
    setup::setup_wheel,
    wheel::{move_wheel, wheel_rotation},
};

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WheelState>();
        app.add_systems(Startup, setup_wheel);
        app.add_systems(Update, (wheel_rotation, move_wheel));
    }
}

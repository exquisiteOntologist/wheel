use bevy::app::{App, Plugin, Startup};

use super::setup::setup;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

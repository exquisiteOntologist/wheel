use bevy::app::{App, Plugin, Startup, Update};

use super::movement::{move_camera, setup_camera};

pub struct PCameraPlugin;

impl Plugin for PCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, move_camera);
    }
}

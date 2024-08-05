use bevy::app::{App, Plugin, Startup, Update};

use super::{
    movement::{adjust_camera_speed, move_camera, move_camera_gravity, move_camera_old},
    setup::setup_camera,
};

pub struct PCameraPlugin;

impl Plugin for PCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(
            Update,
            (adjust_camera_speed, move_camera, move_camera_gravity),
        );
    }
}

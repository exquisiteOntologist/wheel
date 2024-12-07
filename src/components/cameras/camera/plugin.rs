use bevy::app::{App, Plugin, Startup, Update};

use super::{
    movement::{adjust_camera_speed, move_camera, move_camera_gravity},
    setup::setup_camera,
};

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(
            Update,
            (adjust_camera_speed, move_camera, move_camera_gravity),
        );
    }
}

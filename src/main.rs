//! Rolls a player-controlled wheel

use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    pbr::DirectionalLightShadowMap,
    prelude::*,
};
use wheel::{
    camera::move_camera,
    controls::keyboard_control,
    resources::Game,
    setup::{setup, setup_scene_once_loaded},
    wheel::{move_wheel, spin_wheel},
};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .insert_resource(DirectionalLightShadowMap { size: 8192 })
        .insert_resource(Msaa::Sample4)
        .init_resource::<Game>()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Wheel".into(),
                name: Some("Wheel.app".into()),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }),))
        // .add_plugins((TemporalAntiAliasPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                setup_scene_once_loaded,
                spin_wheel,
                move_wheel,
                move_camera,
                keyboard_control,
            ),
        )
        .run();
}

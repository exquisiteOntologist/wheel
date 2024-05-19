//! Rolls a player-controlled wheel

use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use iyes_perf_ui::PerfUiPlugin;
use wheel::{
    camera::PCameraPlugin,
    controls::keyboard_control,
    gens::clouds::CloudPlugin,
    resources::Game,
    setup::{setup, setup_scene_once_loaded},
    utils::colours::rgb,
    wheel::WheelPlugin,
};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .insert_resource(DirectionalLightShadowMap { size: 8192 })
        .insert_resource(ClearColor(rgb(52., 167., 211.)))
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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins((PCameraPlugin, WheelPlugin, CloudPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (setup_scene_once_loaded, keyboard_control))
        .run();
}

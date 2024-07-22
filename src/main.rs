//! Rolls a player-controlled wheel

use bevy::{
    input::common_conditions::input_just_pressed, pbr::DirectionalLightShadowMap, prelude::*,
    window::WindowFocused,
};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use iyes_perf_ui::PerfUiPlugin;
use wheel::{
    components::{camera::PCameraPlugin, character::CharacterPlugin, wheel::WheelPlugin},
    controls::keyboard_control,
    gens::{clouds::CloudPlugin, particles::ParticlesPlugin, terrain::TerrainPlugin},
    operation::toggle_pause,
    resources::Game,
    setup::{setup, setup_scene_once_loaded},
    utils::colours::rgb,
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
        .add_systems(
            Update,
            (toggle_pause.run_if(input_just_pressed(KeyCode::Escape)),),
        )
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins((PerfUiPlugin, RapierPhysicsPlugin::<NoUserData>::default()))
        .add_plugins((CharacterPlugin, WheelPlugin))
        .add_plugins((PCameraPlugin, CloudPlugin, TerrainPlugin))
        // .add_plugins((ParticlesPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (setup_scene_once_loaded, keyboard_control))
        .run();
}

//! Rolls a player-controlled wheel

use bevy::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, pbr::DirectionalLightShadowMap};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use iyes_perf_ui::PerfUiPlugin;
use wheel::components::cameras::camera::plugin::PCameraPlugin;
use wheel::components::characters::player::plugin::PlayerCharacterPlugin;
use wheel::controls::keyboard_control_debugging;
use wheel::gens::grass::GrassPlugin;
use wheel::resources::DebugRoller;
use wheel::ui::plugin::UserInterfacePlugin;
use wheel::utils::perlin::PerlinPlugin;
use wheel::{
    components::wheel::WheelPlugin,
    controls::keyboard_control,
    gens::{clouds::CloudPlugin, terrain::TerrainPlugin},
    operation::toggle_pause,
    resources::Game,
    setup::setup,
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
        .init_resource::<DebugRoller>()
        // .add_plugins((DebugGridPlugin::without_floor_grid()))
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
        .add_plugins((PlayerCharacterPlugin, WheelPlugin))
        .add_plugins((PCameraPlugin, CloudPlugin))
        .add_plugins((PerlinPlugin, TerrainPlugin, GrassPlugin))
        .add_plugins((UserInterfacePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_control, keyboard_control_debugging))
        .run();
}

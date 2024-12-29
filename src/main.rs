use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::gltf::GltfLoader;
use bevy::image::{CompressedImageFormats, ImageLoader};
use bevy::prelude::*;
use bevy::render::render_resource::ShaderLoader;
use bevy::text::FontLoader;
use bevy::utils::hashbrown::HashMap;
use bevy::{input::common_conditions::input_just_pressed, pbr::DirectionalLightShadowMap};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use iyes_perf_ui::PerfUiPlugin;
use wheel::components::cameras::camera::plugin::PlayerCameraPlugin;
use wheel::components::characters::player::plugin::PlayerCharacterPlugin;
use wheel::controls::keyboard_control_debugging;
use wheel::debug::debug_reset_actors;
use wheel::levels::plugin::LevelsPlugin;
use wheel::resources::{DebugRoller, DebugState};
use wheel::setup::{setup, setup_framerate};
use wheel::ui::plugin::UserInterfacePlugin;
use wheel::{
    components::wheel::plugin::WheelPlugin, controls::keyboard_control, operation::toggle_pause,
    resources::Game, utils::colours::rgb,
};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Wheel".into(),
                    name: Some("Wheel.app".into()),
                    resolution: (1920., 1080.).into(),
                    // resolution: (3840., 2160.).into(),
                    // mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                mode: AssetMode::Processed,
                ..default()
            }),))
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
            LogDiagnosticsPlugin::default(),
            // ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(1.0 / 30.)),
            bevy_framepace::FramepacePlugin,
        ))
        .add_plugins((PerfUiPlugin, RapierPhysicsPlugin::<NoUserData>::default()))
        // asset loaders: https://docs.rs/bevy/latest/bevy/asset/trait.AssetLoader.html
        // .register_asset_loader(AnimationGraphAssetLoader::default())
        // .register_asset_loader(FontLoader::default())
        // .register_asset_loader(GltfLoader {
        //     supported_compressed_formats: CompressedImageFormats::all(),
        //     custom_vertex_attributes: HashMap::new(),
        // })
        // .register_asset_loader(ImageLoader::new(CompressedImageFormats::all()))
        // .register_asset_loader(ShaderLoader::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.,
        })
        .insert_resource(DirectionalLightShadowMap {
            size: 1024, // size: 8192
        })
        .insert_resource(ClearColor(rgb(52., 167., 211.)))
        .init_resource::<Game>()
        .init_resource::<DebugRoller>()
        .init_resource::<DebugState>()
        // .add_plugins((DebugGridPlugin::without_floor_grid()))
        .add_systems(
            Update,
            (toggle_pause.run_if(input_just_pressed(KeyCode::Escape)),),
        )
        .add_plugins((PlayerCharacterPlugin, WheelPlugin))
        .add_plugins((PlayerCameraPlugin))
        // .add_plugins((CloudPlugin))
        // .add_plugins((PerlinPlugin, TerrainPlugin, GrassPlugin, RockPlugin))
        .add_plugins(UserInterfacePlugin)
        .add_systems(Startup, (setup, setup_framerate))
        .add_plugins(LevelsPlugin)
        .add_systems(Update, (keyboard_control, keyboard_control_debugging))
        .add_systems(Update, (debug_reset_actors))
        .run();
}

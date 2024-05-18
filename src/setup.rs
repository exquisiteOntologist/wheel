use std::f32::consts::PI;

use bevy::{
    animation::AnimationPlayer,
    asset::{AssetServer, Assets},
    core_pipeline::{
        core_3d::Camera3dBundle, experimental::taa::TemporalAntiAliasSettings,
        tonemapping::DebandDither,
    },
    ecs::{
        query::Added,
        system::{Commands, Query, Res, ResMut},
    },
    math::{primitives::Plane3d, EulerRot, Quat, Vec3},
    pbr::{
        light_consts, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle,
        FogFalloff, FogSettings, PbrBundle, ScreenSpaceAmbientOcclusionSettings, StandardMaterial,
    },
    render::{
        color::Color,
        mesh::{Mesh, Meshable},
    },
    scene::SceneBundle,
    transform::components::Transform,
    utils::default,
};
use iyes_perf_ui::{
    diagnostics::{PerfUiEntryFPS, PerfUiEntryFPSWorst},
    PerfUiRoot,
};

use crate::{
    constants::MAX_SPEED,
    meshes::{image_settings_with_repeat_image_sampler, mesh_update_uv},
    resources::{Animations, Game, PlayerCamera, PlayerCharacter},
    utils::colours::rgba,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    // Insert a resource with the current scene information
    // taken from the Bevy fox example (but not used, yet)
    commands.insert_resource(Animations(vec![
        // asset_server.load("models/animated/Fox.glb#Animation2"),
        // asset_server.load("models/animated/Fox.glb#Animation1"),
        // asset_server.load("models/animated/Fox.glb#Animation0"),
        // asset_server.load("models/Wheel.glb#x"),
    ]));

    // Performance FPS
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 3.0, 0.0)
                .looking_at(Vec3::new(0.0, 1.0, -0.0), Vec3::Y),
            dither: DebandDither::Enabled,
            ..default()
        },
        FogSettings {
            // color: Color::rgba(0.13, 0.14, 0.17, 1.),
            // color: Color::rgba(52. / 255., 167. / 255., 211. / 255., 0.5),
            color: rgba(52., 167., 211., 0.5),
            falloff: FogFalloff::Linear {
                start: 100.0,
                end: 160.0,
            },
            // falloff: FogFalloff::from_visibility_color(0.3, Color::rgba(1., 1., 1., 1.)),
            // falloff: FogFalloff::Atmospheric {
            //     extinction: Vec3::new(x, y, z),
            //     inscattering: Vec3::new(x, y, z),
            // },
            // falloff: FogFalloff::Exponential { density: 0.03 },
            // objects retain visibility (>= 5% contrast) for up to 15 units
            // falloff: FogFalloff::from_visibility(70.0),
            ..default()
        },
        TemporalAntiAliasSettings { ..default() },
        ScreenSpaceAmbientOcclusionSettings { ..default() },
        PlayerCamera,
    ));

    // Fog
    // commands.spawn(
    //     (),
    // );

    let image_ground_settings = image_settings_with_repeat_image_sampler();
    let texture_ground =
        asset_server.load_with_settings("textures/tex_exp.png", image_ground_settings);

    let ground_size = (5000., 5000.);
    let mut ground_mesh: Mesh = Plane3d::default()
        .mesh()
        .size(ground_size.0, ground_size.1)
        .build();

    mesh_update_uv(&mut ground_mesh, ground_size.0 / 2., ground_size.1 / 2.);

    // Plane
    commands.spawn(PbrBundle {
        // mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
        mesh: meshes.add(ground_mesh),
        // material: materials.add(Color::hex("#887A63").unwrap().as_rgba()),
        // see https://bevyengine.org/news/bevy-0-12/#asset-meta-files
        // see https://github.com/bevyengine/bevy/issues/399#issuecomment-2042133456
        material: materials.add(StandardMaterial {
            // base_color: Color::hex("#887A63").unwrap(),
            base_color_texture: Some(texture_ground.clone()),
            // alpha_mode: bevy::pbr::AlphaMode::Opaque,
            fog_enabled: true,
            ..default()
        }),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, -PI / 3.5)),
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            // first_cascade_far_bound: 200.0,
            // maximum_distance: 400.0,
            maximum_distance: 400.0,
            first_cascade_far_bound: 0.9,
            ..default()
        }
        .into(),
        ..default()
    });

    // Wheel
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Wheel.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 1.2, 0.0),
            ..default()
        },
        // PbrBundle {
        //     mesh: asset_server.load("models/Wheel.glb#Mesh0"),
        //     ..default()
        // },
        PlayerCharacter,
    ));

    game.player_wheel.speed_z = MAX_SPEED;

    println!("Controls:");
    println!("  - arrow up / down: roll");
    println!("  - arrow left / right: turn direction");
}

// Once the scene is loaded, start the animation
pub fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

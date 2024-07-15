use std::f32::consts::PI;

use bevy::{
    animation::AnimationPlayer,
    asset::{AssetServer, Assets},
    color::Color,
    ecs::{
        query::Added,
        system::{Commands, Query, Res, ResMut},
    },
    math::{primitives::Plane3d, EulerRot, Quat},
    pbr::{
        light_consts, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle,
        PbrBundle, StandardMaterial,
    },
    render::mesh::{Mesh, Meshable},
    scene::SceneBundle,
    transform::components::Transform,
    utils::default,
};
use iyes_perf_ui::{
    prelude::{PerfUiEntryFPS, PerfUiEntryFPSWorst},
    ui::root::PerfUiRoot,
};

use crate::{
    constants::MAX_SPEED,
    meshes::{image_settings_with_repeat_image_sampler, mesh_update_uv},
    resources::{Animations, Game, PlayerCharacter},
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

    // Fog
    // commands.spawn(
    //     (),
    // );

    let image_ground_settings = image_settings_with_repeat_image_sampler();
    let texture_ground =
        asset_server.load_with_settings("textures/tex_exp.png", image_ground_settings);

    let ground_size = (5000., 5000.);
    let mut ground_mesh: Mesh =
        Mesh::from(Plane3d::default().mesh().size(ground_size.0, ground_size.1));

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
            color: Color::srgb(1.0, 1.0, 1.0),
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
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        // PbrBundle {
        //     mesh: asset_server.load("models/Wheel.glb#Mesh0"),
        //     ..default()
        // },
        PlayerCharacter,
    ));

    println!("Controls:");
    println!("  - arrow up / down: roll");
    println!("  - arrow left / right: turn direction");
    println!("  - arrow left / right: tilt");
}

// Once the scene is loaded, start the animation
pub fn setup_scene_once_loaded(
    _animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut _player in &mut players {
        // player.play(animations.0[0].clone_weak()).repeat();
    }
}

use std::f32::consts::PI;

use bevy::{
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    ecs::system::{Commands, Res, ResMut},
    hierarchy::BuildChildren,
    math::{EulerRot, Quat},
    pbr::{
        light_consts, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle,
        StandardMaterial,
    },
    prelude::SpatialBundle,
    render::mesh::Mesh,
    scene::SceneBundle,
    transform::components::Transform,
    utils::default,
};
use bevy_framepace::FramepaceSettings;
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};
use iyes_perf_ui::{
    prelude::{PerfUiEntryFPS, PerfUiEntryFPSWorst},
    ui::root::PerfUiRoot,
};

use crate::{
    components::{
        characters::player::{resources::PlayerCharacter, spawn::spawn_player},
        wheel::spawn::spawn_wheel,
    },
    constants::SPAWN_TRANSFORM,
    resources::Game,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    game: ResMut<Game>,
) {
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

    // let image_ground_settings = image_settings_with_repeat_image_sampler();
    // let texture_ground =
    //     asset_server.load_with_settings("textures/ground/sand.png", image_ground_settings);

    // let ground_size = (5000., 5000.);
    // let mut ground_mesh: Mesh =
    //     Mesh::from(Plane3d::default().mesh().size(ground_size.0, ground_size.1));

    // mesh_update_uv(&mut ground_mesh, ground_size.0 / 2., ground_size.1 / 2.);

    // // Plane
    // commands.spawn(PbrBundle {
    //     // mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
    //     mesh: meshes.add(ground_mesh),
    //     // material: materials.add(Color::hex("#887A63").unwrap().as_rgba()),
    //     // see https://bevyengine.org/news/bevy-0-12/#asset-meta-files
    //     // see https://github.com/bevyengine/bevy/issues/399#issuecomment-2042133456
    //     material: materials.add(StandardMaterial {
    //         // base_color: Color::hex("#887A63").unwrap(),
    //         base_color_texture: Some(texture_ground.clone()),
    //         // alpha_mode: bevy::pbr::AlphaMode::Opaque,
    //         fog_enabled: true,
    //         ..default()
    //     }),
    //     ..default()
    // });

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
            maximum_distance: 400.0,
            first_cascade_far_bound: 0.9,
            ..default()
        }
        .into(),
        ..default()
    });

    // Sun Blocker -- to test perf difference when blocking the horizon
    // let wall_mesh = Mesh::from(),
    // let sun_blocker = commands.spawn(PbrBundle {
    //     mesh: meshes.add(wall_mesh),
    //     ..default()
    // });

    println!("Controls:");
    println!("  - arrow up / down: roll");
    println!("  - arrow left / right: turn direction");
    println!("  - 'r' key: reset");
}

pub fn setup_framerate(mut settings: ResMut<FramepaceSettings>) {
    // settings.limiter = Limiter::from_framerate(32.0);
}

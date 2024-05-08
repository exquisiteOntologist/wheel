use std::f32::consts::PI;

use bevy::{
    animation::AnimationPlayer,
    asset::{AssetServer, Assets},
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        query::Added,
        system::{Commands, Query, Res, ResMut},
        world::FromWorld,
    },
    math::{primitives::Plane3d, EulerRot, Quat, Vec2, Vec3},
    pbr::{
        light_consts, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle,
        PbrBundle, StandardMaterial,
    },
    render::{
        camera::{PerspectiveProjection, Projection},
        color::Color,
        mesh::{shape::Quad, Mesh, Meshable, VertexAttributeValues},
        texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    },
    scene::SceneBundle,
    transform::components::Transform,
    utils::default,
};

use crate::{
    constants::MAX_SPEED,
    resources::{Animations, Game, PlayerCamera, PlayerCharacter},
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

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 3.0, 0.0)
                .looking_at(Vec3::new(0.0, 1.0, -0.0), Vec3::Y),
            ..default()
        },
        PlayerCamera,
    ));

    // this is necessary, but also necessary is for the UV of the mesh to repeat
    // https://www.reddit.com/r/bevy/comments/18qoctw/how_do_i_make_a_texture_tilerepeat_in_a_material/?rdt=35295
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    // let tex_sand = asset_server.load("textures/tex_exp.png");
    let tex_checkers = asset_server.load_with_settings("textures/checkers.png", settings);

    let mut ground_mesh: Mesh = Plane3d::default().mesh().size(40., 40.).build();

    if let Some(VertexAttributeValues::Float32x2(uvs)) =
        ground_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
    {
        for uv in uvs {
            uv[0] *= 20.;
            uv[1] *= 20.;
        }
    };

    // Plane
    commands.spawn(PbrBundle {
        // mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
        mesh: meshes.add(ground_mesh),
        // material: materials.add(Color::hex("#887A63").unwrap().as_rgba()),
        // see https://bevyengine.org/news/bevy-0-12/#asset-meta-files
        // see https://github.com/bevyengine/bevy/issues/399#issuecomment-2042133456
        material: materials.add(StandardMaterial {
            // base_color: Color::hex("#887A63").unwrap(),
            base_color_texture: Some(tex_checkers.clone()),
            // alpha_mode: bevy::pbr::AlphaMode::Opaque,
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

use crate::{constants::SPAWN_TRANSFORM, resources::PlayerCamera, utils::colours::rgba};
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        experimental::taa::TemporalAntiAliasSettings,
        tonemapping::{DebandDither, Tonemapping},
    },
    pbr::ScreenSpaceAmbientOcclusionSettings,
    prelude::*,
};
use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};

pub fn setup_camera(mut commands: Commands) {
    let mut camera = commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                // clear_color: Color::BLACK.into(),
                ..default()
            },
            // intentionally starting a way behind player so it moves in
            transform: Transform::from_xyz(-100.0, SPAWN_TRANSFORM.translation.y + 3.0 + 20., 0.0)
                .looking_at(Vec3::new(0.0, 1.0, -0.0), Vec3::Y),
            deband_dither: DebandDither::Enabled,
            tonemapping: Tonemapping::TonyMcMapface,
            // tonemapping: Tonemapping::None,
            // projection: bevy::prelude::Projection::Perspective(PerspectiveProjection {
            //     far: VIEW_DISTANCE,
            //     ..default()
            // }),
            ..default()
        },
        FogSettings {
            // color: Color::rgba(0.13, 0.14, 0.17, 1.),
            // color: Color::rgba(52. / 255., 167. / 255., 211. / 255., 0.5),
            color: rgba(52., 167., 211., 0.5),
            falloff: FogFalloff::Linear {
                // start: 100.0,
                // end: 160.0,
                start: 200.0,
                end: 260.0,
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
        // bloom is what adds the intense shine on ground (+ everywhere)
        // BloomSettings::default(),
        TemporalAntiAliasSettings { ..default() },
        ScreenSpaceAmbientOcclusionSettings { ..default() },
        PlayerCamera,
    ));

    camera
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(3.0))
        .insert(KinematicCharacterController {
            normal_nudge_factor: 1.0e-3,
            ..default()
        })
        .insert(Name::new("Camera"));
}

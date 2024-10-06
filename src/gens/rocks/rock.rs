use bevy::{
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    ecs::system::{Commands, Res, ResMut},
    hierarchy::BuildChildren,
    math::{primitives, EulerRot, Quat},
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
use bevy::{
    color::palettes::basic::SILVER,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use std::f32::consts::PI;

use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_pbr::PbrBundle;

use crate::constants::SPAWN_Y;

use super::{materials::rock_material, resources::Rock};

pub fn spawn_rock(
    mut commands: &Commands,
    mut asset_server: &Res<AssetServer>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut images: &ResMut<Assets<Image>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    i: u32,
) -> (MaterialMeshBundle<StandardMaterial>, Rock) {
    let mat = rock_material(asset_server, images, materials);
    let mut sphere_mesh = Sphere::new(1.0).mesh().build();
    sphere_mesh
        .generate_tangents()
        .expect("Failed to generate tangents");
    let mesh = meshes.add(Mesh::from(sphere_mesh));

    let pos_offset = i as f32 * 10.;

    let rock = (
        PbrBundle {
            mesh,
            material: mat,
            transform: Transform::from_xyz(0. + pos_offset, SPAWN_Y, 0.)
                .with_rotation(Quat::from_rotation_x(-PI / 1.)),
            ..default()
        },
        Rock,
    );

    rock
}

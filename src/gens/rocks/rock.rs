use bevy::prelude::*;
use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res, ResMut},
    math::Quat,
    pbr::StandardMaterial,
    render::mesh::Mesh,
    transform::components::Transform,
};
use std::f32::consts::PI;

use crate::constants::SPAWN_Y;

use super::types::RockBundle;
use super::{materials::rock_material, resources::Rock};

pub fn spawn_rock(
    mut commands: &Commands,
    mut asset_server: &Res<AssetServer>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut images: &ResMut<Assets<Image>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    i: u32,
) -> RockBundle {
    let mut sphere_mesh = Sphere::new(1.0).mesh().build();
    sphere_mesh
        .generate_tangents()
        .expect("Failed to generate tangents");

    let pos_offset = i as f32 * 10.;

    (
        // PbrBundle {
        //     mesh: Mesh3d(mesh),
        //     material: MeshMaterial3d(mat),
        //     transform: Transform::from_xyz(0. + pos_offset, SPAWN_Y, 0.)
        //         .with_rotation(Quat::from_rotation_x(-PI / 1.)),
        //     ..default()
        // },
        Mesh3d(meshes.add(Mesh::from(sphere_mesh))),
        MeshMaterial3d(rock_material(asset_server, images, materials)),
        Transform::from_xyz(0. + pos_offset, SPAWN_Y, 0.)
            .with_rotation(Quat::from_rotation_x(-PI / 1.)),
        Rock,
    )
}

use rand::Rng;
use std::f32::consts::PI;

use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        component::Component,
        query::Without,
        system::{Query, Res, ResMut},
    },
    math::{primitives::Plane3d, EulerRot, Quat, Vec3, Vec3Swizzles},
    pbr::{AlphaMode, PbrBundle, StandardMaterial},
    prelude::default,
    render::mesh::{Mesh, Meshable},
    transform::components::Transform,
};

use crate::resources::PlayerCharacter;

#[derive(Component)]
pub struct Cloud;

const CLOUD_TEXTURES: [&str; 2] = ["textures/cloud-a.png", "textures/cloud-b.png"];

pub fn create_cloud<'a>(
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>, // do the &mut like in another func (wheel/camera)
    x: f32,
    z: f32,
    y: f32,
    rot_y: f32,
) -> PbrBundle {
    let mut rng = rand::thread_rng();
    // let x = rand::random::<f32>();
    let path_index = rng.gen_range(0..CLOUD_TEXTURES.len() - 1);
    let texture_path = CLOUD_TEXTURES[path_index];
    let texture_cloud = asset_server.load(texture_path);
    let mut cloud_mesh: Mesh = Plane3d::default().mesh().size(10., 10.).build();

    let rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, -PI / 2.);
    cloud_mesh.rotate_by(rotation);
    // Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, -PI / 3.5))

    // TODO: Backface visibility of this plane

    PbrBundle {
        mesh: meshes.add(cloud_mesh),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_cloud.clone()),
            alpha_mode: AlphaMode::Add,
            fog_enabled: false,
            ..default()
        }),
        transform: Transform::from_xyz(x, y, z), //.looking_at(Vec3::new(0.0, 1.0, -0.0), Vec3::Y),
        ..default()
    }
}

pub fn update_cloud_orientations(
    mut q_char: Query<(&PlayerCharacter, &mut Transform)>,
    mut q_clouds: Query<(&Cloud, &mut Transform), Without<PlayerCharacter>>,
) {
    let (_, t_char) = q_char.get_single_mut().unwrap();
    for (_, mut t_cloud) in &mut q_clouds {
        t_cloud.look_at(t_char.translation.xyz(), Vec3::Y);
    }
}

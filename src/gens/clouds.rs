use std::f32::consts::PI;

use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Res, ResMut},
    math::{primitives::Plane3d, EulerRot, Quat, Vec3},
    pbr::{AlphaMode, PbrBundle, StandardMaterial},
    prelude::default,
    render::mesh::{Mesh, Meshable},
    transform::components::Transform,
};

pub fn create_cloud<'a>(
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>, // do the &mut like in another func (wheel/camera)
    x: f32,
    z: f32,
    y: f32,
    rot_y: f32,
) -> PbrBundle {
    let texture_cloud = asset_server.load("textures/cloud-a.png");
    let mut cloud_mesh: Mesh = Plane3d::default().mesh().size(10., 10.).build();

    let rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, -PI / 2.);
    cloud_mesh.rotate_by(rotation);
    // Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, -PI / 3.5))

    // TODO: Backface visibility of this plane

    PbrBundle {
        mesh: meshes.add(cloud_mesh),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_cloud.clone()),
            alpha_mode: AlphaMode::Blend,
            fog_enabled: false,
            ..default()
        }),
        transform: Transform::from_xyz(x, y, z), //.looking_at(Vec3::new(0.0, 1.0, -0.0), Vec3::Y),
        ..default()
    }
}

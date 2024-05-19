use rand::Rng;
use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    ecs::{
        component::Component,
        query::Without,
        system::{Commands, Query, Res, ResMut},
    },
    math::{primitives::Plane3d, EulerRot, Quat},
    pbr::{AlphaMode, NotShadowCaster, PbrBundle, StandardMaterial},
    prelude::default,
    render::mesh::{Mesh, Meshable},
    transform::components::Transform,
};

use crate::{movement::orientation::look_at_on_y, resources::PlayerCamera};

#[derive(Component)]
pub struct Cloud;

// TODO: Move cloud setup and update to a cloud-specific plugin or system
pub fn setup_clouds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        create_cloud(
            &asset_server,
            &mut meshes,
            &mut materials,
            // should use a position coordinates struct for this
            10.,
            40.,
            15.,
            20.,
        ),
        NotShadowCaster,
        Cloud,
    ));

    commands.spawn((
        create_cloud(
            &asset_server,
            &mut meshes,
            &mut materials,
            // should use a position coordinates struct for this
            12.,
            38.,
            15.5,
            20.,
        ),
        NotShadowCaster,
        Cloud,
    ));

    commands.spawn((
        create_cloud(
            &asset_server,
            &mut meshes,
            &mut materials,
            // should use a position coordinates struct for this
            15.,
            50.,
            13.,
            20.,
        ),
        NotShadowCaster,
        Cloud,
    ));

    commands.spawn((
        create_cloud(
            &asset_server,
            &mut meshes,
            &mut materials,
            // should use a position coordinates struct for this
            -15.,
            -50.,
            20.,
            20.,
        ),
        NotShadowCaster,
        Cloud,
    ));

    commands.spawn((
        create_cloud(
            &asset_server,
            &mut meshes,
            &mut materials,
            // should use a position coordinates struct for this
            20.,
            43.,
            15.5,
            20.,
        ),
        NotShadowCaster,
        Cloud,
    ));
}

const CLOUD_TEXTURES: [&str; 3] = [
    "textures/cloud-a.png",
    "textures/cloud-b.png",
    "textures/cloud-c.png",
];

pub fn create_cloud<'a>(
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>, // do the &mut like in another func (wheel/camera)
    x: f32,
    z: f32,
    y: f32,
    _rot_y: f32,
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

pub fn update_clouds(
    q_cam: Query<(&PlayerCamera, &Transform)>,
    mut q_clouds: Query<(&Cloud, &mut Transform), Without<PlayerCamera>>,
) {
    update_cloud_positions(&mut q_clouds);
    update_cloud_orientations(&q_cam, &mut q_clouds);
}

pub fn update_cloud_positions(
    q_clouds: &mut Query<(&Cloud, &mut Transform), Without<PlayerCamera>>,
) {
    // Clouds are affected by wind and drift
    for (_, mut t_cloud) in q_clouds {
        t_cloud.translation.x -= 0.009;
        t_cloud.translation.z -= 0.005;
    }
}

pub fn update_cloud_orientations(
    q_cam: &Query<(&PlayerCamera, &Transform)>,
    q_clouds: &mut Query<(&Cloud, &mut Transform), Without<PlayerCamera>>,
) {
    let (_, t_cam) = q_cam.single();
    for (_, mut t_cloud) in q_clouds {
        look_at_on_y(&mut t_cloud, &t_cam);
    }
}

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_clouds);
        app.add_systems(Update, update_clouds);
    }
}

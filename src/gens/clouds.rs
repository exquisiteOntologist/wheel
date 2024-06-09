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
    math::{
        primitives::{Direction3d, Plane3d},
        EulerRot, Quat, Vec3,
    },
    pbr::{AlphaMode, NotShadowCaster, PbrBundle, StandardMaterial},
    prelude::default,
    reflect::Reflect,
    render::mesh::{Mesh, Meshable},
    transform::components::Transform,
};

use crate::{movement::orientation::look_at_on_y, resources::PlayerCamera};

const CLOUD_BUDGET: u32 = 30;
const CLOUD_REGEN_DISTANCE: f32 = 300.;

#[derive(Component)]
pub struct Cloud;

pub fn setup_clouds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 0..CLOUD_BUDGET {
        // we will update the translation at a later stage
        commands.spawn((
            create_cloud(
                &asset_server,
                &mut meshes,
                &mut materials,
                10000.,
                10000.,
                5.,
            ),
            NotShadowCaster,
            Cloud,
        ));
    }

    // commands.spawn((
    //     create_cloud(&asset_server, &mut meshes, &mut materials, 5., 40., 13.),
    //     NotShadowCaster,
    //     Cloud,
    // ));

    // commands.spawn((
    //     create_cloud(&asset_server, &mut meshes, &mut materials, 45., 38., 12.),
    //     NotShadowCaster,
    //     Cloud,
    // ));

    // commands.spawn((
    //     create_cloud(&asset_server, &mut meshes, &mut materials, 30., 50., 15.),
    //     NotShadowCaster,
    //     Cloud,
    // ));

    // commands.spawn((
    //     create_cloud(&asset_server, &mut meshes, &mut materials, -30., -50., 12.),
    //     NotShadowCaster,
    //     Cloud,
    // ));

    // commands.spawn((
    //     create_cloud(&asset_server, &mut meshes, &mut materials, -12., -35., 15.),
    //     NotShadowCaster,
    //     Cloud,
    // ));

    // commands.spawn((
    //     create_cloud(&asset_server, &mut meshes, &mut materials, 15., 43., 12.),
    //     NotShadowCaster,
    //     Cloud,
    // ));
}

const CLOUD_TEXTURES: [&str; 5] = [
    "textures/cloud-a.png",
    "textures/cloud-b.png",
    "textures/cloud-c.png",
    "textures/cloud-d.png",
    "textures/cloud-e.png",
];

pub fn create_cloud<'a>(
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>, // do the &mut like in another func (wheel/camera)
    x: f32,
    z: f32,
    y: f32,
) -> PbrBundle {
    let mut rng = rand::thread_rng();
    // let x = rand::random::<f32>();
    let path_index = rng.gen_range(0..CLOUD_TEXTURES.len() - 0);
    let texture_path = CLOUD_TEXTURES[path_index];
    let texture_cloud = asset_server.load(texture_path);

    let ratio = if path_index == 3 {
        (17., 10.)
    } else if path_index == 4 {
        (25., 10.)
    } else {
        (10., 10.)
    };
    let mut cloud_mesh: Mesh = Plane3d::default().mesh().size(ratio.0, ratio.1).build();

    let rotation = Quat::from_euler(EulerRot::ZYX, 0., PI / 1., PI / 2.);
    cloud_mesh.rotate_by(rotation);

    PbrBundle {
        mesh: meshes.add(cloud_mesh),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_cloud),
            alpha_mode: AlphaMode::Add,
            fog_enabled: false,
            double_sided: true,
            metallic: 0.1,
            cull_mode: None,
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
    distribute_clouds(&q_cam, &mut q_clouds);
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

/// Distribute clouds relative to camera,
/// redistributing the same clouds when the camera moves.
/// [ TL, , ]
/// [ , C, ]
/// [ , , BR]
pub fn distribute_clouds(
    q_cam: &Query<(&PlayerCamera, &Transform)>,
    q_clouds: &mut Query<(&Cloud, &mut Transform), Without<PlayerCamera>>,
) {
    let mut rng = rand::thread_rng();
    let farthest_cloud = CLOUD_REGEN_DISTANCE - 10.;
    let (_, t_cam) = q_cam.get_single().unwrap();
    let mut i_c = 0.;
    for (_, mut t_cloud) in &mut q_clouds.iter_mut() {
        i_c += 1.;
        let distance = t_cloud.translation.distance(t_cam.translation);
        if distance < CLOUD_REGEN_DISTANCE {
            // note that we could reverse order and break if they get increasingly distant
            continue;
        }

        let elevation: f32 = 20. + rng.gen_range(0..15) as f32;
        t_cloud.translation.x = 0.;
        t_cloud.translation.z = 0.;
        t_cloud.translation.y = 0.;
        t_cloud.rotation.y = 0.;
        t_cloud.rotation.x = 0.;
        t_cloud.rotation.z = 0.;
        let rot_range: f32 = rng.gen_range(-10..10) as f32 / 10.;
        t_cloud.rotate_local_y(rot_range);
        let rotation = t_cloud.rotation;
        let dir = Direction3d::new(rotation * -Vec3::X).unwrap();
        let dist = (farthest_cloud / CLOUD_BUDGET as f32) * (i_c + 1.);
        t_cloud.translation = t_cam.translation + dir * dist;
        t_cloud.rotate_around(t_cam.translation, rotation);
        t_cloud.translate_around(t_cam.translation, rotation);
        t_cloud.translation.y = elevation;

        // println!(
        //     "Cloud {:1} {:2} {:3} {:4} {:5}",
        //     i_c, t_cloud.translation.x, dist, rotation.y, rot_range
        // );
    }
}

pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_clouds);
        app.add_systems(Update, update_clouds);
    }
}

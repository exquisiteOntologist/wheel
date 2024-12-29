use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    pbr::StandardMaterial,
    prelude::{default, Mesh3d},
    render::{
        alpha::AlphaMode,
        mesh::{Mesh, PlaneMeshBuilder, VertexAttributeValues},
        render_resource::Face,
    },
    transform::components::Transform,
};
use bevy_pbr::MeshMaterial3d;
use bevy_rapier3d::{
    geometry::{Collider, ComputedColliderShape},
    prelude::TriMeshFlags,
};

use crate::{
    components::characters::player::resources::PlayerCharacter,
    constants::{
        COLOR_SAND, PLANE_SIZE, SIZE_NO_PLAYER, SUBDIVISIONS_LEVEL_1, SUBDIVISIONS_LEVEL_2,
        TEXTURE_SCALE, TILE_WIDTH,
    },
    utils::perlin::{self, sample_terrain_height},
};

use super::{
    colours::get_terrain_color,
    resources::{MainTerrain, Terrain},
    texture::get_terrain_texture,
};

/// Generate terrain mesh
pub fn generate_terrain_mesh(x: f32, z: f32, size: f32, subdivisions: u32) -> Mesh {
    let num_vertices: usize =
        (SUBDIVISIONS_LEVEL_1 as usize + 2) * (SUBDIVISIONS_LEVEL_1 as usize + 2);
    let height_map = perlin::terrain_perlin();
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
    let mut vertex_colors: Vec<[f32; 4]> = Vec::with_capacity(num_vertices);
    let mesh_builder =
        PlaneMeshBuilder::from_size(Vec2 { x: size, y: size }).subdivisions(subdivisions);
    // let mut mesh: Mesh = Prelude::Mesh { size, subdivisions }.into();
    let mut mesh: Mesh = mesh_builder.into();
    // get positions
    let pos_attr = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
    let VertexAttributeValues::Float32x3(pos_attr) = pos_attr else {
        panic!("Unexpected vertex format, expected Float32x3");
    };
    // modify y with height sampling
    for i in 0..pos_attr.len() {
        let pos = pos_attr.get_mut(i).unwrap();
        pos[1] = sample_terrain_height(&height_map, x + pos[0], z + pos[2]);
        uvs.push([
            pos[0] / (TILE_WIDTH as f32 * TEXTURE_SCALE),
            pos[2] / (TILE_WIDTH as f32 * TEXTURE_SCALE),
        ]);
        vertex_colors.push(get_terrain_color(pos[1]));
    }

    // println!("===");
    // for c in &vertex_colors {
    //     println!("colour {:1} {:2} {:3} {:4}", c[0], c[1], c[2], c[3]);
    // }
    // println!("===");

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let _ = mesh.generate_tangents();

    mesh
}

fn spawn_terrain_chunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    x: f32,
    z: f32,
    contains_player: bool,
    size: f32,
    subdivisions: u32,
) -> Entity {
    let mesh = generate_terrain_mesh(x, z, size, subdivisions);
    let texture_ground = get_terrain_texture(&asset_server, &COLOR_SAND);

    // let sampler_desc = ImageSamplerDescriptor {
    //     address_mode_u: ImageAddressMode::Repeat,
    //     address_mode_v: ImageAddressMode::Repeat,
    //     ..default()
    // };
    // let settings = move |s: &mut ImageLoaderSettings| {
    //     s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    // };
    // let texture_handle = asset_server.load_with_settings("terrain/rocky_soil.png", settings.clone());
    // let normal_handle = asset_server.load_with_settings("terrain/rocky_soil_normal.png", settings);
    let terrain_material = StandardMaterial {
        // base_color: if contains_player { Color::WHITE } else { Color::WHITE }, // use to see difference in terrain chunks
        // base_color_texture: Some(texture_handle.clone()),
        // normal_map_texture: Some(normal_handle.clone()),
        alpha_mode: AlphaMode::Opaque,
        double_sided: false,
        perceptual_roughness: 1., // 0.8,
        reflectance: 0.3,         // 0.3,
        cull_mode: Some(Face::Back),
        flip_normal_map_y: true,
        base_color_texture: Some(texture_ground.clone()),
        fog_enabled: true,
        ..default()
    };

    let collider_shape = ComputedColliderShape::TriMesh(TriMeshFlags::default());

    let mut binding = commands.spawn((
        Mesh3d(meshes.add(mesh.clone())),
        MeshMaterial3d(materials.add(terrain_material)),
        Transform::from_xyz(x, 0., z),
    ));
    let parent_terrain = binding
        .insert(Terrain)
        .insert(Collider::from_bevy_mesh(&mesh, &collider_shape).unwrap());
    if contains_player {
        parent_terrain.insert(MainTerrain);
    }
    parent_terrain.id()
}

fn regenerate_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    main_terrain: &mut Query<(Entity, &mut Transform, &Mesh3d), (With<Terrain>, With<MainTerrain>)>,
    distant_terrain: &mut Query<
        (Entity, &mut Transform, &Mesh3d),
        (With<Terrain>, Without<MainTerrain>),
    >,
    delta: Vec3,
) {
    let collider_shape = ComputedColliderShape::TriMesh(TriMeshFlags::default());

    // shift over and regen terrain
    for (ent, mut trans, mh) in main_terrain.iter_mut() {
        trans.translation = trans.translation + delta;
        trans.translation.y = 0.;
        let mesh = meshes.get_mut(mh).unwrap();
        let new_mesh = &mut generate_terrain_mesh(
            trans.translation.x,
            trans.translation.z,
            PLANE_SIZE,
            SUBDIVISIONS_LEVEL_1,
        );
        *mesh = new_mesh.clone();
        commands
            .get_entity(ent)
            .unwrap()
            .insert(Collider::from_bevy_mesh(&mesh, &collider_shape).unwrap());
    }
    for (ent, mut trans, mh) in distant_terrain.iter_mut() {
        trans.translation = trans.translation + delta;
        trans.translation.y = 0.;
        let mesh = meshes.get_mut(mh).unwrap();
        let new_mesh = &mut generate_terrain_mesh(
            trans.translation.x,
            trans.translation.z,
            PLANE_SIZE,
            SUBDIVISIONS_LEVEL_2,
        );
        *mesh = new_mesh.clone();
        // commands.get_entity(pl_ent).unwrap().insert(Collider::from_bevy_mesh(&mesh, &collider_shape).unwrap()); // no need for collider here atm
    }
}

/// set up a simple 3D scene
pub fn update_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut main_terrain: Query<(Entity, &mut Transform, &Mesh3d), (With<Terrain>, With<MainTerrain>)>,
    mut distant_terrain: Query<
        (Entity, &mut Transform, &Mesh3d),
        (With<Terrain>, Without<MainTerrain>),
    >,
    // player: Query<&Transform, (With<player::Player>, Without<Terrain>)>,
    q_char: Query<&Transform, (With<PlayerCharacter>, Without<Terrain>)>,
) {
    if meshes.is_empty() {
        return;
    }
    if main_terrain.is_empty() {
        // scene start
        // spawn chunk at player
        let player_trans = if let Ok(char) = q_char.get_single() {
            char.translation
        } else {
            // if no player return (assuming player will exist later on)
            return;
        };
        spawn_terrain_chunk(
            &mut commands,
            &mut meshes,
            &mut materials,
            &asset_server,
            0.,
            0.,
            true,
            PLANE_SIZE,
            SUBDIVISIONS_LEVEL_1,
        );
        // spawn chunks without player in them
        for (dx, dz) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            // continue;
            //
            // 8FPS improvement not generating these other chunks
            let calc_dx = dx as f32 * (PLANE_SIZE / 2. + SIZE_NO_PLAYER / 2.);
            let calc_dz = dz as f32 * (PLANE_SIZE / 2. + SIZE_NO_PLAYER / 2.);
            spawn_terrain_chunk(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                player_trans.x + calc_dx,
                player_trans.z + calc_dz,
                false,
                SIZE_NO_PLAYER,
                SUBDIVISIONS_LEVEL_2,
            );
        }
        // spawn_water_plane(&mut commands, &mut meshes, &mut materials, &asset_server);
    } else {
        // main update logic
        if let Ok(terrain) = main_terrain.get_single_mut() {
            let (terrain_ent, terrain_trans, terrain_mesh) = terrain;
            let player_trans = q_char.get_single().unwrap();
            let mut delta: Option<Vec3> = None;

            // determine player triggering terrain refresh
            if (player_trans.translation.x - terrain_trans.translation.x).abs() > PLANE_SIZE / 4.
                || (player_trans.translation.z - terrain_trans.translation.z).abs()
                    > PLANE_SIZE / 4.
            {
                delta = Some(player_trans.translation - terrain_trans.translation);
            }

            // if they have, regenerate the terrain
            if let Some(delta) = delta {
                println!("Player has triggered terrain regeneration");
                regenerate_terrain(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &asset_server,
                    &mut main_terrain,
                    &mut distant_terrain,
                    delta,
                );
            }
        }
    }
}

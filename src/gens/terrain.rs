use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    math::Vec3,
    pbr::{AlphaMode, PbrBundle, StandardMaterial},
    prelude::default,
    render::{
        mesh::{Mesh, VertexAttributeValues},
        render_resource::Face,
        texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    },
    transform::components::Transform,
};
use bevy_rapier3d::geometry::{Collider, ComputedColliderShape};

use crate::{
    constants::{
        COLOR_PEAKS, COLOR_SAND, COLOR_TEMPERATE, HEIGHT_PEAKS, HEIGHT_SAND, HEIGHT_TEMPERATE_END,
        HEIGHT_TEMPERATE_START, PLANE_SIZE, SIZE_NO_PLAYER, SUBDIVISIONS_LEVEL_1,
        SUBDIVISIONS_LEVEL_2, TEXTURE_SCALE, TILE_WIDTH,
    },
    resources::PlayerCharacter,
    utils::perlin::{self, sample_terrain_height},
};

// struct for marking terrain
#[derive(Component)]
pub struct Terrain;

#[derive(Component)]
pub struct MainTerrain;

fn get_terrain_color(y: f32) -> [f32; 4] {
    if y < HEIGHT_SAND {
        COLOR_SAND
    } else if y > HEIGHT_PEAKS {
        COLOR_PEAKS
    } else if y < HEIGHT_TEMPERATE_START {
        terrain_color_gradient(
            (y - HEIGHT_SAND) / (HEIGHT_TEMPERATE_START - HEIGHT_SAND),
            COLOR_SAND,
            COLOR_TEMPERATE,
        )
    } else if y < HEIGHT_TEMPERATE_END {
        COLOR_TEMPERATE
    } else {
        terrain_color_gradient(
            (y - HEIGHT_TEMPERATE_END) / (HEIGHT_PEAKS - HEIGHT_TEMPERATE_END),
            COLOR_TEMPERATE,
            COLOR_PEAKS,
        )
    }
}

fn terrain_color_gradient(ratio: f32, rgba1: [f32; 4], rgba2: [f32; 4]) -> [f32; 4] {
    let [r1, g1, b1, a1] = rgba1;
    let [r2, g2, b2, a2] = rgba2;

    [
        r1 + (r2 - r1) * (ratio),
        g1 + (g2 - g1) * (ratio),
        b1 + (b2 - b1) * (ratio),
        a1 + (a2 - a1) * (ratio),
    ]
}

/// Generate terrain mesh
pub fn generate_terrain_mesh(x: f32, z: f32, size: f32, subdivisions: u32) -> Mesh {
    let num_vertices: usize =
        (SUBDIVISIONS_LEVEL_1 as usize + 2) * (SUBDIVISIONS_LEVEL_1 as usize + 2);
    let height_map = perlin::terrain_perlin();
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
    let mut vertex_colors: Vec<[f32; 4]> = Vec::with_capacity(num_vertices);
    let mut mesh: Mesh = bevy::prelude::shape::Plane { size, subdivisions }.into();
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

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    _ = mesh.generate_tangents();

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
    println!("generated terrain mesh");

    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    };
    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    // let texture_handle = asset_server.load_with_settings("terrain/rocky_soil.png", settings.clone());
    // let normal_handle = asset_server.load_with_settings("terrain/rocky_soil_normal.png", settings);
    let terrain_material = StandardMaterial {
        // base_color: if contains_player { Color::WHITE } else { Color::WHITE }, // use to see difference in terrain chunks
        // base_color_texture: Some(texture_handle.clone()),
        // normal_map_texture: Some(normal_handle.clone()),
        alpha_mode: AlphaMode::Opaque,
        double_sided: true,
        perceptual_roughness: 1.0,
        reflectance: 0.4,
        cull_mode: Some(Face::Back),
        flip_normal_map_y: true,
        ..default()
    };

    // terrain
    let collider_shape = ComputedColliderShape::TriMesh;

    let mut binding = commands.spawn(PbrBundle {
        mesh: meshes.add(mesh.clone()),
        material: materials.add(terrain_material),
        transform: Transform::from_xyz(x, 0., z),
        ..default()
    });
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
    main_terrain: &mut Query<
        (Entity, &mut Transform, &Handle<Mesh>),
        (With<Terrain>, With<MainTerrain>),
    >,
    distant_terrain: &mut Query<
        (Entity, &mut Transform, &Handle<Mesh>),
        (With<Terrain>, Without<MainTerrain>),
    >,
    delta: Vec3,
) {
    let collider_shape = ComputedColliderShape::TriMesh;

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
    mut main_terrain: Query<
        (Entity, &mut Transform, &Handle<Mesh>),
        (With<Terrain>, With<MainTerrain>),
    >,
    mut distant_terrain: Query<
        (Entity, &mut Transform, &Handle<Mesh>),
        (With<Terrain>, Without<MainTerrain>),
    >,
    // player: Query<&Transform, (With<player::Player>, Without<Terrain>)>,
    q_char: Query<(&PlayerCharacter, &Transform), (With<PlayerCharacter>, Without<Terrain>)>,
) {
    let Ok((_, t_player)) = q_char.get_single() else {
        eprintln!("There was no player in the query");
        return;
    };
    // let player_trans = t_player.translation;
    if main_terrain.is_empty() {
        println!("Main terrain empty");
        // scene start
        // spawn chunk at player
        // let player_trans = q_char.get_single().unwrap().translation;
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
        println!("Spawning other chunks");
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
            println!("Spawning trunk {:1} {:2}", dx, dz);
            let calc_dx = dx as f32 * (PLANE_SIZE / 2. + SIZE_NO_PLAYER / 2.);
            let calc_dz = dz as f32 * (PLANE_SIZE / 2. + SIZE_NO_PLAYER / 2.);
            spawn_terrain_chunk(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                t_player.translation.x + calc_dx,
                t_player.translation.z + calc_dz,
                false,
                SIZE_NO_PLAYER,
                SUBDIVISIONS_LEVEL_2,
            );
        }
        // spawn_water_plane(&mut commands, &mut meshes, &mut materials, &asset_server);
    } else {
        println!("Main terrain NOT empty");
        // main update logic
        if let Ok(terrain) = main_terrain.get_single_mut() {
            let (terrain_ent, terrain_trans, terrain_mesh) = terrain;
            // let player_trans = player.get_single().unwrap();
            let mut delta: Option<Vec3> = None;

            // determine player triggering terrain refresh
            if (t_player.translation.x - terrain_trans.translation.x).abs() > PLANE_SIZE / 4.
                || (t_player.translation.z - terrain_trans.translation.z).abs() > PLANE_SIZE / 4.
            {
                delta = Some(t_player.translation - terrain_trans.translation);
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

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, update_terrain);
        // app.add_systems(Update, update_terrain);
    }
}

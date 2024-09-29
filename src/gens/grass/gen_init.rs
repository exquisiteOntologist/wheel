use bevy::pbr::ExtendedMaterial;

use bevy::render::render_asset::RenderAssetUsages;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use noise::NoiseFn;

use rand::{thread_rng, Rng};

use crate::constants::{HEIGHT_TEMPERATE_END, HEIGHT_TEMPERATE_START};
use crate::utils::perlin::{self, sample_terrain_height};

use super::constants::{
    GRASS_BASE_COLOR_2, GRASS_BASE_COLOR_3, GRASS_HEIGHT, GRASS_HEIGHT_VARIATION_FACTOR,
    GRASS_OFFSET,
};
use super::gen_geometry::generate_grass_geometry;
use super::gen_verts::generate_single_blade_verts;
use super::materials::{grass_material, GrassMaterialExtension};
use super::resources::{Grass, GrassData};

/// Generate a grass mesh.
/// This can be used to generate new grass,
/// when initializing the grass grid,
/// or when there is none in part of the grid.
pub fn generate_grass_mesh(
    spawn_x: f32,
    spawn_z: f32,
    density: u32,
    tile_size: f32,
) -> (Mesh, GrassData) {
    let mut grass_offsets = vec![];
    let mut rng = thread_rng();
    let asset_usage = RenderAssetUsages::RENDER_WORLD; // | RenderAssetUsages::MAIN_WORLD;
                                                       // let asset_usage = RenderAssetUsages::RENDER_WORLD;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, asset_usage);
    let mut all_verts: Vec<Vec3> = vec![];
    let mut all_indices: Vec<u32> = vec![];
    let mut all_colors: Vec<[f32; 4]> = vec![];
    let mut blade_number = 0;
    let height_perlin = perlin::grass_perlin();
    let terrain_perlin = perlin::terrain_perlin();
    let start_x = -tile_size / 2.;
    let start_z = -tile_size / 2.;
    for i in 0..density {
        let x = start_x + i as f32 * tile_size / density as f32;
        for j in 0..density {
            let z = start_z + j as f32 * tile_size / density as f32;
            let rand1 = if GRASS_OFFSET != 0.0 {
                rng.gen_range(-GRASS_OFFSET..GRASS_OFFSET)
            } else {
                0.0
            };
            let rand2 = if GRASS_OFFSET != 0.0 {
                rng.gen_range(-GRASS_OFFSET..GRASS_OFFSET)
            } else {
                0.0
            };
            let x_offset = x + rand1;
            let z_offset = z + rand2;
            // this calculation is based on the perlin used to generate the geometry, not the geometry itself
            let y = sample_terrain_height(&terrain_perlin, spawn_x + x_offset, spawn_z + z_offset)
                - 0.2; // minus small amount to avoid floating
            let blade_height = GRASS_HEIGHT
                + (height_perlin.get([(spawn_x + x_offset) as f64, (spawn_z + z_offset) as f64])
                    as f32
                    * GRASS_HEIGHT_VARIATION_FACTOR);
            if y > HEIGHT_TEMPERATE_START && y < HEIGHT_TEMPERATE_END {
                let (mut verts, mut indices) =
                    generate_single_blade_verts(x_offset, y, z_offset, blade_number, blade_height);
                for v in &verts {
                    grass_offsets.push([spawn_x + x_offset, y, spawn_z + z_offset]);
                    // let r_color_shift = (terrain_perlin.get([
                    //     (spawn_x + x_offset) as f64 / 100.,
                    //     (spawn_z + z_offset) as f64 / 100.,
                    // ]) * 0.001) as f32;
                    // let mut color =
                    //     color_gradient_y_based(v.y - y, GRASS_BASE_COLOR_3, GRASS_SECOND_COLOR_3);
                    // color[1] += r_color_shift;
                    // let b_color_shift = r_color_shift;
                    // color[2] += b_color_shift;
                    let color = if (i + j) % 2 == 0 {
                        // darker, more blue
                        GRASS_BASE_COLOR_3
                    } else {
                        // lighter, more green
                        GRASS_BASE_COLOR_2
                    };
                    all_colors.push(color);
                }
                all_verts.append(&mut verts);
                all_indices.append(&mut indices);
                blade_number += 1;
            }
        }
    }

    generate_grass_geometry(
        &all_verts,
        all_indices,
        &mut mesh,
        &grass_offsets,
        all_colors,
    );

    (
        mesh,
        GrassData {
            initial_vertices: all_verts,
            initial_positions: grass_offsets,
        },
    )
}

/// Generate Grass.
/// This generates the material and mesh,
/// typically for when there is none yet.
/// The material and mesh will next need to be spawned.
pub fn generate_grass(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ExtendedMaterial<StandardMaterial, GrassMaterialExtension>>>,
    spawn_x: f32,
    spawn_z: f32,
    density: u32,
    tile_size: f32,
) -> (
    MaterialMeshBundle<ExtendedMaterial<StandardMaterial, GrassMaterialExtension>>,
    Grass,
    GrassData,
) {
    let (mesh, grass_data) = generate_grass_mesh(spawn_x, spawn_z, density, tile_size);

    let grass_material_ext = GrassMaterialExtension {};

    let grass_material_std = grass_material();

    let grass_material = ExtendedMaterial {
        base: grass_material_std,
        extension: grass_material_ext,
    };

    let bundle = MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: materials.add(grass_material),
        transform: Transform::from_xyz(spawn_x, 0., spawn_z),
        ..default()
    };

    (bundle, Grass {}, grass_data)
}

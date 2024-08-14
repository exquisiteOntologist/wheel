use bevy::{prelude::*, render::mesh};

use super::constants::{ATTRIBUTE_BASE_Y, ATTRIBUTE_STARTING_POSITION, ATTRIBUTE_WORLD_POSITION};

pub fn generate_grass_geometry(
    verts: &Vec<Vec3>,
    vec_indices: Vec<u32>,
    mesh: &mut Mesh,
    grass_offsets: &Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
) {
    let indices = mesh::Indices::U32(vec_indices);

    let vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = verts
        .iter()
        .map(|v| (v.to_array(), [0., 1., 0.], [0.0, 0.0]))
        .collect();

    let mut positions = Vec::with_capacity(verts.capacity());
    let mut normals = Vec::with_capacity(verts.capacity());
    let bases: Vec<f32> = grass_offsets.iter().map(|x| x[1]).collect();
    // let mut uvs: Vec<[f32; 2]> = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        // uvs.push(*uv);
    }

    mesh.insert_indices(indices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    // mesh.generate_tangents().unwrap();
    mesh.insert_attribute(ATTRIBUTE_BASE_Y, bases);
    mesh.insert_attribute(ATTRIBUTE_STARTING_POSITION, positions);
    mesh.insert_attribute(ATTRIBUTE_WORLD_POSITION, grass_offsets.clone());
}

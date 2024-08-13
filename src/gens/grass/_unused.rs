fn _grass_data_to_base_data(
    grass_data: Vec<[f32; 3]>,
) -> [f32; (GRASS_BLADE_VERTICES * (NUM_GRASS_1 * NUM_GRASS_1 + 2)) as usize] {
    let mut arr = [0.; (GRASS_BLADE_VERTICES * (NUM_GRASS_1 * NUM_GRASS_1 + 2)) as usize];
    for (i, v) in grass_data.iter().enumerate() {
        arr[i] = v[1];
    }
    arr
}

fn _apply_wind(
    mesh: &mut Mesh,
    grass: &GrassData,
    perlin: &PerlinNoiseEntity,
    time: f64,
    player_xz: Vec2,
) {
    let wind_perlin = perlin.wind;
    let pos_attr = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
    let VertexAttributeValues::Float32x3(pos_attr) = pos_attr else {
        panic!("Unexpected vertex format, expected Float32x3");
    };
    // for now modify x,z pos. Ideally apply curve instead
    for i in 0..pos_attr.len() {
        let pos = pos_attr.get_mut(i).unwrap(); // current vertex positions
        let initial = grass.initial_vertices.get(i).unwrap(); // initial vertex positions
        let grass_pos = grass.initial_positions.get(i).unwrap(); // initial grass positions

        let [x, y, z] = grass_pos;

        let relative_vertex_height = pos[1] - y;

        let curve_amount = WIND_STRENGTH
            * ((WIND_SIM_DISTANCE - player_xz.distance(Vec2::new(*x, *z))) / WIND_SIM_DISTANCE)
                .powi(2)
            * (sample_noise(&wind_perlin, *x, *z, time)
                * (relative_vertex_height.powf(CURVE_POWER) / GRASS_HEIGHT.powf(CURVE_POWER)));
        pos[0] = initial.x + curve_amount;
        pos[2] = initial.z + curve_amount;
    }
}

fn sample_noise(perlin: &Perlin, x: f32, z: f32, time: f64) -> f32 {
    WIND_LEAN
        + perlin.get([
            WIND_SPEED * time + (x as f64 / WIND_CONSISTENCY),
            WIND_SPEED * time + (z as f64 / WIND_CONSISTENCY),
        ]) as f32
}

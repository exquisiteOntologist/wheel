use bevy::prelude::*;

use cgmath::num_traits::ToPrimitive;
use rand::{thread_rng, Rng};

use super::constants::{
    GRASS_HEIGHT, GRASS_STRAIGHTNESS, GRASS_STRAIGHTNESS_MAX, GRASS_STRAIGHTNESS_MIN, GRASS_WIDTH,
};

pub fn generate_single_blade_verts(
    x: f32,
    y: f32,
    z: f32,
    blade_number: u32,
    blade_height: f32,
) -> (Vec<Vec3>, Vec<u32>) {
    if blade_height < (GRASS_HEIGHT * 0.25) {
        return (Vec::new(), Vec::new());
    }

    // For grass with 7 vertices, uncomment t3-6, and uncomment indices
    // vertex transforms
    let t1 = Transform::from_xyz(x, y, z);
    let t2 = Transform::from_xyz(x + GRASS_WIDTH, y, z);
    // Optional vertice range START
    // let t3 = Transform::from_xyz(x + 0.05, y + blade_height / 3.0, z);
    // let t35 = Transform::from_xyz(x + GRASS_WIDTH - 0.05, y + blade_height / 3.0, z);
    // let t4 = Transform::from_xyz(x + GRASS_WIDTH, y + blade_height / 3.0, z);
    // let t5 = Transform::from_xyz(x, y + 2.0 * blade_height / 3.0, z);
    // let t6 = Transform::from_xyz(x + GRASS_WIDTH, y + 2.0 * blade_height / 3.0, z);
    // Optional vertice range END
    let t7 = Transform::from_xyz(x + (GRASS_WIDTH / 2.0), y + blade_height, z);

    // vec![t1, t2, t3, t4, t5, t6, t7]
    // let mut transforms = vec![t1, t2, t7];

    // let mut transforms = vec![t1, t2, t3, t4, t5, t6, t7];
    // let mut transforms = vec![t1,t2,t5,t6,t7];
    let mut transforms = vec![t1, t2, t7];
    let blade_number_shift = blade_number * transforms.len() as u32;

    // // physical randomization of grass blades
    // rotate grass randomly around y
    apply_y_rotation(&mut transforms, x, y, z);

    // curve the grass all one way
    apply_curve(&mut transforms, x, y, z);

    // rotate grass again
    apply_y_rotation(&mut transforms, x, y, z);

    let verts: Vec<Vec3> = transforms.iter().map(|t| t.translation).collect();

    let indices: Vec<u32> = vec![
        blade_number_shift + 0,
        blade_number_shift + 1,
        blade_number_shift + 2,
        // Comment out all the shifts below if downscaling
        // blade_number_shift + 2,
        // blade_number_shift + 1,
        // blade_number_shift + 3,
        // blade_number_shift + 2,
        // blade_number_shift + 3,
        // blade_number_shift + 4,
        // blade_number_shift + 4,
        // blade_number_shift + 3,
        // blade_number_shift + 5,
        // blade_number_shift + 4,
        // blade_number_shift + 5,
        // blade_number_shift + 6,
    ];

    (verts, indices)
}

fn apply_y_rotation(transforms: &mut Vec<Transform>, x: f32, y: f32, z: f32) {
    let y_rotation_point = Vec3::new(x, y, z);
    let rand_rotation = (thread_rng().gen_range(0..628) / 100) as f32;
    for t in transforms {
        t.rotate_around(y_rotation_point, Quat::from_rotation_y(rand_rotation));
    }
}

fn apply_curve(transforms: &mut Vec<Transform>, x: f32, y: f32, z: f32) {
    let curve_rotation_point = Vec3::new(
        x + thread_rng().gen_range(0..2) as f32 / 10.0,
        y,
        z + thread_rng().gen_range(0..2) as f32 / 10.0,
    );
    let rand_curve = (thread_rng().gen_range(90..110) / 100) as f32;
    let rand_straight =
        thread_rng().gen_range(GRASS_STRAIGHTNESS_MIN..GRASS_STRAIGHTNESS_MAX) as f32;
    for t in transforms {
        t.rotate_around(
            curve_rotation_point,
            Quat::from_rotation_z(
                rand_curve * ((t.translation.y - y) / (GRASS_HEIGHT * rand_straight)),
            ),
        );
    }
}

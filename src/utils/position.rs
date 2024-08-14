use bevy::math::Vec3;

/// Get the distance between a and b.
/// You could compare the X coordinates of two objects.
pub fn get_distance(a: f32, b: f32) -> f32 {
    (a - b).abs()
}

// Get XZ translation distance between two objects.
pub fn get_translation_distance_xz(translation_a: Vec3, translation_b: Vec3) -> (f32, f32) {
    let dist_x = (translation_a.x - translation_b.x).abs();
    let dist_z = (translation_a.z - translation_b.z).abs();
    (dist_x, dist_z)
}

// Get XZ translation distance between two objects
pub fn translations_in_range(
    translation_a: Vec3,
    translation_b: Vec3,
    distance_range: f32,
) -> bool {
    let (dist_x, dist_z) = get_translation_distance_xz(translation_a, translation_b);
    dist_x <= distance_range && dist_z <= distance_range
}

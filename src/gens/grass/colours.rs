use super::constants::GRASS_HEIGHT;

pub fn color_gradient_y_based(y: f32, rgba1: [f32; 4], rgba2: [f32; 4]) -> [f32; 4] {
    let [r1, g1, b1, a1] = rgba1;
    let [r2, g2, b2, a2] = rgba2;
    let r = r1 + (r2 - r1) * (y / GRASS_HEIGHT);
    let g = g1 + (g2 - g1) * (y / GRASS_HEIGHT);
    let b = b1 + (b2 - b1) * (y / GRASS_HEIGHT);
    let a = a1 + (a2 - a1) * (y / GRASS_HEIGHT);
    [r, g, b, a]
}

use crate::constants::{
    COLOR_PEAKS, COLOR_SAND, COLOR_TEMPERATE, HEIGHT_PEAKS, HEIGHT_SAND, HEIGHT_TEMPERATE_END,
    HEIGHT_TEMPERATE_START,
};

pub fn get_terrain_color(y: f32) -> [f32; 4] {
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

pub fn terrain_color_gradient(ratio: f32, rgba1: [f32; 4], rgba2: [f32; 4]) -> [f32; 4] {
    let [r1, g1, b1, a1] = rgba1;
    let [r2, g2, b2, a2] = rgba2;

    [
        r1 + (r2 - r1) * (ratio),
        g1 + (g2 - g1) * (ratio),
        b1 + (b2 - b1) * (ratio),
        a1 + (a2 - a1) * (ratio),
    ]
}

use crate::constants::TERRAIN_SEED;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub fn terrain_perlin() -> Perlin {
    Perlin::new(TERRAIN_SEED)
}

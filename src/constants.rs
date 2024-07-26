use bevy::prelude::Transform;

pub const PLAYER_HEIGHT: f32 = 1.0;
pub const SPAWN_TRANSFORM: Transform = Transform::from_xyz(0.0, 200. + PLAYER_HEIGHT + 5., 0.0);
pub const TURN_SPEED: f32 = 0.2;
pub const MAX_TURN_SPEED: f32 = 1.;
pub const FORWARD_SPEED: f32 = 0.03;
// pub const FORWARD_SPEED: f32 = 0.001;
// we can make the forward speed faster than max when going down hill
pub const MAX_SPEED: f32 = 3.;
pub const MAX_CAM_DISTANCE: f32 = 12.;

pub const PLANE_SIZE: f32 = 6000.;
pub const SIZE_NO_PLAYER: f32 = 6000.; // TODO: This actually causes overlaps if it is bigger than PLANE_SIZE
pub const SUBDIVISIONS_LEVEL_1: u32 = 1024;
pub const SUBDIVISIONS_LEVEL_2: u32 = 256;
pub const SUBDIVISIONS_LEVEL_3: u32 = 2;
pub const TILE_WIDTH: u32 = 4; // how wide a tile should be
pub const TEXTURE_SCALE: f32 = 7.;
pub const WATER_TEXTURE_SCALE: f32 = 20.;
pub const BASE_LEVEL: f32 = 200.;
pub const WATER_LEVEL: f32 = 189.;
pub const WATER_SCROLL_SPEED: f32 = 0.0002;
pub const HEIGHT_PEAKS: f32 = 1500.;
pub const HEIGHT_SAND: f32 = 200.;
pub const HEIGHT_TEMPERATE_START: f32 = 210.;
pub const HEIGHT_TEMPERATE_END: f32 = 800.;
pub const COLOR_TEMPERATE: [f32; 4] = [0.079, 0.079, 0., 1.];
pub const COLOR_SAND: [f32; 4] = [80. / 255., 72. / 255., 49. / 255., 255. / 255.];
pub const COLOR_PEAKS: [f32; 4] = [255. / 255., 255. / 255., 255. / 255., 255. / 255.];

pub const WIND_SEED: u32 = 0;
pub const GRASS_HEIGHT_SEED: u32 = 1;
pub const TORCH_SEED: u32 = 2;
pub const TERRAIN_SEED: u32 = 040658;
pub const HILL_HEIGHTS: f32 = 10.0;
pub const TERRAIN_BUMPINESS: f32 = 2.0;
pub const MOUNTAIN_HEIGHTS: f32 = 256.;

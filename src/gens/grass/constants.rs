use bevy::render::{mesh::MeshVertexAttribute, render_resource::VertexFormat};

pub const ATTRIBUTE_BASE_Y: MeshVertexAttribute =
    MeshVertexAttribute::new("BaseY", 988540917, VertexFormat::Float32);
pub const ATTRIBUTE_STARTING_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("StartingPosition", 988540916, VertexFormat::Float32x3);
pub const ATTRIBUTE_WORLD_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("WorldPosition", 988540915, VertexFormat::Float32x3);

pub const GRID_SIZE_HALF: i32 = 8;

pub const GRASS_TILE_SIZE_1: f32 = 16.; // 32.;
pub const GRASS_TILE_SIZE_2: f32 = 32.; // TODO: like terrain, this causes overlaps if bigger than SIZE_1
pub const NUM_GRASS_1: u32 = 64; // 128; // number of grass blades in one row of a tile
pub const NUM_GRASS_2: u32 = 32;
pub const GRASS_BLADE_VERTICES: u32 = 3;
pub const GRASS_WIDTH: f32 = 0.15;
pub const GRASS_HEIGHT: f32 = 2.1;
pub const GRASS_BASE_COLOR_1: [f32; 4] = [0.102, 0.153, 0., 1.];
pub const GRASS_BASE_COLOR_2: [f32; 4] = [0., 0.019, 0., 1.];
pub const GRASS_BASE_COLOR_3: [f32; 4] = [0.01019607843, 0.04803921569, 0.0123529412, 1.];
// pub const GRASS_BASE_COLOR_3: [f32; 4] = [0.00919607843, 0.06403921569, 0.0323529412, 1.];
pub const GRASS_BASE_COLOR_4: [f32; 4] = [0.001176470588, 0.002745098039, 0.002470588235, 1.];
pub const GRASS_SECOND_COLOR: [f32; 4] = [0.079, 0.079, 0., 1.];
pub const GRASS_SECOND_COLOR_2: [f32; 4] = [0.00819607843, 0.02253921569, 0.01764705882, 1.];
// pub const GRASS_SECOND_COLOR_3: [f32; 4] = [0.019, 0.079, 0.03, 1.];
pub const GRASS_SECOND_COLOR_3: [f32; 4] = [0.008, 0.03, 0.01, 1.];
pub const GRASS_SCALE_FACTOR: f32 = 1.0;
pub const GRASS_HEIGHT_VARIATION_FACTOR: f32 = 0.6;
// 10. default straightness. 3. for leaning, 1. for trampled.
pub const GRASS_STRAIGHTNESS: f32 = 3.0; // 10. default for now, as opposed to a curve factor, just modifying denominator for curve calcs
pub const GRASS_STRAIGHTNESS_MIN: u32 = 1;
pub const GRASS_STRAIGHTNESS_MAX: u32 = 12;
pub const GRASS_SPACING: f32 = 0.1;
pub const GRASS_OFFSET: f32 = 0.2;
pub const ENABLE_WIREFRAME: bool = false;
pub const WIND_STRENGTH: f32 = 1.;
pub const WIND_SPEED: f64 = 0.5;
pub const WIND_CONSISTENCY: f64 = 50.0; //
pub const WIND_LEAN: f32 = 0.3; // determines how already bent grass will be at 0 wind
pub const CURVE_POWER: f32 = 1.0; // the linearity / exponentiality of the application/bend of the wind
pub const DESPAWN_DISTANCE: f32 =
    (GRID_SIZE_HALF + 1) as f32 * GRASS_TILE_SIZE_1 + GRID_SIZE_HALF as f32;
// pub const DESPAWN_DISTANCE: f32 = 2. * GRASS_TILE_SIZE_1;
pub const WIND_SIM_TRIGGER_DISTANCE: f32 = 1.5 * GRASS_TILE_SIZE_1;
pub const WIND_SIM_DISTANCE: f32 = WIND_SIM_TRIGGER_DISTANCE - GRASS_TILE_SIZE_1 / 2.;

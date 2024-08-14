use bevy::{
    asset::Handle,
    ecs::world::CommandQueue,
    math::Vec3,
    prelude::{Component, Deref, Mesh, Resource},
    tasks::Task,
    utils::HashMap,
};
use bevy_pbr::StandardMaterial;

#[derive(Component)]
pub struct GrassData {
    pub initial_vertices: Vec<Vec3>,
    pub initial_positions: Vec<[f32; 3]>,
}

#[derive(Component, Clone)]
pub struct Grass;

/// Grass offsets component.
/// (X, Z) are the coordinates.
/// The boolean represents generation state.
/// We set it to false when despawning.
#[derive(Component)]
pub struct GrassGrid(pub HashMap<(i32, i32), bool>);

#[derive(Component)]
pub struct GenGrassTask(pub Task<CommandQueue>);

#[derive(Resource, Deref)]
pub struct GrassMeshHandle(pub Handle<Mesh>);

#[derive(Resource, Deref)]
pub struct GrassMaterialHandle(pub Handle<StandardMaterial>);

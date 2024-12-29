use bevy::prelude::{Mesh3d, Transform};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};

use super::resources::Rock;

pub type RockBundle = (Mesh3d, MeshMaterial3d<StandardMaterial>, Transform, Rock);

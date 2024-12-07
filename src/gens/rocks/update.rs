use bevy::prelude::*;

use super::{constants::NUM_ROCKS, resources::Rock, rock::spawn_rock};

/// Controls spawning of rocks. Does not update rocks.
pub fn update_rocks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    //
}

pub fn spawn_rocks_basic(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rocks: Query<(Entity), With<Rock>>,
) {
    let mut new_rocks: Vec<(MaterialMeshBundle<StandardMaterial>, Rock)> = Vec::new();
    let quantity = NUM_ROCKS - rocks.iter().count() as u32;

    if quantity == 0 {
        return;
    }

    for i in 0..quantity {
        new_rocks.push(spawn_rock(
            &commands,
            &asset_server,
            &mut meshes,
            &images,
            &mut materials,
            i,
        ));
    }

    commands.spawn_batch(new_rocks);
}

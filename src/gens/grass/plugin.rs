use bevy::{
    app::{App, Plugin, Update},
    prelude::default,
};
use bevy_pbr::{ExtendedMaterial, MaterialPlugin, StandardMaterial};

use super::{
    materials::GrassMaterialExtension,
    update::{handle_tasks, update_grass},
};

pub struct GrassPlugin;

impl Plugin for GrassPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, GrassMaterialExtension>,
        > {
            prepass_enabled: false,
            ..default()
        });
        app.add_systems(Update, (update_grass, handle_tasks));
    }
}

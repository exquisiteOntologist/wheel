use bevy::{
    asset::{AssetServer, Assets, Handle},
    prelude::{default, Image, Res, ResMut},
};
use bevy_pbr::StandardMaterial;

use super::textures::rock_texture;

pub fn rock_material(
    mut asset_server: &Res<AssetServer>,
    mut images: &ResMut<Assets<Image>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let texture = rock_texture(&asset_server);
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        ..default()
    });

    debug_material
}

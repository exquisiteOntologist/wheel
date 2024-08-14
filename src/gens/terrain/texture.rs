use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Res,
    prelude::Image,
};

use crate::meshes::image_settings_with_repeat_image_sampler;

pub fn get_terrain_texture(asset_server: &Res<AssetServer>, t_color: &[f32; 4]) -> Handle<Image> {
    let asset_settings = image_settings_with_repeat_image_sampler();
    let t_texture = asset_server.load_with_settings("textures/ground/sand.png", asset_settings);
    t_texture
}

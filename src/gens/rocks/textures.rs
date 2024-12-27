use bevy::{
    asset::{AssetServer, Handle},
    image::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
    prelude::{default, Image, Res},
};

const ROCK_TEXTURE_PATHS: [&str; 1] = ["textures/rocks/ochre-bna.png"];

pub fn rock_texture(asset_server: &Res<AssetServer>) -> Handle<Image> {
    let path = ROCK_TEXTURE_PATHS[0];
    let t_texture = asset_server.load_with_settings(path, asset_settings());
    t_texture
}

fn asset_settings() -> impl Fn(&mut ImageLoaderSettings) {
    let sampler_repeat_image: ImageSamplerDescriptor = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        // mag_filter: ImageFilterMode::Nearest,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        lod_min_clamp: 0.0,
        lod_max_clamp: 32.0,
        anisotropy_clamp: 100,
        ..default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_repeat_image.clone());
    };

    settings
}

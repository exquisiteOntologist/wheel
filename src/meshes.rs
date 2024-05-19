use bevy::render::{
    mesh::{Mesh, VertexAttributeValues},
    texture::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
};

// this is necessary, but also necessary is for the UV of the mesh to repeat
// the critical part of repeating the image is the address mode being set to repeat
// https://www.reddit.com/r/bevy/comments/18qoctw/how_do_i_make_a_texture_tilerepeat_in_a_material/?rdt=35295
pub fn image_settings_with_repeat_image_sampler() -> impl Fn(&mut ImageLoaderSettings) {
    let sampler_repeat_image: ImageSamplerDescriptor = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        lod_min_clamp: 0.0,
        lod_max_clamp: 32.0,
        anisotropy_clamp: 1,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_repeat_image.clone());
    };

    settings
}

/// Update the UVs of a mesh, taking 2 multipliers
pub fn mesh_update_uv(mesh: &mut Mesh, m_a: f32, m_b: f32) {
    if let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs {
            uv[0] *= m_a;
            uv[1] *= m_b;
        }
    };
}

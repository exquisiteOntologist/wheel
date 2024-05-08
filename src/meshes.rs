use bevy::render::{
    mesh::{Mesh, VertexAttributeValues},
    texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
};

// this is necessary, but also necessary is for the UV of the mesh to repeat
// https://www.reddit.com/r/bevy/comments/18qoctw/how_do_i_make_a_texture_tilerepeat_in_a_material/?rdt=35295
pub fn image_settings_with_repeat_image_sampler() -> impl Fn(&mut ImageLoaderSettings) {
    let sampler_repeat_image: ImageSamplerDescriptor = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
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

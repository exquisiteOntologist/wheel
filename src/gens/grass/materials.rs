use bevy::pbr::{MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};

use bevy::render::mesh::MeshVertexBufferLayoutRef;

use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};

use super::constants::{ATTRIBUTE_BASE_Y, ATTRIBUTE_STARTING_POSITION, ATTRIBUTE_WORLD_POSITION};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GrassMaterialExtension {}

impl MaterialExtension for GrassMaterialExtension {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/grass_shader.wgsl".into()
    }

    // fn fragment_shader() -> ShaderRef {
    //     "shaders/grass_shader.wgsl".into()
    // }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<GrassMaterialExtension>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let mut pos_position = 0;
        let mut normal_position = 1;
        let mut color_position = 5;
        if let Some(label) = &mut descriptor.label {
            println!("Label is: {}", label);
            if label == "pbr_prepass_pipeline" {
                pos_position = 0;
                normal_position = 3;
                color_position = 7;
            }
        }

        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(pos_position),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(normal_position),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(color_position),
            // Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            // Mesh::ATTRIBUTE_TANGENT.at_shader_location(4),
            ATTRIBUTE_BASE_Y.at_shader_location(16),
            ATTRIBUTE_STARTING_POSITION.at_shader_location(17),
            ATTRIBUTE_WORLD_POSITION.at_shader_location(18),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

pub fn grass_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::WHITE,
        double_sided: false,
        perceptual_roughness: 0.9,
        reflectance: 0.2,
        // cull_mode: None,
        opaque_render_method: bevy::pbr::OpaqueRendererMethod::Forward,
        unlit: false,
        ..default()
    }
}

use bevy::math::IVec3;
use bevy::prelude::Mesh;
use crate::chunk_vertexes::{generate_chunk_quad_groups, QuadDirection, VoxelQuads};
use crate::{Chunk};

pub fn generate_mesh(chunk: &Chunk) -> Mesh {
    let voxel_quad_groups = generate_chunk_quad_groups(&chunk);
    create_chunk_mesh(&voxel_quad_groups)
}

fn create_chunk_mesh(quads: &VoxelQuads) -> Mesh {
    // TODO group the vertices by quads (every 6 vertices = quad), determine which face they are,
    // TODO pick a texture based on that (grass top dirt side)
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for voxel_quad in quads {

        let mut texture_atlas_index = 0;
        let height = voxel_quad.y as f32;

        if height >= 24.0 {
            // Top = grass, rest = dirt
            texture_atlas_index = match voxel_quad.quad.direction {
                QuadDirection::TOP => 1,
                _ => 4
            };
        } else if height >= 18.0 {

            // Top = grass, rest = dirt
            texture_atlas_index = match voxel_quad.quad.direction {
                QuadDirection::TOP => 0,
                _ => 4
            };

        } else if height >= 10.0 {
            // Top = grass, rest = dirt
            texture_atlas_index = match voxel_quad.quad.direction {
                QuadDirection::TOP => 2,
                _ => 4
            };
        } else {
            texture_atlas_index = 3;
        }

        for vertex in voxel_quad.quad.vertexes {
            let (position, normal, uv) = &vertex;

            positions.push(*position);
            normals.push(*normal);
            uvs.push(uvs_to_atlas_uvs(uv, 4, texture_atlas_index));
        }
    }

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

fn uvs_to_atlas_uvs(uvs: &[f32;2], atlas_width: i32, atlas_index: i32) -> [f32; 2] {
    let x_index = atlas_index % atlas_width;
    let y_index = (atlas_index as f32 / atlas_width as f32) as i32;
    let texture_width = 1.0 / atlas_width as f32;

    let mut new_uv = [ 0.0, 0.0 ];

    if uvs[0] == 0.0 {
        new_uv[0] = x_index as f32 * texture_width;
    } else {
        new_uv[0] = (x_index + 1) as f32 * texture_width;
    }

    if uvs[1] == 0.0 {
        new_uv[1] = y_index as f32 * texture_width;
    } else {
        new_uv[1] = (y_index + 1) as f32 * texture_width;
    }

    return new_uv;
}
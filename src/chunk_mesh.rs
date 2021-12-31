use bevy::prelude::Mesh;
use crate::chunk_vertexes::Vertexes;
use crate::{Chunk, generate_chunk_vertexes};

pub fn generate_mesh(chunk_x: i32, chunk_y: i32, chunk_z: i32) -> Vec<Mesh> {
    let mut meshes = Vec::new();
    let vertices_arr = generate_chunk_vertexes(&Chunk::noise(chunk_x, chunk_y, chunk_z));

    for (_, vertices) in vertices_arr.iter().enumerate() {
        meshes.push(create_chunk_mesh(vertices));
    }

    meshes
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

fn create_chunk_mesh(vertices: &Vertexes) -> Mesh {
    // TODO group the vertices by quads (every 6 vertices = quad), determine which face they are,
    // TODO pick a texture based on that (grass top dirt side)
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    for i in (0..vertices.len()).step_by(6) {

        // TODO get min position height

        let (position_, _, _) = vertices.get(i).unwrap();

        let mut texture_atlas_index = 0;
        let height = position_[1];

        if height >= 24.0 {
            texture_atlas_index = 1;
        } else if height >= 18.0 {
            texture_atlas_index = 0;
        } else if height >= 10.0 {
            texture_atlas_index = 2;
        } else {
            texture_atlas_index = 3;
        }

        for v_index in 0..6 {
            let (position, normal, uv) = vertices.get(i + v_index).unwrap();

            positions.push(*position);
            normals.push(*normal);
            uvs.push(uvs_to_atlas_uvs(uv, 4, texture_atlas_index));
        }
    }

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
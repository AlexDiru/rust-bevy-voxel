use crate::vert_gen::{bottom_plane_vertices, top_plane_vertices};
use crate::chunk::{Chunk};
use crate::{back_plane_vertices, front_plane_vertices, left_plane_vertices, right_plane_vertices};

pub fn generate_chunk_mesh(chunk: &Chunk) -> Vec<Vertexes>{
    let mut meshes = Vec::new();
    let mut visited = Vec::new();

    for n in 0..(32 * 32 * 32) {
        /**
        x = i % max_x
        y = ( i / max_x ) % max_y
        z = i / ( max_x * max_y ) */
        let x = n % 32;
        let y = (n / 32) % 32;
        let z = n / (32 * 32);

        if !visited.contains(&(x, y, z)) {
            if chunk.get_voxel(&(x, y, z)).solid {
                let mut res = generate_chunk_mesh_from_voxel(chunk, x, y, z);
                meshes.push(res.vertexes);
                visited.append(&mut res.visited);
            } else {
                visited.push((x, y, z));
            }
        }
    }

    meshes
}

type Vertexes = Vec<([f32; 3], [f32; 3], [f32; 2])>;

struct ChunkMeshGenResult {
    pub vertexes: Vertexes,
    pub visited: Vec<(usize, usize, usize)>
}

fn generate_chunk_mesh_from_voxel(chunk: &Chunk, start_x: usize, start_y: usize, start_z: usize) -> ChunkMeshGenResult {
    let mut vertices = Vec::new();

    let mut visited = Vec::new();
    visited.push((start_x, start_y, start_z));

    let mut queue = Vec::new();
    queue.push((start_x, start_y, start_z));

    while !queue.is_empty() {
        let (x, y, z) = queue.pop().unwrap();

        let mut vxs = Vec::new();

        if x > 0 {
            vxs.push((x - 1, y, z)); // FRONT
        } else {
            vertices.extend_from_slice(&front_plane_vertices(x as f32, z as f32, y as f32));
        }

        if x < 31 {
            vxs.push((x + 1, y, z)); // BACK
        } else {
            vertices.extend_from_slice(&back_plane_vertices(x as f32, z as f32, y as f32));
        }

        if y > 0 {
            vxs.push((x, y - 1, z)); // LEFT
        } else {
            vertices.extend_from_slice(&left_plane_vertices(x as f32, z as f32, y as f32));
        }

        if y < 31 {
            vxs.push((x, y + 1, z)); // RIGHT
        } else {
            vertices.extend_from_slice(&right_plane_vertices(x as f32, z as f32, y as f32));
        }

        if z > 0 {
            vxs.push((x, y, z - 1)); // BOTTOM
        } else {
            vertices.extend_from_slice(&bottom_plane_vertices(x as f32, z as f32, y as f32));
        }

        if z < 31 {
        vxs.push((x, y, z + 1)); // TOP
        } else {
            vertices.extend_from_slice(&top_plane_vertices(x as f32, z as f32, y as f32));
        }

        for (_, vx) in vxs.iter().enumerate() {
            if chunk.get_voxel(vx).solid {
                if !visited.contains(vx) {
                    visited.push(vx.clone());
                    queue.push(vx.clone());
                }
            } else {
                // Fill in a wall
                if vx.0 < x {
                    vertices.extend_from_slice(&front_plane_vertices(x as f32, z as f32, y as f32));
                } else if vx.0 > x {
                    vertices.extend_from_slice(&back_plane_vertices(x as f32, z as f32, y as f32));
                } else if vx.1 < y {
                    vertices.extend_from_slice(&left_plane_vertices(x as f32, z as f32, y as f32));
                } else if vx.1 > y {
                    vertices.extend_from_slice(&right_plane_vertices(x as f32, z as f32, y as f32));
                } else if vx.2 < z {
                    vertices.extend_from_slice(&bottom_plane_vertices(x as f32, z as f32, y as f32));
                } else if vx.2 > z {
                    vertices.extend_from_slice(&top_plane_vertices(x as f32, z as f32, y as f32));
                }
            }
        }
    }

    ChunkMeshGenResult {
        visited,
        vertexes: vertices
    }
}
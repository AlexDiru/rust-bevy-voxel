
use crate::chunk::{Chunk};
use crate::{CHUNK_SIZE};

pub type Vertexes = Vec<([f32; 3], [f32; 3], [f32; 2])>;
pub type Vertex = ([f32; 3], [f32; 3], [f32; 2]);
pub type Quad = [Vertex; 6];

#[exec_time]
pub fn generate_chunk_vertexes(chunk: &Chunk) -> Vec<Vertexes> {
    let mut meshes = Vec::new();
    let mut visited = Vec::new();

    for mut n in 0..(CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) {
        if visited.contains(&n) {
            // We would have already checked the neighbours
            n = n + 1;
            continue;
        }

        let x = n % 32;
        let y = (n / 32) % 32;
        let z = n / (32 * 32);

        if chunk.get_voxel(x, y, z).solid {
            /*
            x = i % max_x
            y = ( i / max_x ) % max_y
            z = i / ( max_x * max_y ) */
            let mut res = generate_chunk_mesh_from_voxel(chunk, n,x, y, z);
            meshes.push(res.vertexes);
            visited.append(&mut res.visited);
        } //else {
          //  visited.push(n);
        //}
    }

    meshes
}

struct ChunkMeshGenResult {
    pub vertexes: Vertexes,
    pub visited: Vec<usize>
}

fn generate_chunk_mesh_from_voxel(chunk: &Chunk, voxel_index: usize, start_x: usize, start_y: usize, start_z: usize) -> ChunkMeshGenResult {
    let mut vertices = Vec::new();

    let mut visited = Vec::new();
    visited.push(voxel_index);

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
            if chunk.get_voxel(vx.0, vx.1, vx.2).solid {
                let vx_index = xyz_to_voxel_index(vx.0, vx.1, vx.2);
                if !visited.contains(&vx_index) {
                    visited.push(vx_index.clone());
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

fn xyz_to_voxel_index(x: usize, y: usize, z: usize) -> usize {
    x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_SIZE)
}

fn top_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> Quad {
    [
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

fn left_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> Quad {
    [
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
    ]
}

fn right_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> Quad {
    [
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),


        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
    ]
}

fn bottom_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> Quad {
    [
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
    ]
}

fn back_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> Quad {
    [
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
    ]
}

fn front_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> Quad {
    [
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
    ]
}
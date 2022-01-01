
use crate::chunk::{Chunk};
use crate::{CHUNK_SIZE, IVec3};
use crate::chunk_vertexes::QuadDirection::FRONT;
use crate::KeyCode::P;

pub enum QuadDirection {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    FRONT,
    BACK
}

pub type Vertexes = Vec<([f32; 3], [f32; 3], [f32; 2])>;
pub type Vertex = ([f32; 3], [f32; 3], [f32; 2]);

pub struct Quad {
    pub vertexes: [Vertex; 6],
    pub direction: QuadDirection
}

pub struct VoxelQuad {
    pub quad: Quad,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Quad {
    pub fn get_lowest_vertex_x(&self) -> f32 {
        // lol
        f32::min(f32::min(f32::min(f32::min(f32::min(
            self.vertexes[0].0[0],
            self.vertexes[1].0[0]),
                                            self.vertexes[2].0[0]),
                                   self.vertexes[3].0[0]),
                          self.vertexes[4].0[0]),
                 self.vertexes[5].0[0])
    }

    pub fn get_lowest_vertex_z(&self) -> f32 {
        // lol
        f32::min(f32::min(f32::min(f32::min(f32::min(
            self.vertexes[0].0[2],
            self.vertexes[1].0[2]),
                                            self.vertexes[2].0[2]),
                                   self.vertexes[3].0[2]),
                          self.vertexes[4].0[2]),
                 self.vertexes[5].0[2])
    }

    pub fn get_lowest_vertex_y(&self) -> f32 {
        // lol
        f32::min(f32::min(f32::min(f32::min(f32::min(
            self.vertexes[0].0[1],
            self.vertexes[1].0[1]),
            self.vertexes[2].0[1]),
            self.vertexes[3].0[1]),
            self.vertexes[4].0[1]),
            self.vertexes[5].0[1])
    }
}

pub type Quads = Vec<Quad>;
pub type VoxelQuads = Vec<VoxelQuad>;

#[exec_time]
pub fn generate_chunk_quad_groups(chunk: &Chunk) -> Vec<VoxelQuads> {
    let mut meshes : Vec<VoxelQuads> = Vec::new();
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
            meshes.push(res.voxel_quads);
            visited.append(&mut res.visited);
        } //else {
          //  visited.push(n);
        //}
    }

    meshes
}

struct ChunkMeshGenResult {
    pub voxel_quads: VoxelQuads,
    pub visited: Vec<usize>
}

fn generate_chunk_mesh_from_voxel(chunk: &Chunk, voxel_index: usize, start_x: usize, start_y: usize, start_z: usize) -> ChunkMeshGenResult {
    let mut voxel_quads: Vec<VoxelQuad> = Vec::new();

    let mut visited = Vec::new();
    visited.push(voxel_index);

    let mut queue = Vec::new();
    queue.push((start_x, start_y, start_z));

    while !queue.is_empty() {
        let (x, y, z) = queue.pop().unwrap();

        let mut vxs = Vec::new();

        let mut quad_directions = Vec::new();

        if x > 0 {
            vxs.push((x - 1, y, z));
        } else {
            quad_directions.push(QuadDirection::FRONT);
        }

        if x < 31 {
            vxs.push((x + 1, y, z));
        } else {
            quad_directions.push(QuadDirection::BACK);
        }

        if y > 0 {
            vxs.push((x, y - 1, z));
        } else {
            quad_directions.push(QuadDirection::LEFT);
        }

        if y < 31 {
            vxs.push((x, y + 1, z));
        } else {
            quad_directions.push(QuadDirection::RIGHT);
        }

        if z > 0 {
            vxs.push((x, y, z - 1));
        } else {
            quad_directions.push(QuadDirection::BOTTOM);
        }

        if z < 31 {
            vxs.push((x, y, z + 1));
        } else {
            quad_directions.push(QuadDirection::TOP);
        }

        for quad_direction in quad_directions.into_iter() {
            voxel_quads.push(generate_voxel_quad(quad_direction, x, y, z))
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
                let mut quad_directions_2 = Vec::new();

                if vx.0 < x {
                    quad_directions_2.push(QuadDirection::FRONT)
                } else if vx.0 > x {
                    quad_directions_2.push(QuadDirection::BACK)
                } else if vx.1 < y {
                    quad_directions_2.push(QuadDirection::LEFT)
                } else if vx.1 > y {
                    quad_directions_2.push(QuadDirection::RIGHT)
                } else if vx.2 < z {
                    quad_directions_2.push(QuadDirection::BOTTOM)
                } else if vx.2 > z {
                    quad_directions_2.push(QuadDirection::TOP)
                };

                for quad_direction in quad_directions_2.into_iter() {
                    voxel_quads.push(generate_voxel_quad(quad_direction, x, y, z));
                }
            }
        }
    }

    ChunkMeshGenResult {
        visited,
        voxel_quads
    }
}

fn xyz_to_voxel_index(x: usize, y: usize, z: usize) -> usize {
    x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_SIZE)
}

fn generate_voxel_quad(quad_direction: QuadDirection, x: usize, y: usize, z: usize) -> VoxelQuad {
    VoxelQuad {
        // Note Y <=> Z swapped to work, cba to work out why
        quad: generate_quad(quad_direction, x as f32, z as f32, y as f32),
        x,
        y: z,
        z: y,
    }
}

fn generate_quad(quad_direction: QuadDirection, x_offset: f32, y_offset: f32, z_offset: f32) -> Quad {
    match quad_direction {
        QuadDirection::TOP => Quad {
            vertexes: [
                ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
                ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
            ],
            direction: quad_direction },
        QuadDirection::LEFT =>  Quad {
            vertexes: [
                ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
            ],
            direction: quad_direction },
        QuadDirection::RIGHT => Quad {
            vertexes: [
                ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
                ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
            ],
            direction: quad_direction },
        QuadDirection::BOTTOM => Quad {
            vertexes: [
                ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ],
            direction: quad_direction },
        QuadDirection::BACK => Quad {
            vertexes: [
                ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ],
            direction: quad_direction },
        QuadDirection::FRONT => Quad {
            vertexes: [
                ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
                ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
                ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
                ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
                ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ],
            direction: quad_direction }
    }
}
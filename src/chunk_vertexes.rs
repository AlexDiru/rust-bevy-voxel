use crate::{Chunk, IVec3};
use crate::chunk_utils::{voxel_index_to_xyz, xyz_to_voxel_index};

#[derive(PartialEq, Eq, Copy, Clone)]
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
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
pub fn generate_chunk_quad_groups(chunk: &Chunk) -> VoxelQuads {
    generate_chunk_mesh_from_voxel(chunk).voxel_quads
}

struct ChunkMeshGenResult {
    pub voxel_quads: VoxelQuads,
    pub visited: Vec<usize>
}

struct OffsetAndDirection {
    pub offset: IVec3,
    pub direction: QuadDirection,
}

fn generate_chunk_mesh_from_voxel(chunk: &Chunk) -> ChunkMeshGenResult {
    let mut voxel_quads: Vec<VoxelQuad> = Vec::new();

    for n in 0..(32 * 32 * 32) {
        if !chunk.voxels[n].solid {
            continue
        }

        let center_voxel = voxel_index_to_xyz(n as usize as i32, &chunk.size);
        let x = center_voxel.x;
        let y = center_voxel.y;
        let z = center_voxel.z;

        let offset_and_directions : [OffsetAndDirection; 6] = [
            OffsetAndDirection { offset: IVec3::new(-1, 0, 0), direction: QuadDirection::FRONT } ,
            OffsetAndDirection { offset: IVec3::new(1, 0, 0), direction: QuadDirection::BACK } ,
            OffsetAndDirection { offset: IVec3::new(0, 0, -1), direction: QuadDirection::BOTTOM } ,
            OffsetAndDirection { offset: IVec3::new(0, 0, 1), direction: QuadDirection::TOP } ,
            OffsetAndDirection { offset: IVec3::new(0, -1,  0),direction:  QuadDirection::LEFT } ,
            OffsetAndDirection { offset: IVec3::new(0, 1,0), direction: QuadDirection::RIGHT } ,
        ];

        for offset_and_direction in offset_and_directions {
            let neighbour_voxel_location = center_voxel + offset_and_direction.offset;
            let neighbour_voxel = chunk.generate_voxel_in_localspace(&neighbour_voxel_location);

            let mut add_face = || voxel_quads.push(generate_voxel_quad(offset_and_direction.direction.clone(), x, y, z));

            // Height OOB
            if neighbour_voxel_location.z == 31 {
                if offset_and_direction.direction == QuadDirection::TOP {
                    add_face();
                }
                continue;
            }

            if !neighbour_voxel.solid {
                add_face();
                continue;
            }
        }
    }

    println!("VOxel Quads size {}", voxel_quads.len());

    ChunkMeshGenResult {
        voxel_quads,
        visited: Vec::new(),
    }
}

fn generate_voxel_quad(quad_direction: QuadDirection, x: i32, y: i32, z: i32) -> VoxelQuad {
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
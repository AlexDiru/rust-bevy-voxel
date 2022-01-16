use bevy::math::IVec3;
use opensimplex_noise_rs::OpenSimplexNoise;
use crate::voxel::Voxel;

pub const CHUNK_SIZE_X: usize = 32;
pub const CHUNK_SIZE_Y: usize = 32;
pub const CHUNK_SIZE_Z: usize = 32;
pub const CHUNK_SIZE_X_I32: i32 = CHUNK_SIZE_X as i32;
pub const CHUNK_SIZE_Y_I32: i32 = CHUNK_SIZE_Y as i32;
pub const CHUNK_SIZE_Z_I32: i32 = CHUNK_SIZE_Z as i32;
type Voxels = [[[Voxel; CHUNK_SIZE_X]; CHUNK_SIZE_Y]; CHUNK_SIZE_Z];

pub struct Chunk {
    voxels: Voxels,
    pub is_empty: bool, // A chunk with no solid voxels, massively optimised the y rendering of empty chunks
    // TODO maybe take Minecraft approach of 16x16x280 chunks
}

pub struct ChunkGenerationAttributes {
    pub calculate_solid_probability: fn(x: f32, y: f32, z: f32) -> f32
}

fn sparse_chunk() -> ChunkGenerationAttributes {
    ChunkGenerationAttributes {
        calculate_solid_probability: |x: f32, y: f32, z: f32| -> f32 {
            return 0.3;
        }
    }
}

fn flat_chunk() -> ChunkGenerationAttributes {
    ChunkGenerationAttributes {
        calculate_solid_probability: |x: f32, y: f32, z: f32| -> f32 {
            let c = 28.0; // The higher c is, the higher the probability that the higher voxels are solid
            1.0 - (z / c)
        }
    }
}

fn solid_voxels() -> Voxels { [[[Voxel::new(true); CHUNK_SIZE_X]; CHUNK_SIZE_Y]; CHUNK_SIZE_Z] }
fn non_solid_voxels() -> Voxels { [[[Voxel::new(false); CHUNK_SIZE_X]; CHUNK_SIZE_Y]; CHUNK_SIZE_Z] }

pub fn to_3d(idx: i32) -> IVec3 {
    let z = idx / (CHUNK_SIZE_X_I32 * CHUNK_SIZE_Y_I32);
    let idx_yx = idx - (z * CHUNK_SIZE_X_I32 * CHUNK_SIZE_Y_I32);
    let y = idx_yx / CHUNK_SIZE_X_I32;
    let x = idx_yx % CHUNK_SIZE_X_I32;
    return IVec3::new(x, y, z)
}

impl Chunk {
    pub fn noise(chunk_x: i32, chunk_y: i32, chunk_z: i32) -> Chunk {
        let mut voxels = non_solid_voxels();
        let mut contains_solid_voxels = false;

        let noise_generator = OpenSimplexNoise::new(Some(883_279_212_983_182_319)); // if not provided, default seed is equal to 0
        let scale = 0.1;
        let x_offset = chunk_x * CHUNK_SIZE_X_I32;
        let y_offset = chunk_y * CHUNK_SIZE_Y_I32 + 8;
        let z_offset = chunk_z * CHUNK_SIZE_Z_I32;

        for n in 0..(CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z) {
            let xyz = to_3d(n as i32);
            let x = xyz.x as usize;
            let y = xyz.y as usize;
            let z = xyz.z as usize;

            let mut val = noise_generator.eval_3d(
                (x as i32 + x_offset) as f64 * scale,
                (y as i32 + y_offset) as f64 * scale,
                (z as i32 + z_offset) as f64 * scale);

            // Normalise val from -1 to 1, to 0 to 1
            val = (val + 1.0) / 2.0;

            let chance = (flat_chunk().calculate_solid_probability)(x as f32, y as f32, (z as i32 + z_offset) as f32);

            // The chance of the voxel being solid, increases the lower y is
            //let chance = (z as f64 / 16.0);

            let solid = val as f32 <= chance;

            voxels[z][y][x].solid = solid;
            if solid {
                contains_solid_voxels = true;
            }
        }

        Chunk { voxels, is_empty: !contains_solid_voxels }
    }

    // pub fn sphere() -> Chunk {
    //     let mut voxels = NON_SOLID_VOXELS;
    //     for n in 0..(32 * 32 * 32) {
    //         let x = n % 32;
    //         let y = (n / 32) % 32;
    //         let z = n / (32 * 32);
    //
    //         let sx = x as f32 - 16.0;
    //         let sy = y as f32 - 16.0;
    //         let sz = z as f32 - 16.0;
    //         let s = sz * sz + sy * sy + sx * sx;
    //         voxels[z][y][x].solid = (s < (15.0 * 15.0) && s > (5.0 * 5.0)) || (s < 2.0);
    //     }
    //
    //     Chunk { voxels }
    // }

    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> &Voxel {
        return &self.voxels[z][y][x];
    }

    pub fn in_bounds(vx: &(i32, i32, i32)) -> bool {
        vx.2 >= 0 && vx.2 <= 32 && vx.1 >= 0 && vx.1 <= 32 && vx.0 >= 0 && vx.0 <= 32
    }
}





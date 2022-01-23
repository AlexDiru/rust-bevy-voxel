use bevy::math::{IVec3, Vec3};
use opensimplex_noise_rs::OpenSimplexNoise;
use crate::chunk_utils::{voxel_index_to_xyz, xyz_to_voxel_index};
use crate::Transform;
use crate::voxel::Voxel;

pub struct Chunk {
    pub location: IVec3,
    pub size: IVec3,
    pub voxels: Vec<Voxel>,
    pub noise_generator: OpenSimplexNoise,
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

impl Chunk {
    pub fn get_transform(&self) -> Transform {
        Transform::from_translation(Vec3::new(
            (self.size.x * self.location.x) as f32,
            (self.size.y * self.location.y) as f32,
            (self.size.z * self.location.z) as f32
        ))
    }

    pub fn noise(size: IVec3, location: IVec3) -> Chunk {
        let mut voxels = Vec::new();

        let noise_generator = OpenSimplexNoise::new(Some(883_279_212_983_182_319)); // if not provided, default seed is equal to 0
        let offset = location * size;
        let voxel_length = (size.x * size.y * size.z) as usize;

        for n in 0..voxel_length {
            let xyz = voxel_index_to_xyz(n as i32, &size);
            let xyz_offset = offset + xyz;
            voxels.push(generate_voxel_at_xyz(&noise_generator, &xyz_offset));
        }

        println!("Voxel Count is {}", voxel_length);

        Chunk {
            voxels,
            size,
            location,
            noise_generator,
        }
    }

    // Capable of generating voxels for different chunks, i.e. local_xyz = { -1, -1, -1 } is possible
    pub fn generate_voxel_in_localspace(&self, local_xyz: &IVec3) -> Voxel {
        let global_xyz = (self.location * self.size) + *local_xyz;
        generate_voxel_at_xyz(&self.noise_generator, &global_xyz)
    }

    pub fn get_voxel(&self, xyz: &IVec3) -> &Voxel {
        return &self.voxels[xyz_to_voxel_index(xyz.x as usize, xyz.y as usize, xyz.z as usize)];
    }
}

// Generates the voxel at xyz, needs to also be able to generate voxels for neighbouring chunks
fn generate_voxel_at_xyz(noise_generator: &OpenSimplexNoise, global_xyz: &IVec3) -> Voxel {
    // global xyz means that voxel xyz is from 0..inf
    // i.e. Chunk 0,0 xyz = 0..32
    // Chunk 1,1 xyz = 32..64
    // Chunk 10,10 xyz = 320..352
    // let chunk_location_x = global_xyz.x % self.size.x;
    // let chunk_location_z = global_xyz.z % self.size.z;
    // let chunk_location = IVec3::new(chunk_location_x, 0, chunk_location_z);
    // let local_xyz =
    let scale = 0.1;

    let mut val = noise_generator.eval_3d(
        (global_xyz.x) as f64 * scale,
        (global_xyz.y) as f64 * scale,
        (global_xyz.z) as f64 * scale);

    // Normalise val from -1 to 1, to 0 to 1
    val = (val + 1.0) / 2.0;

    let chance = 0.5;//(flat_chunk().calculate_solid_probability)(x as f32, y as f32, (z as i32 + z_offset) as f32);

    // The chance of the voxel being solid, increases the lower y is
    //let chance = (z as f64 / 16.0);

   //let solid = val as f32 <= chance;

    let mut solid = ((global_xyz.x) % 4) != 0;

    if global_xyz.y == 31 {
       solid = true;
    }

    Voxel {
        solid,
    }
}





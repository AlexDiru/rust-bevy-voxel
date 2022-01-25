use std::hash::Hash;
use std::ops::Div;
use bevy::math::{IVec3, Vec3};
use opensimplex_noise_rs::OpenSimplexNoise;
use crate::chunk::Biome::{FLAT, PERLIN_MOUNTAINS, QUARRY};
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
            voxels.push(generate_voxel_at_xyz(&noise_generator, &xyz_offset, &size));
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
        generate_voxel_at_xyz(&self.noise_generator, &global_xyz, &self.size)
    }

    pub fn get_voxel(&self, xyz: &IVec3) -> &Voxel {
        return &self.voxels[xyz_to_voxel_index(&xyz, &self.size)];
    }
}

enum Biome {
    PERLIN_MOUNTAINS,
    FLAT,
    QUARRY,
}

fn get_biome(noise_generator: &OpenSimplexNoise, global_xyz: &IVec3, chunk_size: &IVec3) -> Biome {
    // let chunk_x = (global_xyz.x as f32/ chunk_size.x as f32).floor() as f64;
    // let chunk_z = (global_xyz.z as f32 / chunk_size.z as f32).floor() as f64;

    let noise = noise_generator.eval_2d(global_xyz.x as f64 / 100.0, global_xyz.z as f64 / 100.0);
    // Normalise val from -1 to 1, to 0 to 1
    let normalised_noise = (noise + 1.0) / 2.0;

    if normalised_noise < 0.3333 {
        return PERLIN_MOUNTAINS
    } else if normalised_noise < 0.6666 {
        return FLAT
    }

    return QUARRY
}

fn perlin_mountains(noise_generator: &OpenSimplexNoise, global_xyz: &IVec3, scale: f64) -> bool {

    let mut val = noise_generator.eval_3d(
        (global_xyz.x) as f64 * scale,
        (global_xyz.y) as f64 * scale,
        (global_xyz.z) as f64 * scale);

    // Normalise val from -1 to 1, to 0 to 1
    val = (val + 1.0) / 2.0;

    // The chance of the voxel being solid, increases the lower y is
    let chance = ((global_xyz.y) as f64).log10() / (64.0_f64).log10();

    let solid = val as f64 > chance;
    solid
}

fn solid_mountains(noise_generator: &OpenSimplexNoise, global_xyz: &IVec3, chunk_size: &IVec3) -> bool {
    for y in (global_xyz.y..chunk_size.y).rev() {
        if perlin_mountains(noise_generator, &IVec3::new(global_xyz.x, y, global_xyz.z), 0.05) {
            return true
        }
    }
    return false
}


fn mc(noise_generator: &OpenSimplexNoise, global_xyz: &IVec3, min_height: i32, max_height: i32, scale: f64) -> bool {
    if global_xyz.y <= min_height {
        return true
    } else if global_xyz.y >= max_height {
        return false
    }

    let noise = noise_generator.eval_2d(
        (global_xyz.x) as f64 * scale,
        (global_xyz.z) as f64 * scale);

    // Normalise val from -1 to 1, to 0 to 1
    let normalised_noise = (noise + 1.0) / 2.0;

    let offset = normalised_noise * (max_height - min_height) as f64;

    let solid = global_xyz.y < offset as i32;
    solid
}

fn flat(noise_generator: &OpenSimplexNoise, global_xyz: &IVec3, min_height: i32, max_height: i32, scale: f64) -> bool {
    if global_xyz.y <= min_height {
        return true
    } else if global_xyz.y >= max_height {
        return false
    }

    let mut val = noise_generator.eval_3d(
        (global_xyz.x) as f64 * scale,
        (global_xyz.y) as f64 * scale,
        (global_xyz.z) as f64 * scale);

    // Normalise val from -1 to 1, to 0 to 1
    val = (val + 1.0) / 2.0;

    // The chance of the voxel being solid, increases the lower y is
    let height = (global_xyz.y as f32 - min_height as f32) / (max_height - min_height) as f32; // 0 - 1

    let solid = val as f64 > height as f64;
    solid
}

// Generates the voxel at xyz, needs to also be able to generate voxels for neighbouring chunks
fn generate_voxel_at_xyz(noise_generator: &OpenSimplexNoise, global_xyz: &IVec3, chunk_size: &IVec3) -> Voxel {
    // global xyz means that voxel xyz is from 0..inf
    // i.e. Chunk 0,0 xyz = 0..32
    // Chunk 1,1 xyz = 32..64
    // Chunk 10,10 xyz = 320..352

    let biome = get_biome(noise_generator, global_xyz, chunk_size);

    match biome {
        PERLIN_MOUNTAINS => {
            Voxel { solid: perlin_mountains(noise_generator, global_xyz, 0.1) }
        },
        FLAT => {
            Voxel {
                solid: flat(noise_generator, global_xyz, 8, 24, 0.07),
            }
        },
        QUARRY => {
            Voxel { solid: mc(noise_generator, global_xyz, 7, 64, 0.03) }
        }
    }
}


#[cfg(test)]
mod tests {
    use opensimplex_noise_rs::OpenSimplexNoise;
    use crate::chunk::generate_voxel_at_xyz;
    use crate::chunk_utils::voxel_index_to_xyz;
    use crate::IVec3;

    #[test]
    fn generate_voxel_at_xyz_test() {
        let noise_generator = OpenSimplexNoise::new(Some(883_279_212_983_182_319));
        let actual = generate_voxel_at_xyz(&noise_generator, &IVec3::new(0, 0, 0));
        assert_eq!(generate_voxel_at_xyz(&noise_generator, &IVec3::new(0, 0, 0)), actual);

        let noise_generator2 = OpenSimplexNoise::new(Some(883_279_212_983_182_319));
        assert_eq!(generate_voxel_at_xyz(&noise_generator2, &IVec3::new(0, 0, 0)), actual);

        for x in -3..67 {
            for y in -3..67 {
                for z in -3..67 {
                    assert_eq!(
                        generate_voxel_at_xyz(&noise_generator, &IVec3::new(x, y, z)),
                        generate_voxel_at_xyz(&noise_generator2, &IVec3::new(x, y, z)));
                }
            }
        }
    }
}


